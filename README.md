[![crates.io](https://img.shields.io/crates/v/libunwind-sys.svg)](https://crates.io/crates/libunwind-sys)
# libunwind-sys

This Rust crate provides low-level bindings for the [libunwind] library.
It has the following limitations:

*  x86/x86_64/arm targets are supported for now;
* tests are provided only for x86_64 target;

[libunwind]: https://www.nongnu.org/libunwind/

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
libunwind-sys = "0.1.3"
```
## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
