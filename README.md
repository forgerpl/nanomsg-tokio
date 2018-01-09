# Tokio wrapper for Nanomsg

## The purpose

While nanomsg provides dedicated non-blocking APIs (nn_poll, NN_DONTWAIT) their use is limited to nanomsg sockets only.
This library implements native Tokio adapters (Stream and Sink) on top of nanomsg's raw file descriptors and thus allows for multiplexing nanomsg socket's IO with other asynchronous libraries.

## Running examples

There are two example applications available: `listener` and `sender`.

```sh
cargo run --example=listener
```
```sh
cargo run --example=sender
```
