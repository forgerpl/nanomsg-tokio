use colored_logger::FormatterBuilder;
use futures::{stream, Stream};
use nanomsg::result::Error as NanoError;
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

    let mut socket = Socket::new(Protocol::Push, &core.handle()).unwrap();
    socket.connect("ipc:///tmp/nanomsg-tokio.ipc").unwrap();

    let gen = stream::repeat::<_, NanoError>("test message".into());

    let _ = core.run(gen.forward(socket)).unwrap();
}
