use nanomsg::{Socket as NanoSocket, Protocol};
use nanomsg::endpoint::Endpoint;
use nanomsg::result::Error as NanoError;

use futures::{Stream, Async, Poll};

use std::fmt::{Debug, Formatter, Result as FmtResult};

use tokio_core::reactor::Handle;

use evented::{self, NanoEvented};


static MAX_SOCKET_NAME_LENGTH: usize = 128;

pub struct Socket {
    socket: NanoSocket,
    endpoint: Option<Endpoint>,
    name: String,
    // poll stuff
    recv_evented: Option<NanoEvented>,
    send_evented: Option<NanoEvented>,
}

impl Socket {
    pub fn new(protocol: Protocol, handle: &Handle) -> Result<Socket, NanoError> {
        let mut socket = NanoSocket::new(protocol)?;

        let recv_fd = socket.get_receive_fd().ok();
        let send_fd = socket.get_send_fd().ok();

        let recv_evented = recv_fd.and_then(|fd| NanoEvented::try_new(fd, handle).ok());
        let send_evented = send_fd.and_then(|fd| NanoEvented::try_new(fd, handle).ok());

        // at least one listening socket should be available
        if recv_evented.is_none() && send_evented.is_none() {
            // to be fixed
            return Err(NanoError::Unknown);
        }

        let name = socket
            .get_socket_name(MAX_SOCKET_NAME_LENGTH)
            .unwrap_or_else(|_| "Unable to get socket name".to_string());


        Ok(Socket {
               socket: socket,
               endpoint: None,
               name: name,
               recv_evented: recv_evented,
               send_evented: send_evented,
           })
    }

    pub fn bind(&mut self, addr: &str) -> Result<(), NanoError> {
        if self.endpoint.is_some() {
            // to be fixed
            Err(NanoError::Unknown)
        } else {
            self.endpoint = Some(self.socket.bind(addr)?);

            Ok(())
        }
    }

    pub fn connect(&mut self, addr: &str) -> Result<(), NanoError> {
        if self.endpoint.is_some() {
            // to be fixed
            Err(NanoError::Unknown)
        } else {
            self.endpoint = Some(self.socket.connect(addr)?);

            Ok(())
        }
    }

    fn fetch(&mut self) -> Result<Option<Vec<u8>>, NanoError> {
        let mut buf = Vec::new();

        match self.socket.nb_read_to_end(&mut buf) {
            Ok(size) => {
                trace!("Read ok, got {} bytes", size);

                buf.truncate(size);

                Ok(Some(buf))
            }
            Err(NanoError::TryAgain) => {
                trace!("Async::NotReady while reading from socket");
                self.recv_evented
                    .as_mut()
                    .expect("recv_evented is None")
                    .schedule();

                Ok(None)
            }
            Err(err) => Err(err),
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        if let Some(ref mut endpoint) = self.endpoint {
            endpoint
                .shutdown()
                .expect("socket endpoint shutdown failed");
        }
    }
}

impl Debug for Socket {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Socket")
            .field("socket", &self.name)
            .field("endpoint", &self.endpoint.is_some())
            .field("recv_evented", &self.recv_evented)
            .field("send_evented", &self.send_evented)
            .finish()
    }
}

impl Stream for Socket {
    type Item = Vec<u8>;
    type Error = NanoError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        trace!("Trying to poll the socket");

        if self.endpoint.is_none() {
            error!("Endpoint is empty");
            return Err(NanoError::Unknown);
        }

        match self.recv_evented.as_mut().unwrap().poll() {
            evented::Async::Ready => {
                match self.fetch() {
                    Ok(Some(buf)) => Ok(Async::Ready(Some(buf))),
                    Ok(None) => Ok(Async::NotReady),
                    Err(err) => Err(err),
                }
            }
            evented::Async::NotReady => Ok(Async::NotReady),
        }
    }
}
