use futures;
use log::*;

use std::io;
use std::os::unix::io::RawFd;

use tokio_core::reactor::{Handle, PollEvented};

use crate::socket_fd::SocketFd;

#[derive(Debug)]
pub enum Async {
    /// socket is ready
    Ready,
    /// socket is not ready
    NotReady,
}

#[derive(Debug)]
pub struct NanoEvented {
    io: PollEvented<SocketFd>,
    should_poll: bool,
}

impl NanoEvented {
    pub fn try_new(sockfd: RawFd, handle: &Handle) -> io::Result<NanoEvented> {
        Ok(NanoEvented {
            io: PollEvented::new(SocketFd(sockfd), handle)?,
            should_poll: true,
        })
    }

    pub fn poll(&mut self) -> Async {
        if self.should_poll {
            match self.io.poll_read() {
                futures::Async::NotReady => {
                    trace!("Async::NotReady");
                    self.io.need_read();

                    Async::NotReady
                }
                futures::Async::Ready(_) => {
                    trace!("Async::Ready");
                    self.should_poll = false;

                    Async::Ready
                }
            }
        } else {
            Async::Ready
        }
    }

    pub fn schedule(&mut self) {
        self.should_poll = true;
        self.io.need_read();
    }
}
