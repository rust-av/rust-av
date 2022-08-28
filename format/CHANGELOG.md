## Version 0.7.0

- Simplify the `Muxer` trait to have one `Write` generic instead of two.

## Version 0.6.0

- Remove `Send` requirement from `Context`'s `Muxer`.
- Removed the `WriteOwned` and `WriteSeek` traits. These were too restrictive for real-world use cases.
  - Users should use `Write` or `Write + Seek` directly.
  - This is technically a breaking change, as there are some usages which will break. These usages should be discouraged anyway, as there shouldn't be a need for the caller to take ownership of the writer, since the caller should already be the one owning the underlying buffer when it creates the writer.
- Upgrade to Rust edition 2021.
- Improve documentation.
