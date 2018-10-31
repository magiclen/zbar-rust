//! High-level and low-level ZBar binding for the Rust language.
//!
//! ## Compilation
//!
//! To compile this crate, you need to compile the ZBar library first. You can install ZBar in your operating system, or in somewhere in your file system. As for the latter, you need to set the following environment variables to link the ZBar library:
//!
//! * `ZBAR_LIB_DIRS`: The directories of library files, like `-L`. Use `:` to separate.
//! * `ZBAR_LIBS`: The library names that you want to link, like `-l`. Use `:` to separate. Typically, it is **iconv:zbar**.
//! * `ZBAR_INCLUDE_DIRS`: The directories of header files, like `-i`. Use `:` to separate.
//!
//! ## Examples
//!
//! See `tests/tests.rs`.

#[macro_use]
extern crate enum_ordinalize;

extern crate libc;

use std::io::{self, Read, ErrorKind};
use std::mem::transmute;
use std::ptr;

use libc::{c_void, c_int, c_char, c_uint, c_ulong};

create_ordinalized_enum!(pub ZBarColor,
    ZBarSpace = 0,
    ZBarBar = 1,
);

create_ordinalized_enum!(pub ZBarSymbolType,
    ZBarNone = 0,
    ZBarPartial = 1,
    ZBarEAN2 = 2,
    ZBarEAN5 = 5,
    ZBarEAN8 = 8,
    ZBarUPCE = 9,
    ZBarISBN10 = 10,
    ZBarUPCA = 12,
    ZBarEAN13 = 13,
    ZBarISBN13 = 14,
    ZBarComposite = 15,
    ZBarI25 = 25,
    ZBarDataBar = 34,
    ZBarDataBarExp = 35,
    ZBarCodeBar = 38,
    ZBarCode39 = 39,
    ZBarPDF417 = 57,
    ZBarQRCode = 64,
    ZBarCode93 = 93,
    ZBarCode128 = 128,
    ZBarSymbol = 0x00ff,
    ZBarAddOn2 = 0x0200,
    ZBarAddOn5 = 0x0500,
    ZBarAddOn = 0x0700,
);

create_ordinalized_enum!(pub ZBarOrientation,
    ZBarOrientUnknown = -1,
    ZBarOrientUp = 0,
    ZBarOrientRight = 1,
    ZBarOrientDown = 2,
    ZBarOrientLeft = 3
);

create_ordinalized_enum!(pub ZBarError,
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
    ZBarErrNum
);

create_ordinalized_enum!(pub ZBarConfig,
    ZBarCfgEnable = 0,
    ZBarCfgAddCheck = 1,
    ZBarCfgEmitCheck = 2,
    ZBarCfgASCII = 3,
    ZBarCfgNum = 4,
    ZBarCfgMinLen = 0x20,
    ZBarCfgMaxLen = 0x21,
    ZBarCfgPosition = 0x80,
    ZBarCfgXDensity = 0x100,
    ZBarCfgYDensity = 0x101
);

create_ordinalized_enum!(pub ZBarModifier,
    ZBarModGS1,
    ZBarModAIM,
    ZBarModNum
);

create_ordinalized_enum!(pub VideoControlType,
    VideoCntlInteger = 1,
    VideoCntlMenu = 2,
    VideoCntlButton = 3,
    VideoCntlInteger64 = 4,
    VideoCntlString = 5,
    VideoCntlBoolean = 6,
);

// TODO: ----- General Interface START-----

#[link(name = "zbar")]
extern "C" {
    pub fn zbar_version(major: *mut c_uint, minor: *mut c_uint) -> c_int;
    pub fn zbar_set_verbosity(verbosity: c_int);
}

#[cfg(test)]
mod gereral_tests {
    use super::*;

    #[test]
    fn test_version() {
        let mut major = 0;
        let mut minor = 0;
        let result = unsafe { zbar_version(&mut major, &mut minor) };

        assert_eq!(0, result);
        assert_eq!(0, major);
        assert!(minor >= 10 && minor <= 20);
    }
}

// TODO: ----- General Interface END-----

// TODO: ----- Image Interface START-----

