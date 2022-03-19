/*!
High-level and low-level ZBar binding for the Rust language.

## Compilation

To compile this crate, you need to compile the ZBar library first. You can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:

* `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
* `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
* `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.

## Examples

```rust,ignore
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
*/

#![allow(clippy::enum_clike_unportable_variant)]

#[macro_use]
extern crate enum_ordinalize;

use std::ptr;

use libc::{c_char, c_int, c_uint, c_ulong, c_void};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarColor {
    ZBarSpace = 0,
    ZBarBar   = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarSymbolType {
    ZBarNone       = 0,
    ZBarPartial    = 1,
    ZBarEAN2       = 2,
    ZBarEAN5       = 5,
    ZBarEAN8       = 8,
    ZBarUPCE       = 9,
    ZBarISBN10     = 10,
    ZBarUPCA       = 12,
    ZBarEAN13      = 13,
    ZBarISBN13     = 14,
    ZBarComposite  = 15,
    ZBarI25        = 25,
    ZBarDataBar    = 34,
    ZBarDataBarExp = 35,
    ZBarCodeBar    = 38,
    ZBarCode39     = 39,
    ZBarPDF417     = 57,
    ZBarQRCode     = 64,
    ZBarCode93     = 93,
    ZBarCode128    = 128,
    ZBarSymbol     = 0x00ff,
    ZBarAddOn2     = 0x0200,
    ZBarAddOn5     = 0x0500,
    ZBarAddOn      = 0x0700,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarOrientation {
    ZBarOrientUnknown = -1,
    ZBarOrientUp      = 0,
    ZBarOrientRight   = 1,
    ZBarOrientDown    = 2,
    ZBarOrientLeft    = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarError {
    ZBarOK,
    ZBarErrNoMem,
    ZBarErrInternal,
    ZBarErrUnsupported,
    ZBarErrInvalid,
    ZBarErrSystem,
    ZBarErrLocking,
    ZBarErrBudy,
    ZBarErrXDisplay,
    ZBarErrXProto,
    ZBarErrClosed,
    ZBarErrWinAPI,
    ZBarErrNum,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarConfig {
    ZBarCfgEnable    = 0,
    ZBarCfgAddCheck  = 1,
    ZBarCfgEmitCheck = 2,
    ZBarCfgASCII     = 3,
    ZBarCfgNum       = 4,
    ZBarCfgMinLen    = 0x20,
    ZBarCfgMaxLen    = 0x21,
    ZBarCfgPosition  = 0x80,
    ZBarCfgXDensity  = 0x100,
    ZBarCfgYDensity  = 0x101,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum ZBarModifier {
    ZBarModGS1,
    ZBarModAIM,
    ZBarModNum,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(isize)]
pub enum VideoControlType {
    VideoCntlInteger   = 1,
    VideoCntlMenu      = 2,
    VideoCntlButton    = 3,
    VideoCntlInteger64 = 4,
    VideoCntlString    = 5,
    VideoCntlBoolean   = 6,
}

// TODO: ----- General Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_version(major: *mut c_uint, minor: *mut c_uint) -> c_int;
    pub fn zbar_set_verbosity(verbosity: c_int);
}

// TODO: ----- General Interface END-----

// TODO: ----- Image Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_image_create() -> *mut c_void;
    pub fn zbar_image_destroy(image: *mut c_void);
    pub fn zbar_image_ref(image: *mut c_void, refs: c_int);
    pub fn zbar_image_convert(image: *const c_void, format: c_ulong) -> *mut c_void;
    pub fn zbar_image_convert_resize(
        image: *const c_void,
        format: c_ulong,
        width: c_uint,
        height: c_int,
    ) -> *mut c_void;
    pub fn zbar_image_get_format(image: *const c_void) -> c_ulong;
    pub fn zbar_image_get_sequence(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_width(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_height(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_size(image: *const c_void, width: *mut c_uint, height: *mut c_uint);
    pub fn zbar_image_get_crop(
        image: *const c_void,
        x: *mut c_uint,
        y: *mut c_uint,
        width: *mut c_uint,
        height: *mut c_uint,
    );
    pub fn zbar_image_get_data(image: *const c_void) -> *const c_void;
    pub fn zbar_image_get_data_length(image: *const c_void) -> c_ulong;
    pub fn zbar_image_get_symbols(image: *const c_void) -> *const c_void;
    pub fn zbar_image_set_symbols(image: *mut c_void, symbols: *const c_void);
    pub fn zbar_image_first_symbol(image: *const c_void) -> *const c_void;
    pub fn zbar_image_set_format(image: *mut c_void, format: c_ulong);
    pub fn zbar_image_set_sequence(image: *mut c_void, sequence_num: c_ulong);
    pub fn zbar_image_set_size(image: *mut c_void, width: c_ulong, height: c_ulong);
    pub fn zbar_image_set_crop(
        image: *mut c_void,
        x: c_ulong,
        y: c_ulong,
        width: c_ulong,
        height: c_ulong,
    );
    pub fn zbar_image_set_data(
        image: *mut c_void,
        data: *const c_void,
        data_byte_length: c_ulong,
        handler: *mut c_void,
    );
    pub fn zbar_image_free_data(image: *mut c_void);
    pub fn zbar_image_set_userdata(image: *mut c_void, userdata: *const c_void);
    pub fn zbar_image_get_userdata(image: *const c_void) -> *const c_void;
    pub fn zbar_image_write(image: *const c_void, filebase: *const c_char) -> c_uint;
    pub fn zbar_image_read(filename: *mut c_char) -> *const c_void;
}

pub struct ZBarImage {
    image: *mut c_void,
}

impl ZBarImage {
    pub fn new() -> ZBarImage {
        let image = unsafe { zbar_image_create() };

        ZBarImage {
            image,
        }
    }

    pub fn set_format(&mut self, format: u32) {
        unsafe {
            zbar_image_set_format(self.image, c_ulong::from(format));
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            zbar_image_set_size(self.image, c_ulong::from(width), c_ulong::from(height));
        }
    }

    pub fn set_ref(&mut self, r: isize) {
        unsafe {
            zbar_image_ref(self.image, r as c_int);
        }
    }

    pub fn destroy(mut self) {
        unsafe {
            zbar_image_destroy(self.image);
            self.image = ptr::null_mut();
        }
    }
}

impl Default for ZBarImage {
    #[inline]
    fn default() -> Self {
        ZBarImage::new()
    }
}

// TODO: ----- Image Interface END-----

// TODO: ----- Symbol Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_symbol_ref(symbol: *const c_void, refs: c_int);
    pub fn zbar_symbol_get_type(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_configs(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_modifiers(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_data(symbol: *const c_void) -> *mut c_char;
    pub fn zbar_symbol_get_data_length(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_quality(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_count(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_get_loc_size(symbol: *const c_void) -> c_uint;
    pub fn zbar_symbol_get_loc_x(symbol: *const c_void, index: c_uint) -> c_int;
    pub fn zbar_symbol_get_loc_y(symbol: *const c_void, index: c_uint) -> c_int;
    pub fn zbar_symbol_get_orientation(symbol: *const c_void) -> c_int;
    pub fn zbar_symbol_next(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_get_components(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_first_component(symbol: *const c_void) -> *const c_void;
    pub fn zbar_symbol_xml(
        symbol: *const c_void,
        buffer: *mut *mut c_char,
        buflen: *mut c_uint,
    ) -> *mut c_char;
}

// TODO: ----- Symbol Interface END-----

// TODO: ----- Image Scanner Interface START-----

#[link(name = "zbar")]
extern {
    pub fn zbar_image_scanner_create() -> *mut c_void;
    pub fn zbar_image_scanner_destroy(scanner: *mut c_void);
    pub fn zbar_image_scanner_set_data_handler(
        scanner: *mut c_void,
        handler: *const c_void,
        userdata: *const c_void,
    );
    pub fn zbar_image_scanner_set_config(
        scanner: *mut c_void,
        symbology: c_int,
        config: c_int,
        value: c_int,
    ) -> c_int;
    pub fn zbar_image_scanner_parse_config(
        scanner: *mut c_void,
        config_string: *const c_char,
    ) -> c_int;
    pub fn zbar_image_scanner_enable_cache(scanner: *mut c_void, enable: c_int);
    pub fn zbar_image_scanner_recycle_image(scanner: *mut c_void, image: *mut c_void);
    pub fn zbar_image_scanner_get_results(scanner: *const c_void) -> *const c_void;
    pub fn zbar_scan_image(scanner: *mut c_void, image: *mut c_void) -> c_int;
}

#[derive(Debug)]
pub struct ZBarImageScanResult {
    pub symbol_type: ZBarSymbolType,
    pub data: Vec<u8>,
}

pub struct ZBarImageScanner {
    scanner: *mut c_void,
}

impl ZBarImageScanner {
    pub fn new() -> ZBarImageScanner {
        let scanner = unsafe { zbar_image_scanner_create() };

        ZBarImageScanner {
            scanner,
        }
    }

    pub fn set_config(
        &mut self,
        symbology: ZBarSymbolType,
        config: ZBarConfig,
        value: isize,
    ) -> Result<(), &'static str> {
        let result = unsafe {
            zbar_image_scanner_set_config(
                self.scanner,
                symbology.ordinal() as c_int,
                config.ordinal() as c_int,
                value as c_int,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err("unsuccessfully")
        }
    }

    pub fn destroy(mut self) {
        unsafe {
            zbar_image_scanner_destroy(self.scanner);
            self.scanner = ptr::null_mut();
        }
    }

    pub fn scan_y800<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
    ) -> Result<Vec<ZBarImageScanResult>, &'static str> {
        //        let format: u32 = unsafe { transmute([b'Y', b'8', b'0', b'0']) };
        self.scan(data, width, height, 808_466_521)
    }

    pub fn scan_gray<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
    ) -> Result<Vec<ZBarImageScanResult>, &'static str> {
        //        let format: u32 = unsafe { transmute([b'G', b'R', b'A', b'Y']) };
        self.scan(data, width, height, 1_497_453_127)
    }

    pub fn scan<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
        format: u32,
    ) -> Result<Vec<ZBarImageScanResult>, &'static str> {
        let data = data.as_ref();

        let mut image = ZBarImage::new();

        image.set_size(width, height);
        image.set_format(format);

        unsafe {
            zbar_image_set_data(
                image.image,
                data.as_ptr() as *const c_void,
                data.len() as c_ulong,
                zbar_image_free_data as *mut c_void,
            );
        }

        let n = unsafe { zbar_scan_image(self.scanner, image.image) };

        if n < 0 {
            return Err("incorrect image");
        }

        let mut result_array = Vec::with_capacity(n as usize);

        let mut symbol = unsafe { zbar_image_first_symbol(image.image) };

        while !symbol.is_null() {
            let symbol_type = unsafe { zbar_symbol_get_type(symbol) };
            let symbol_type = unsafe { ZBarSymbolType::from_ordinal_unsafe(symbol_type as isize) };
            let data = unsafe {
                let data = zbar_symbol_get_data(symbol);
                let data_length = zbar_symbol_get_data_length(symbol) as usize;
                Vec::from_raw_parts(data as *mut u8, data_length, data_length)
            };

            let result = ZBarImageScanResult {
                symbol_type,
                data,
            };

            result_array.push(result);

            symbol = unsafe { zbar_symbol_next(symbol) };
        }

        Ok(result_array)
    }
}

impl Default for ZBarImageScanner {
    #[inline]
    fn default() -> Self {
        ZBarImageScanner::new()
    }
}

impl Drop for ZBarImageScanner {
    fn drop(&mut self) {
        if !self.scanner.is_null() {
            unsafe {
                zbar_image_scanner_destroy(self.scanner);
            }
        }
    }
}
// TODO: ----- Image Scanner Interface END-----
