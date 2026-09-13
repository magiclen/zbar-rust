ZBar Rust
====================

[![CI](https://github.com/magiclen/zbar-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/zbar-rust/actions/workflows/ci.yml)

High-level and low-level ZBar binding for the Rust language.

## Compilation

To compile this crate, you need to compile the ZBar library first. You can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.

The following environment variables are optional:

* `ZBAR_DIR`: A prefix whose `lib` and `include` subdirectories are used when `ZBAR_LIB_DIRS` or `ZBAR_INCLUDE_DIRS` is not set.
* `ZBAR_STATIC`: Set it to `0` to force dynamic linking, or to anything else to force static linking. When it is not set, the library files that are actually present decide.
* `ZBAR_DYLIB_STDCPP`: Set it to anything but `0` to also link `stdc++` dynamically.

When none of `ZBAR_LIB_DIRS`, `ZBAR_INCLUDE_DIRS` and `ZBAR_DIR` is set, `pkg-config` is used to find ZBar.

## Examples

```rust
use image::GenericImageView;
use zbar_rust::ZBarImageScanner;

let img = image::open("examples/data/magiclen.org.png").unwrap();

let (width, height) = img.dimensions();

let mut scanner = ZBarImageScanner::new();

let results = scanner.scan_y800(img.into_luma8().into_raw(), width, height).unwrap();

for result in results {
    println!("{}", String::from_utf8(result.data).unwrap())
}
```

ZBar only scans 8-bit grayscale images. Use `ZBarImage::convert` for anything else, and the `zbar_rust::ffi` module for the parts of the C API this crate does not wrap.

More examples are in the `examples` folder.

## Crates.io

https://crates.io/crates/zbar-rust

## Documentation

https://docs.rs/zbar-rust

## License

[LGPL-2.1](LICENSE)