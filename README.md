ZBar Rust
====================

[![Build Status](https://travis-ci.org/magiclen/zbar-rust.svg?branch=master)](https://travis-ci.org/magiclen/zbar-rust)
[![Build status](https://ci.appveyor.com/api/projects/status/8jpi310odv26f2pv/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/zbar-rust/branch/master)

High-level and low-level ZBar binding for the Rust language.

## Compilation

To compile this crate, you need to compile the ZBar library first. You can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.

## Examples

```rust
extern crate zbar_rust;
extern crate image;

use zbar_rust::ZBarImageScanner;

use image::GenericImageView;

let img = image::open(INPUT_IMAGE_PATH).unwrap();

let (width, height) = img.dimensions();

let luma_img = img.to_luma();

let luma_img_data: Vec<u8> = luma_img.to_vec();

let mut scanner = ZBarImageScanner::new();

let results = scanner.scan_y800(&luma_img_data, width, height).unwrap();

for result in results {
    println!("{}", String::from_utf8(result.data).unwrap())
}
```

More examples are in the `examples` folder.

## Crates.io

https://crates.io/crates/zbar-rust

## Documentation

https://docs.rs/zbar-rust

## License

[LGPL-2.1](LICENSE)