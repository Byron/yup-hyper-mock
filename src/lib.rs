//! Contains various utility types and macros useful for testing hyper clients. 
//! 
//! # Macros
//! The `mock_connector!` and `mock_connector_in_order!` macros can be used to 
//! feed a client with preset data. That way you can control exactly what it will
//! see, confining the test-case to its own sandbox that way.
//!
//! All types they define are public to allow them to be used in other unit-tests.
//! Please note that integration tests cannot share their mock types anyway, as each
//! integration test goes into its own binary.
//!
//! # Usage
//! 
//! Set it up for use in tests in `Cargo.toml`
//!
//! ```toml
//! [dev-dependencies]
//! yup-hyper-mock = "*"
//! log = "*"  # log macros are used within yup-hyper-mock
//! ```
//! 
//! Link it into your `src/(lib.rs|main.rs)`
//! 
//! ```Rust
//! #[cfg(test)] #[macro_use]
//! extern crate "yup-hyper-mock" as hyper_mock
//! ```


extern crate hyper;
extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_reactor;
extern crate mio;

#[macro_use]
extern crate log;

use std::io::{self, Read, Write};
use std::sync::Mutex;
use std::collections::HashMap;
use futures::Future;
use tokio_io::{AsyncRead, AsyncWrite};
use futures::task;
use tokio_reactor::PollEvented;
use mio::event::Evented;
use mio::PollOpt;
use mio::Poll;
use mio::Ready;
use mio::Token;
use mio::SetReadiness;
use mio::Registration;
use hyper::client::connect;

/// This macro maps host URLs to a respective reply, which is given in plain-text.
/// It ignores everything that is written to it.
#[macro_export]
macro_rules! mock_connector (
    ($name:ident {
        $($url:expr => $res:expr)*
    }) => (

        pub struct $name($crate::HostToReplyConnector);

        impl Default for $name {
            fn default() -> $name {
                let mut c = $name(Default::default());
                $(c.0.m.insert($url.to_string(), $res.to_string());)*
                c
            }
        }

        impl hyper::client::connect::Connect for $name {
            type Transport = $crate::MockPollStream;
            type Error = std::io::Error;
            type Future = Box<Future<Item=(Self::Transport, hyper::client::connect::Connected), Error=Self::Error> + Send>;

            fn connect(&self, dst: hyper::client::connect::Destination) -> Self::Future {
                let key = format!("{}://{}", dst.scheme(), dst.host());
                // ignore port for now
                match self.0.m.get(&key) {
                    Some(res) => Box::new(futures::future::ok(($crate::MockPollStream::new(res.clone().into_bytes()), hyper::client::connect::Connected::new()))),
                    None => panic!("mock_connector doesn't know url {}", key)
                }
            }
        }
    )
);

/// A `NetworkConnector` which provides a single reply stream per host.
///
/// The mapping is done from full host url (e.g. `http://host.name.com`) to the 
/// singular reply the host is supposed to make.
#[derive(Default, Clone)]
pub struct HostToReplyConnector {
    pub m: HashMap<String, String>
}

pub struct MockHttpStream {
    data: Vec<u8>,
    pos: usize,
    ready_for_response: bool,
    task: Option<task::Task>,
    registration: Registration,
}

pub struct MockPollStream {
    stream: PollEvented<MockHttpStream>,
    set_readiness: SetReadiness
}

impl MockPollStream {
    pub fn new(data: Vec<u8>) -> MockPollStream {
        let (registration, set_readiness) = Registration::new2();
        let m = MockHttpStream {
            data: data,
            pos: 0,
            ready_for_response: false,
            task: None,
            registration: registration
        };
        set_readiness.set_readiness(Ready::writable()).unwrap();
        MockPollStream {
            stream: PollEvented::new(m),
            set_readiness: set_readiness
        }
    }
}

impl Evented for MockHttpStream {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        trace!("Token: {:?}", token);
        Ok(self.registration.register(poll, token, interest, opts)?)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        Ok(self.registration.reregister(poll, token, interest, opts)?)
    }

    fn deregister(&self, _poll: &Poll) -> io::Result<()> {
        Ok(())
    }
}

