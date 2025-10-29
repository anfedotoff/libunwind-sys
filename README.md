[![crates.io](https://img.shields.io/crates/v/libunwind-sys.svg)](https://crates.io/crates/libunwind-sys)
# libunwind-sys

This Rust crate provides low-level bindings for the [libunwind] library.

Supported targets:
* x86_64-unknown-linux-gnu;
* i686-unknown-linux-gnu;
* i586-unknown-linux-gnu;
* arm-unknown-linux-gnueabihf;
* x86_64-unknown-linux-musl.

Tests are provided only for x86_64-unknown-linux-gnu target.

[libunwind]: https://www.nongnu.org/libunwind/

## Installation

First, compile `libunwind` as a shared library and install it. Add this crate to your `Cargo.toml`:

```toml
[dependencies]
libunwind-sys = "0.6.0"
```
## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
