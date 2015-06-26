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

#[macro_use]
extern crate log;

use std::fmt;
use std::net::SocketAddr;
use std::io::{self, Read, Write, Cursor};
use std::sync::Mutex;
use std::collections::HashMap;

use hyper::net::{NetworkStream, NetworkConnector};

/// A `NetworkStream` compatible stream that writes into memory, and reads from memory.
pub struct MockStream {
    pub read: Cursor<Vec<u8>>,
    pub write: Vec<u8>,
}

/// A `NetworkStream` compatible stream which contains another `NetworkStream`, 
/// whose traffic will be written to another stream.
/// Currently that stream will always be standard error.
pub struct TeeStream<T> {
    pub read_write: T,
    pub copy_to: io::Stderr,
}

impl<T> Clone for TeeStream<T>
    where T: Clone {
    fn clone(&self) -> TeeStream<T> {
        TeeStream {
            read_write: self.read_write.clone(),
            copy_to: io::stderr(),
        }
    }
}

impl<T> Read for TeeStream<T>
    where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.read_write.read(buf);
        match res {
            Ok(s) => {
                self.copy_to.write(&buf[..s]).ok();
            }
            _ => {}
        };
        res
    }
}

impl<T> Write for TeeStream<T>
    where T: Write {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        self.copy_to.write(msg).ok();
        self.read_write.write(msg)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.copy_to.flush().ok();
        self.read_write.flush()
    }
}

impl<T> NetworkStream for TeeStream<T>
    where T: NetworkStream + Send + Clone {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        self.read_write.peer_addr()
    }
}
impl Clone for MockStream {
    fn clone(&self) -> MockStream {
        MockStream {
            read: Cursor::new(self.read.get_ref().clone()),
            write: self.write.clone()
        }
    }
}

impl fmt::Debug for MockStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MockStream {{ read: {:?}, write: {:?} }}", self.read.get_ref(), self.write)
    }
}

impl PartialEq for MockStream {
    fn eq(&self, other: &MockStream) -> bool {
        self.read.get_ref() == other.read.get_ref() && self.write == other.write
    }
}

impl MockStream {
    pub fn new() -> MockStream {
        MockStream {
            read: Cursor::new(vec![]),
            write: vec![],
        }
    }

    pub fn with_input(input: &[u8]) -> MockStream {
        MockStream {
            read: Cursor::new(input.to_vec()),
            write: vec![]
        }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read.read(buf)
    }
}

impl Write for MockStream {
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        Write::write(&mut self.write, msg)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl NetworkStream for MockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }
}

/// A `NetworkConnector` which creates `MockStream` instances exclusively.
/// It may be useful to intercept writes.
pub struct MockConnector;

impl NetworkConnector for MockConnector {
    type Stream = MockStream;

    fn connect(&self, _host: &str, _port: u16, _scheme: &str) -> hyper::Result<MockStream> {
        Ok(MockStream::new())
    }
}

/// A `NetworkConnector` embedding another `NetworkConnector` instance, 
/// and sets it up to write all reads and writes to standard error as well.
///
/// > **NOTE** It was originally intended to allow arbitrary streams to copy data to,
/// > but I couldn't get passt the compiler with that as normal streams, like files,
/// > are not normally clonable. Maybe an Arc+Mutex would have helped ... .
pub struct TeeConnector<C>
    where C: NetworkConnector {
    pub connector: C,
}

impl<C, S> NetworkConnector for TeeConnector<C> 
    where C: NetworkConnector<Stream=S>,
          S: NetworkStream + Send + Clone {
    type Stream = TeeStream<<C as NetworkConnector>::Stream>;

    fn connect(&self, _host: &str, _port: u16, _scheme: &str)
        -> hyper::Result<TeeStream<<C as NetworkConnector>::Stream>> {
        match self.connector.connect(_host, _port, _scheme) {
            Ok(s) => {
                Ok(TeeStream {
                        read_write: s,
                        copy_to: io::stderr(),
                    }
                )
            },
            Err(err) => Err(err),
        }
    }
}

/// This macro maps host URLs to a respective reply, which is given in plain-text.
/// It ignores, but stores, everything that is written to it. However, the stored
/// values are not accessible just yet.
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

        impl hyper::net::NetworkConnector for $name {
            type Stream = $crate::MockStream;

            fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<$crate::MockStream> {
                self.0.connect(host, port, scheme)
            }
        }
    )
);

/// A `NetworkConnector` which provides a single reply stream per host.
///
/// The mapping is done from full host url (e.g. `http://host.name.com`) to the 
/// singular reply the host is supposed to make.
#[derive(Default)]
pub struct HostToReplyConnector {
    pub m: HashMap<String, String>
}

impl hyper::net::NetworkConnector for HostToReplyConnector {
    type Stream = MockStream;

    fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<MockStream> {
        debug!("HostToReplyConnector::connect({:?}, {:?}, {:?})", host, port, scheme);
        let key = format!("{}://{}", scheme, host);
        // ignore port for now
        match self.m.get(&key) {
            Some(res) => Ok(MockStream {
                write: vec![],
                read: Cursor::new(res.clone().into_bytes()),
            }),
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

        impl hyper::net::NetworkConnector for $name {
            type Stream = $crate::MockStream;
            fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<$crate::MockStream> {
                self.0.connect(host, port, scheme)
            }
        }
    )
);


/// A connector which requires you to implement the `Default` trait, allowing you
/// to determine the data it should be initialized with
pub struct SequentialConnector {
    pub content: Vec<String>,
    current: Mutex<usize>,
}

impl Default for SequentialConnector {
    fn default() -> Self {
        SequentialConnector {
            content: Vec::new(),
            current: Mutex::new(0)
        }
    }
}

impl hyper::net::NetworkConnector for SequentialConnector {
    type Stream = MockStream;

    fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<MockStream> {
        use std::io::Cursor;
        debug!("SequentialConnector::connect({:?}, {:?}, {:?})", host, port, scheme);

        assert!(self.content.len() != 0, "Not a single streamer return value specified");

        let r = Ok(MockStream {
                write: vec![],
                read: Cursor::new(self.content[*self.current.lock().unwrap()].clone().into_bytes())
        });
        *self.current.lock().unwrap() += 1;
        r
    }
}



