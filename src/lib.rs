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
#[macro_use]
extern crate tokio_core;
extern crate bytes;

#[macro_use]
extern crate log;

//use std::fmt;
//use std::net::SocketAddr;
//use std::io::{self, Read, Write, Cursor};
use std::sync::Mutex;
use std::collections::HashMap;
//use tokio_core::net::TcpStream;
//use std::time::Duration;

mod hyper_mocks;

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

        impl hyper::client::Service for $name {
            type Request = hyper::Uri;
            type Response = tokio_core::net::TcpStream;
            type Error = std::io::Error;
            type Future = $crate::HttpConnecting;

            fn call(&self, _uri: Self::Request) -> Self::Future {
                unimplemented!();
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

#[must_use = "futures do nothing unless polled"]
pub struct HttpConnecting {
}

impl futures::Future for HttpConnecting {
    type Item = tokio_core::net::TcpStream;
    type Error = std::io::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        unimplemented!();
    }
}

impl hyper::client::Service for HostToReplyConnector {
    type Request = hyper::Uri;
    type Response = tokio_core::net::TcpStream;
    type Error = std::io::Error;
    type Future = HttpConnecting;

    fn call(&self, uri: Self::Request) -> Self::Future {
        unimplemented!();
    }

    // fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<MockStream> {
    //     debug!("HostToReplyConnector::connect({:?}, {:?}, {:?})", host, port, scheme);
    //     let key = format!("{}://{}", scheme, host);
    //     // ignore port for now
    //     match self.m.get(&key) {
    //         Some(res) => Ok(MockStream {
    //             write: vec![],
    //             read: Cursor::new(res.clone().into_bytes()),
    //         }),
    //         None => panic!("HostToReplyConnector doesn't know url {}", key)
    //     }
    // }
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

        impl hyper::client::Service for $name {
            type Request = hyper::Uri;
            type Response = tokio_core::net::TcpStream;
            type Error = std::io::Error;
            type Future = $crate::HttpConnecting;

            fn call(&self, uri: Self::Request) -> Self::Future {
                unimplemented!();
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

impl hyper::client::Service for SequentialConnector {
    type Request = hyper::Uri;
    type Response = tokio_core::net::TcpStream;
    type Error = std::io::Error;
    type Future = HttpConnecting;

    fn call(&self, uri: Self::Request) -> Self::Future {
        unimplemented!();
    }

    // fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<MockStream> {
    //     use std::io::Cursor;
    //     debug!("SequentialConnector::connect({:?}, {:?}, {:?})", host, port, scheme);

    //     assert!(self.content.len() != 0, "Not a single streamer return value specified");

    //     let r = Ok(MockStream {
    //             write: vec![],
    //             read: Cursor::new(self.content[*self.current.lock().unwrap()].clone().into_bytes())
    //     });
    //     *self.current.lock().unwrap() += 1;
    //     r
    // }
}



