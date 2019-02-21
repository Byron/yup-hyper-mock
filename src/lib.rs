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
extern crate tokio_reactor;
extern crate mio;

#[macro_use]
extern crate log;

use std::sync::Mutex;
use std::collections::HashMap;
use futures::Future;
use hyper::client::connect;

mod streams;
pub use crate::streams::MockPollStream;

/// This macro maps host URLs to a respective reply, which is given in plain-text.
/// It ignores everything that is written to it.
#[macro_export]
macro_rules! mock_connector (
    ($name:ident {
        $($url:expr => $res:expr)*
    }) => (

        #[derive(Clone)]
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

/// A `Connect` which provides a single reply stream per host.
///
/// The mapping is done from full host url (e.g. `http://host.name.com`) to the
/// singular reply the host is supposed to make.
#[derive(Default, Clone)]
pub struct HostToReplyConnector {
    pub m: HashMap<String, String>
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