# Rust-AV examples

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![dependency status](https://deps.rs/repo/github/rust-av/rust-av/status.svg)](https://deps.rs/repo/github/rust-av/rust-av)
[![IRC](https://img.shields.io/badge/irc-%23rust--av-blue.svg)](http://webchat.freenode.net?channels=%23rust-av&uio=d4)

A series of some multimedia examples

## Build examples

To build the examples, you need to install `libvpx` and `libopus` on your
operating system.

```bash
cargo build --examples
```

## Running examples

```bash
cargo run --example EXAMPLE_NAME -- [EXAMPLE_ARGUMENTS]
```

For example, if you want to run the `streams_info` example:

```bash
cargo run --example streams_info -- -i /path/to/your/matroska/file
```

## License

MIT as per `LICENSE`.
