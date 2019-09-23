# Rust-AV

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Actions Status](https://github.com/rust-av/rust-av/workflows/rust-av/badge.svg)](https://github.com/rust-av/rust-av/actions)
[![Coverage Status](https://coveralls.io/repos/rust-av/rust-av/badge.svg?branch=master)](https://coveralls.io/r/rust-av/rust-av?branch=master)
[![dependency status](https://deps.rs/repo/github/rust-av/rust-av/status.svg)](https://deps.rs/repo/github/rust-av/rust-av)
[![IRC](https://img.shields.io/badge/irc-%23rust--av-blue.svg)](http://webchat.freenode.net?channels=%23rust-av&uio=d4)

Pure-rust implementation of multimedia primitives and eventually demuxer, muxers and codecs.

## Compiling

```bash
cargo build
```

## Running tests

```bash
cargo test
```

## Notes

The code is still in flux and the API is getting slowly fleshed out, please refer to the sub-crates.

## License

MIT as per `LICENSE`.

## Developing
I suggest to use the cargo [paths override](https://doc.rust-lang.org/cargo/reference/config.html) to have a local `rust-av`:

```
# Clone the trees
$ git clone https://github.com/rust-av/rust-av
$ git clone https://github.com/rust-av/${other package}
# Setup the override
$ cd ${other package}
$ mkdir .cargo
$ echo 'paths=["../rust-av"]' > .cargo/config
# Check it is doing the right thing
$ cargo build
```
