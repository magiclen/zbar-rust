OpenCC Rust
====================

[![Build Status](https://travis-ci.org/magiclen/zbar-rust.svg?branch=master)](https://travis-ci.org/magiclen/zbar-rust)

High-level and low-level ZBar binding for the Rust language.

## Compilation

To compile this crate, you need to compile the ZBar library first. You can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.

## Examples

See `tests/tests.rs`.

## Crates.io

https://crates.io/crates/zbar-rust

## Documentation

https://docs.rs/zbar-rust

## License

[LGPL-2.1](LICENSE)