#[link(name = "zbar")]
extern "C" {
    pub fn zbar_image_create() -> *mut c_void;
    pub fn zbar_image_destroy(image: *mut c_void);
    pub fn zbar_image_ref(image: *mut c_void, refs: c_int);
    pub fn zbar_image_convert(image: *const c_void, format: c_ulong) -> *mut c_void;
    pub fn zbar_image_convert_resize(image: *const c_void, format: c_ulong, width: c_uint, height: c_int) -> *mut c_void;
    pub fn zbar_image_get_format(image: *const c_void) -> c_ulong;
    pub fn zbar_image_get_sequence(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_width(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_height(image: *const c_void) -> c_uint;
    pub fn zbar_image_get_size(image: *const c_void, width: *mut c_uint, height: *mut c_uint);
    pub fn zbar_image_get_crop(image: *const c_void, x: *mut c_uint, y: *mut c_uint, width: *mut c_uint, height: *mut c_uint);
    pub fn zbar_image_get_data(image: *const c_void) -> *const c_void;
    pub fn zbar_image_get_data_length(image: *const c_void) -> c_ulong;
    pub fn zbar_image_get_symbols(image: *const c_void) -> *const c_void;
    pub fn zbar_image_set_symbols(image: *mut c_void, symbols: *const c_void);
    pub fn zbar_image_first_symbol(image: *const c_void) -> *const c_void;
    pub fn zbar_image_set_format(image: *mut c_void, format: c_ulong);
    pub fn zbar_image_set_sequence(image: *mut c_void, sequence_num: c_ulong);
    pub fn zbar_image_set_size(image: *mut c_void, width: c_ulong, height: c_ulong);
    pub fn zbar_image_set_crop(image: *mut c_void, x: c_ulong, y: c_ulong, width: c_ulong, height: c_ulong);
    pub fn zbar_image_set_data(image: *mut c_void, data: *const c_void, data_byte_length: c_ulong, handler: *mut c_void);
    pub fn zbar_image_free_data(image: *mut c_void);
    pub fn zbar_image_set_userdata(image: *mut c_void, userdata: *const c_void);
    pub fn zbar_image_get_userdata(image: *const c_void) -> *const c_void;
    pub fn zbar_image_write(image: *const c_void, filebase: *const c_char) -> c_uint;
    pub fn zbar_image_read(filename: *mut c_char) -> *const c_void;
}

pub struct ZBarImage {
    image: *mut c_void,
}

unsafe impl Send for ZBarImage {}

unsafe impl Sync for ZBarImage {}

impl ZBarImage {
    pub fn new() -> ZBarImage {
        let image = unsafe { zbar_image_create() };

        ZBarImage {
            image
        }
    }

    pub fn set_format(&mut self, format: u32) {
        unsafe {
            zbar_image_set_format(self.image, format as c_ulong);
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            zbar_image_set_size(self.image, width as c_ulong, height as c_ulong);
        }
    }

    pub fn destroy(&mut self) {
        if !self.image.is_null() {
            unsafe {
                zbar_image_destroy(self.image);
                self.image = ptr::null_mut();
            }
        }
    }
}

#[cfg(test)]
mod image_tests {
    use super::*;

    #[test]
    fn test_image_create_destroy() {
        let _zbar = ZBarImage::new();
    }
}

// TODO: ----- Image Interface END-----

// TODO: ----- Symbol Interface START-----

#[link(name = "zbar")]
extern "C" {
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
    pub fn zbar_symbol_xml(symbol: *const c_void, buffer: *mut (*mut c_char), buflen: *mut c_uint) -> *mut c_char;
}

#[cfg(test)]
mod symbol_tests {
//    use super::*;
}

// TODO: ----- Symbol Interface END-----

// TODO: ----- Image Scanner Interface START-----

#[link(name = "zbar")]
extern "C" {
    pub fn zbar_image_scanner_create() -> *mut c_void;
    pub fn zbar_image_scanner_destroy(scanner: *mut c_void);
    pub fn zbar_image_scanner_set_data_handler(scanner: *mut c_void, handler: unsafe extern fn(*mut c_void), userdata: *const c_void);
    pub fn zbar_image_scanner_set_config(scanner: *mut c_void, symbology: c_int, config: c_int, value: c_int) -> c_int;
    pub fn zbar_image_scanner_parse_config(scanner: *mut c_void, config_string: *const c_char) -> c_int;
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

unsafe impl Send for ZBarImageScanner {}

unsafe impl Sync for ZBarImageScanner {}

impl ZBarImageScanner {
    pub fn new() -> ZBarImageScanner {
        let scanner = unsafe { zbar_image_scanner_create() };

        ZBarImageScanner {
            scanner
        }
    }

    pub fn set_config(&mut self, symbology: ZBarSymbolType, config: ZBarConfig, value: isize) -> Result<(), &'static str> {
        let result = unsafe { zbar_image_scanner_set_config(self.scanner, symbology.ordinal() as c_int, config.ordinal() as c_int, value as c_int) };
        if result == 0 {
            Ok(())
        } else {
            Err("unsuccessfully")
        }
    }

    pub fn scan_y800<R: Read>(&mut self, reader: R, width: u32, height: u32) -> Result<Vec<ZBarImageScanResult>, io::Error> {
        let format: u32 = unsafe { transmute([b'Y', b'8', b'0', b'0']) };
        self.scan(reader, width, height, format)
    }

    pub fn scan_gray<R: Read>(&mut self, reader: R, width: u32, height: u32) -> Result<Vec<ZBarImageScanResult>, io::Error> {
        let format: u32 = unsafe { transmute([b'G', b'R', b'A', b'Y']) };
        self.scan(reader, width, height, format)
    }

    pub fn scan<R: Read>(&mut self, mut reader: R, width: u32, height: u32, format: u32) -> Result<Vec<ZBarImageScanResult>, io::Error> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        let mut image = ZBarImage::new();

        image.set_size(width, height);
        image.set_format(format);

        unsafe {
            zbar_image_set_data(image.image, data.as_ptr() as *const c_void, data.len() as c_ulong, zbar_image_free_data as *mut c_void);
        }

        let n = unsafe { zbar_scan_image(self.scanner, image.image) };

        if n < 0 {
            return Err(io::Error::new(ErrorKind::Other, "incorrect image"));
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

impl Drop for ZBarImageScanner {
    fn drop(&mut self) {
        if !self.scanner.is_null() {
            unsafe {
                zbar_image_scanner_destroy(self.scanner);
            }
        }
    }
}

#[cfg(test)]
mod image_scanner_tests {
    use super::*;

    #[test]
    fn test_image_create_destroy() {
        let _scanner = ZBarImageScanner::new();
    }

    #[test]
    fn test_set_config() {
        let mut scanner = ZBarImageScanner::new();
        scanner.set_config(ZBarSymbolType::ZBarNone, ZBarConfig::ZBarCfgEnable, 0).unwrap();
        scanner.set_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgEnable, 1).unwrap();
    }
}

// TODO: ----- Image Scanner Interface END-----