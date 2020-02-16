use colored_logger::FormatterBuilder;
use futures::Stream;
use log::*;
use nanomsg::Protocol;
use nanomsg_tokio::Socket;
use tokio_core::reactor::Core;

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
