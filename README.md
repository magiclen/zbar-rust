ZBar Rust
====================

[![CI](https://github.com/magiclen/zbar-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/zbar-rust/actions/workflows/ci.yml)

High-level and low-level ZBar binding for the Rust language.

## Compilation

To compile this crate, you need ZBar 0.22 or later.
`pkg-config` can find ZBar and its link dependencies.

To use a custom installation, set `ZBAR_DIR` to its prefix, or set both `ZBAR_LIB_DIRS` and `ZBAR_INCLUDE_DIRS`:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`.
  Separate paths with `;` on a Windows build host or `:` on other build hosts.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-I`.
  Use the same path separator as `ZBAR_LIB_DIRS`.
* `ZBAR_DIR`: A prefix whose `lib` and `include` subdirectories are used when `ZBAR_LIB_DIRS` or `ZBAR_INCLUDE_DIRS` is not set.
* `ZBAR_LIBS`: The library names to link, separated by `:` on every platform.
  Setting this variable replaces the library list from `pkg-config`.

When library directories are set manually, the default library name is `zbar`.
Set `ZBAR_LIBS` to include any required dependencies, such as `zbar:iconv` for a build that uses a separate iconv library.
When both library and header directories are provided, `pkg-config` is not needed; you must ensure that the installed ZBar is version 0.22 or later.
Missing directories are found with `pkg-config`, and an include-only override keeps automatic library discovery.

The following environment variables are optional:

* `ZBAR_STATIC`: Set it to `0` to select dynamic linking, or to anything else to select static linking.
  When it is not set, dynamic linking is preferred when both kinds are available.
  Automatic discovery queries the private dependencies again if only a static ZBar library is available; dependencies may use dynamic linking when needed.
* `ZBAR_DYLIB_STDCPP`: Set it to anything but `0` to also link `stdc++` dynamically.

Automatic library discovery preserves the frameworks, library files and linker arguments reported by `pkg-config`.
With manual linking, `ZBAR_LIBS` must contain the complete library list.

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
