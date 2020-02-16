#[macro_use]
extern crate log;
extern crate flexi_logger;
extern crate nanomsg_tokio;
extern crate futures;
extern crate nanomsg;
extern crate tokio_core;
extern crate colored_logger;

use tokio_core::reactor::Core;
use nanomsg_tokio::Socket;
use nanomsg::Protocol;
use futures::Stream;
use colored_logger::FormatterBuilder;


fn main() {
    let formatter = FormatterBuilder::default().build();
    flexi_logger::Logger::with_str("info")
        .format(formatter)
        .start()
        .unwrap();

    let mut core = Core::new().unwrap();

    let mut socket = Socket::new(Protocol::Pull, &core.handle()).unwrap();
    socket.bind("ipc:///tmp/nanomsg-tokio.ipc").unwrap();

    let fut = socket.for_each(|buf| {
                                  let message = String::from_utf8_lossy(&buf[..]);
                                  info!("{}", message);

                                  Ok(())
                              });

    core.run(fut).unwrap();
}
