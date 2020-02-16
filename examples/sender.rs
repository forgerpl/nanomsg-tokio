extern crate flexi_logger;
extern crate nanomsg_tokio;
extern crate futures;
extern crate nanomsg;
extern crate tokio_core;
extern crate colored_logger;

use tokio_core::reactor::Core;
use nanomsg_tokio::Socket;
use nanomsg::Protocol;
use nanomsg::result::Error as NanoError;
use futures::Stream;
use futures::stream;
use colored_logger::FormatterBuilder;


fn main() {
    let formatter = FormatterBuilder::default().build();
    flexi_logger::Logger::with_str("info")
        .format(formatter)
        .start()
        .unwrap();

    let mut core = Core::new().unwrap();

    let mut socket = Socket::new(Protocol::Push, &core.handle()).unwrap();
    socket.connect("ipc:///tmp/nanomsg-tokio.ipc").unwrap();

    let gen = stream::repeat::<_, NanoError>("test message".into());

    let _ = core.run(gen.forward(socket)).unwrap();
}
