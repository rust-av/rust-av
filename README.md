# Rust-AV

[![Actions Status][actions badge]][actions]
[![CodeCov][codecov badge]][codecov]
[![dependency status][dependency badge]][dependency]
[![IRC][irc badge]][irc]
[![LICENSE][license badge]][license]

Pure-rust implementation of multimedia primitives and eventually some examples of demuxers, muxers and codecs.

## Compiling

```bash
cargo build --workspace
```

## Running tests

```bash
cargo test --workspace
```

## Examples

Examples can be found in the [examples](https://github.com/rust-av/examples) repository.

## Notes

The code is still in flux and the API is getting slowly fleshed out, please
refer to the sub-crates.
Until we reach version `1.0` assume that the API could change a lot.

## License

Released under the [MIT License](LICENSE).

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

## Getting in Touch

Come chat with us on our IRC channel clicking the badge above!
You can also use a [web client](https://web.libera.chat/?channel=#rust-av) to join with a web browser.

Otherwise, you can open a new [discussion](https://github.com/rust-av/rust-av/discussions)
explaining your idea or problem as best as possible.

<!-- Links -->
[actions]: https://github.com/rust-av/rust-av/actions
[codecov]: https://codecov.io/gh/rust-av/rust-av
[dependency]: https://deps.rs/repo/github/rust-av/rust-av
[irc]: https://web.libera.chat/?channel=#rust-av
[license]: LICENSE

<!-- Badges -->
[actions badge]: https://github.com/rust-av/rust-av/workflows/rust-av/badge.svg
[codecov badge]: https://codecov.io/gh/rust-av/rust-av/branch/master/graph/badge.svg
[dependency badge]: https://deps.rs/repo/github/rust-av/rust-av/status.svg
[irc badge]: https://img.shields.io/badge/irc-%23rust--av-blue.svg
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
