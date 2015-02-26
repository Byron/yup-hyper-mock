//! Contains various utility types and macros useful for testing hyper clients. 
//! 
//! # Macros
//! The `mock_connector!` and `mock_connector_in_order!` macros can be used to 
//! feed a client with preset data. That way you can control exactly what it will
//! see, confining the test-case to its own sandbox that way.
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


#![feature(old_io,io)]
extern crate hyper;

#[macro_use]
extern crate log;

use std::fmt;
use std::old_io::{IoResult, MemReader, MemWriter, stderr, LineBufferedWriter};
use std::old_io::stdio::StdWriter;
use std::old_io::net::ip::SocketAddr;

use hyper::net::{NetworkStream, NetworkConnector};

/// A `NetworkStream` compatible stream that writes into memory, and reads from memory.
pub struct MockStream {
    pub read: MemReader,
    pub write: MemWriter,
}

/// A `NetworkStream` compatible stream which contains another `NetworkStream`, 
/// whose traffic will be written to another stream.
/// Currently that stream will always be standard error.
pub struct TeeStream<T> {
    pub read_write: T,
    pub copy_to: LineBufferedWriter<StdWriter>,
}

impl<T> Clone for TeeStream<T>
    where T: Clone {
    fn clone(&self) -> TeeStream<T> {
        TeeStream {
            read_write: self.read_write.clone(),
            copy_to: stderr(),
        }
    }
}

impl<T> Reader for TeeStream<T>
    where T: Reader {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let res = self.read_write.read(buf);
        match res {
            Ok(s) => {
                self.copy_to.write_all(&buf[..s]).ok();
            }
            _ => {}
        };
        res
    }
}

impl<T> Writer for TeeStream<T>
    where T: Writer {
    fn write_all(&mut self, msg: &[u8]) -> IoResult<()> {
        self.copy_to.write_all(msg).ok();
        self.read_write.write_all(msg)
    }
}

impl<T> NetworkStream for TeeStream<T>
    where T: NetworkStream + Send + Clone {
    fn peer_name(&mut self) -> IoResult<SocketAddr> {
        self.read_write.peer_name()
    }
}

impl Clone for MockStream {
    fn clone(&self) -> MockStream {
        MockStream {
            read: MemReader::new(self.read.get_ref().to_vec()),
            write: MemWriter::from_vec(self.write.get_ref().to_vec()),
        }
    }
}

impl PartialEq for MockStream {
    fn eq(&self, other: &MockStream) -> bool {
        self.read.get_ref() == other.read.get_ref() &&
            self.write.get_ref() == other.write.get_ref()
    }
}

impl fmt::Debug for MockStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MockStream {{ read: {:?}, write: {:?} }}",
               self.read.get_ref(), self.write.get_ref())
    }

}

impl MockStream {
    pub fn new() -> MockStream {
        MockStream {
            read: MemReader::new(vec![]),
            write: MemWriter::new(),
        }
    }

    pub fn with_input(input: &[u8]) -> MockStream {
        MockStream {
            read: MemReader::new(input.to_vec()),
            write: MemWriter::new(),
        }
    }
}
impl Reader for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.read.read(buf)
    }
}

impl Writer for MockStream {
    fn write_all(&mut self, msg: &[u8]) -> IoResult<()> {
        self.write.write_all(msg)
    }
}

impl NetworkStream for MockStream {
    fn peer_name(&mut self) -> IoResult<SocketAddr> {
        Ok("127.0.0.1:1337".parse().unwrap())
    }
}

/// A `NetworkConnector` which creates `MockStream` instances exclusively.
/// It may be useful to intercept writes.
pub struct MockConnector;

impl NetworkConnector for MockConnector {
    type Stream = MockStream;

    fn connect(&mut self, _host: &str, _port: u16, _scheme: &str) -> IoResult<MockStream> {
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

impl<C> NetworkConnector for TeeConnector<C> 
    where C: NetworkConnector,
          <C as NetworkConnector>::Stream: Clone {
    type Stream = TeeStream<<C as NetworkConnector>::Stream>;

    fn connect(&mut self, _host: &str, _port: u16, _scheme: &str)
        -> IoResult<TeeStream<<C as NetworkConnector>::Stream>> {
        match self.connector.connect(_host, _port, _scheme) {
            Ok(s) => {
                Ok(TeeStream {
                        read_write: s,
                        copy_to: stderr(),
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

        struct $name;

        impl hyper::net::NetworkConnector for $name {
            type Stream = $crate::MockStream;
            fn connect(&mut self, host: &str, port: u16, scheme: &str) -> ::std::old_io::IoResult<$crate::MockStream> {
                use std::collections::HashMap;
                debug!("MockStream::connect({:?}, {:?}, {:?})", host, port, scheme);
                let mut map = HashMap::new();
                $(map.insert($url, $res);)*


                let key = format!("{}://{}", scheme, host);
                // ignore port for now
                match map.get(&*key) {
                    Some(res) => Ok($crate::MockStream {
                        write: ::std::old_io::MemWriter::new(),
                        read: ::std::old_io::MemReader::new(res.to_string().into_bytes())
                    }),
                    None => panic!("{:?} doesn't know url {}", stringify!($name), key)
                }
            }

        }

    )
);

/// This macro yields all given server replies in the order they are given.
/// The destination host URL doesn't matter at all.
#[macro_export]
macro_rules! mock_connector_in_order (
    ($name:ident {
        $( $res:expr )*
    }) => (

        #[derive(Default)]
        struct $name {
            streamers: Vec<String>,
            current: usize,
        }

        impl hyper::net::NetworkConnector for $name {
            type Stream = $crate::MockStream;
            fn connect(&mut self, host: &str, port: u16, scheme: &str) -> ::std::old_io::IoResult<$crate::MockStream> {
                debug!("MockStream::connect({:?}, {:?}, {:?})", host, port, scheme);

                if self.streamers.len() == 0 {
                    let mut v = Vec::new();
                    $(v.push($res.to_string());)*
                    self.streamers = v;
                    self.current = 0;
                }
                assert!(self.streamers.len() != 0, "Not a single streamer return value specified");

                let r = Ok($crate::MockStream {
                        write: ::std::old_io::MemWriter::new(),
                        read: ::std::old_io::MemReader::new(self.streamers[self.current]
                                                            .clone().into_bytes())
                });
                self.current += 1;
                r
            }
        }
    )
);

