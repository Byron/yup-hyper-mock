use std::io;
use std::str;
use std::{
    cmp::min,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use hyper::rt::ReadBufCursor;
use hyper_util::client::legacy::connect::{Connected, Connection};
use log::trace;

pub struct MockPollStream {
    data: Vec<u8>,
    pos: usize,
    ready_for_response: bool,
    waker: Option<Waker>,
}

impl MockPollStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: 0,
            ready_for_response: false,
            waker: None,
        }
    }
}

impl hyper::rt::Read for MockPollStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        if !self.ready_for_response {
            trace!("Not ready for read yet");
            self.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }
        let mut read_buf = unsafe { tokio::io::ReadBuf::uninit(buf.as_mut()) };
        trace!(
            "Buffer size: {}, Data size: {}, Pos: {}",
            read_buf.remaining(),
            self.data.len(),
            self.pos
        );
        let n = min(read_buf.remaining(), self.data.len() - self.pos);
        let read_until = self.pos + n;
        read_buf.put_slice(&self.data[self.pos..read_until]);
        self.pos = read_until;
        unsafe { buf.advance(n) };
        trace!(
            "Read {} bytes: '{}'",
            n,
            str::from_utf8(&self.data[self.pos..read_until]).unwrap_or("<bad utf-8>")
        );
        self.waker = Some(cx.waker().clone());
        Poll::Ready(Ok(()))
    }
}

impl hyper::rt::Write for MockPollStream {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        data: &[u8],
    ) -> Poll<io::Result<usize>> {
        trace!(
            "Request data: {}",
            str::from_utf8(data).unwrap_or("<bad utf-8>")
        );
        let Self {
            ready_for_response,
            waker,
            ..
        } = self.get_mut();
        *ready_for_response = true;
        waker.take().map(|w| w.wake());
        Poll::Ready(Ok(data.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl Connection for MockPollStream {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}
