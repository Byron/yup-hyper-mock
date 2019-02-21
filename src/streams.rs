use std::str;
use std::io::{self, Read, Write};

use futures;
use futures::task;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_reactor::PollEvented;
use mio::{event::Evented, PollOpt, Poll, Ready, Token, SetReadiness, Registration};

struct MockHttpStream {
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
            trace!("Not ready for read yet");
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
        trace!("Read {} bytes: '{}'", n, str::from_utf8(&buf[..n]).unwrap_or("<bad utf-8>"));
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
        trace!("Request data: {}", str::from_utf8(data).unwrap_or("<bad utf-8>"));
        self.ready_for_response = true;
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsyncWrite for MockHttpStream {
    fn shutdown(&mut self) -> futures::Poll<(), io::Error> {
        Ok(().into())
    }
}

impl Write for MockPollStream {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let ret = self.stream.write(data);
        match ret {
            Ok(r) => {
                self.set_readiness.set_readiness(Ready::readable()).unwrap();
                Ok(r)
            }

            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                self.stream.clear_write_ready()?;
                Err(io::Error::from(io::ErrorKind::WouldBlock))
            }

            Err(e) => Err(e),
        }
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