impl Read for MockHttpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.ready_for_response {
            debug!("Not ready for read yet");
            self.task = Some(task::current());
            return Err(io::ErrorKind::WouldBlock.into());
        }
        trace!("Buffer size: {}, Data size: {}, Pos: {}", buf.len(), self.data.len(), self.pos);
        let n = if buf.len() < (self.data.len()-self.pos) { buf.len() } else { self.data.len()-self.pos };
        if n > 0 {
            for i in 0..n {
                buf[i] = self.data[self.pos];
                self.pos+=1;
            }
        }
        trace!("Read {} bytes: '{}'", n, std::str::from_utf8(&buf[..n]).unwrap_or("<bad utf-8>"));
        self.task = Some(task::current());
        Ok(n)
    }
}

impl AsyncRead for MockHttpStream {}

impl Read for MockPollStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl AsyncRead for MockPollStream {}

impl Write for MockHttpStream {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        debug!("Request data: {}", std::str::from_utf8(data).unwrap_or("<bad utf-8>"));
        self.ready_for_response = true;
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        debug!("Flush");
        Ok(())
    }
}

impl AsyncWrite for MockHttpStream {
    fn shutdown(&mut self) -> futures::Poll<(), io::Error> {
        debug!("Shutdown");
        Ok(().into())
    }
}

impl Write for MockPollStream {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        trace!("write");
        let ret = self.stream.write(data);
        self.set_readiness.set_readiness(Ready::readable()).unwrap();
        ret
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

impl AsyncWrite for MockPollStream {
    fn shutdown(&mut self) -> futures::Poll<(), io::Error> {
        self.stream.shutdown()
    }
}

impl connect::Connect for HostToReplyConnector {
    type Transport = MockPollStream;
    type Error = std::io::Error;
    type Future = Box<Future<Item=(Self::Transport, connect::Connected), Error=Self::Error> + Send>;

    fn connect(&self, dst: connect::Destination) -> Self::Future {
        debug!("HostToReplyConnector::connect({:?})", dst);
        
        let key = format!("{}://{}", dst.scheme(), dst.host());
        // ignore port for now
        match self.m.get(&key) {
            Some(res) => Box::new(futures::future::ok((MockPollStream::new(res.clone().into_bytes()), connect::Connected::new()))),
            None => panic!("HostToReplyConnector doesn't know url {}", key)
        }
    }
}

/// This macro yields all given server replies in the order they are given.
/// The destination host URL doesn't matter at all.
#[macro_export]
macro_rules! mock_connector_in_order (
    ($name:ident {
        $( $res:expr )*
    }) => (
        pub struct $name($crate::SequentialConnector);

        impl Default for $name {
            fn default() -> $name {
                let mut c = $name(Default::default());
                $(c.0.content.push($res.to_string());)*
                c
            }
        }

        impl hyper::client::connect::Connect for $name {
            type Transport = $crate::MockPollStream;
            type Error = std::io::Error;
            type Future = Box<Future<Item=(Self::Transport, hyper::client::connect::Connected), Error=Self::Error> + Send>;

            fn connect(&self, _dst: hyper::client::connect::Destination) -> Self::Future {
                assert!(self.0.content.len() != 0, "Not a single streamer return value specified");

                let data = self.0.content[*self.0.current.lock().unwrap()].clone().into_bytes();
                *self.0.current.lock().unwrap() += 1;
                Box::new(futures::future::ok(($crate::MockPollStream::new(data), hyper::client::connect::Connected::new())))
            }
        }
    )
);


/// A connector which requires you to implement the `Default` trait, allowing you
/// to determine the data it should be initialized with
pub struct SequentialConnector {
    pub content: Vec<String>,
    pub current: Mutex<usize>,
}

impl Default for SequentialConnector {
    fn default() -> Self {
        SequentialConnector {
            content: Vec::new(),
            current: Mutex::new(0)
        }
    }
}

impl connect::Connect for SequentialConnector {
    type Transport = MockPollStream;
    type Error = std::io::Error;
    type Future = Box<Future<Item=(Self::Transport, connect::Connected), Error=Self::Error> + Send>;

    fn connect(&self, dst: connect::Destination) -> Self::Future {
        debug!("SequentialConnector::connect({:?})", dst);

        assert!(self.content.len() != 0, "Not a single streamer return value specified");

        let data = self.content[*self.current.lock().unwrap()].clone().into_bytes();
        *self.current.lock().unwrap() += 1;
        Box::new(futures::future::ok((MockPollStream::new(data), connect::Connected::new())))
    }
}