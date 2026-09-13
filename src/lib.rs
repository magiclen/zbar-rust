/*!
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

```no_run
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
*/

pub mod ffi;

use core::{
    ffi::{CStr, c_int, c_ulong},
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    slice,
};
use std::{error::Error, ffi::CString};

use enum_ordinalize::Ordinalize;

/// Builds a fourcc format code out of its four characters, the same way the `zbar_fourcc` macro does in C.
#[inline]
pub const fn fourcc(code: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*code)
}

/// The fourcc code of the 8-bit grayscale format that ZBar calls `Y800`.
pub const FOURCC_Y800: u32 = fourcc(b"Y800");

/// The fourcc code of the 8-bit grayscale format that ZBar calls `GREY`.
///
/// It is the same layout as [`FOURCC_Y800`]. Note the spelling: ZBar does not recognize `GRAY`.
pub const FOURCC_GREY: u32 = fourcc(b"GREY");

/// Whether a scanned area is a bar or the space between bars.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(i32)]
pub enum ZBarColor {
    /// A light area.
    ZBarSpace = 0,
    /// A dark area.
    ZBarBar   = 1,
}

/// The symbology of a decoded symbol.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(i32)]
pub enum ZBarSymbolType {
    /// No symbol was decoded.
    ZBarNone       = 0,
    /// An intermediate status, not a finished symbol.
    ZBarPartial    = 1,
    /// A GS1 2-digit add-on.
    ZBarEAN2       = 2,
    /// A GS1 5-digit add-on.
    ZBarEAN5       = 5,
    /// EAN-8.
    ZBarEAN8       = 8,
    /// UPC-E.
    ZBarUPCE       = 9,
    /// ISBN-10, derived from EAN-13.
    ZBarISBN10     = 10,
    /// UPC-A.
    ZBarUPCA       = 12,
    /// EAN-13.
    ZBarEAN13      = 13,
    /// ISBN-13, derived from EAN-13.
    ZBarISBN13     = 14,
    /// An EAN/UPC composite.
    ZBarComposite  = 15,
    /// Interleaved 2 of 5.
    ZBarI25        = 25,
    /// GS1 DataBar, formerly RSS.
    ZBarDataBar    = 34,
    /// GS1 DataBar Expanded.
    ZBarDataBarExp = 35,
    /// Codabar.
    ZBarCodeBar    = 38,
    /// Code 39.
    ZBarCode39     = 39,
    /// PDF417.
    ZBarPDF417     = 57,
    /// QR Code.
    ZBarQRCode     = 64,
    /// SQ Code.
    ZBarSQCode     = 80,
    /// Code 93.
    ZBarCode93     = 93,
    /// Code 128.
    ZBarCode128    = 128,
    /// The mask of the base symbol type. Deprecated since ZBar 0.11.
    ZBarSymbol     = 0x00FF,
    /// The 2-digit add-on flag. Deprecated since ZBar 0.11.
    ZBarAddOn2     = 0x0200,
    /// The 5-digit add-on flag. Deprecated since ZBar 0.11.
    ZBarAddOn5     = 0x0500,
    /// The mask of the add-on flags. Deprecated since ZBar 0.11.
    ZBarAddOn      = 0x0700,
}

impl ZBarSymbolType {
    /// Returns the name ZBar gives this symbology, such as `QR-Code`.
    #[inline]
    pub fn name(self) -> &'static str {
        // The pointer is a string literal inside ZBar, so it lives forever and is always valid ASCII.
        unsafe { CStr::from_ptr(ffi::zbar_get_symbol_name(self.ordinal())).to_str().unwrap() }
    }
}

/// The coarse orientation of a decoded symbol.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(i32)]
pub enum ZBarOrientation {
    /// The orientation could not be determined.
    ZBarOrientUnknown = -1,
    /// Upright, read left to right.
    ZBarOrientUp      = 0,
    /// Sideways, read top to bottom.
    ZBarOrientRight   = 1,
    /// Upside-down, read right to left.
    ZBarOrientDown    = 2,
    /// Sideways, read bottom to top.
    ZBarOrientLeft    = 3,
}

impl ZBarOrientation {
    /// Returns the name ZBar gives this orientation, such as `UP`.
    #[inline]
    pub fn name(self) -> &'static str {
        unsafe { CStr::from_ptr(ffi::zbar_get_orientation_name(self.ordinal())).to_str().unwrap() }
    }
}

/// An error code reported by the ZBar library itself.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(i32)]
pub enum ZBarErrorCode {
    /// No error.
    ZBarOK,
    /// Out of memory.
    ZBarErrNoMem,
    /// An internal library error.
    ZBarErrInternal,
    /// An unsupported request.
    ZBarErrUnsupported,
    /// An invalid request.
    ZBarErrInvalid,
    /// A system error.
    ZBarErrSystem,
    /// A locking error.
    ZBarErrLocking,
    /// All resources are busy.
    ZBarErrBusy,
    /// An X11 display error.
    ZBarErrXDisplay,
    /// An X11 protocol error.
    ZBarErrXProto,
    /// The output window is closed.
    ZBarErrClosed,
    /// A Windows system error.
    ZBarErrWinAPI,
    /// The number of error codes.
    ZBarErrNum,
}

/// One configuration setting of a scanner.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(i32)]
pub enum ZBarConfig {
    /// Enables or disables a symbology or feature.
    ZBarCfgEnable       = 0,
    /// Enables the check digit where it is optional.
    ZBarCfgAddCheck     = 1,
    /// Returns the check digit where it is present.
    ZBarCfgEmitCheck    = 2,
    /// Enables the full ASCII character set.
    ZBarCfgASCII        = 3,
    /// Keeps binary data as it is instead of converting it to text.
    ZBarCfgBinary       = 4,
    /// The number of boolean decoder settings.
    ZBarCfgNum          = 5,
    /// The shortest data length that counts as a valid decode.
    ZBarCfgMinLen       = 0x20,
    /// The longest data length that counts as a valid decode.
    ZBarCfgMaxLen       = 0x21,
    /// How many consistent video frames are required.
    ZBarCfgUncertainty  = 0x40,
    /// Lets the scanner collect position data.
    ZBarCfgPosition     = 0x80,
    /// Retries with an inverted image when decoding fails.
    ZBarCfgTestInverted = 0x81,
    /// The vertical scan density of the image scanner.
    ZBarCfgXDensity     = 0x100,
    /// The horizontal scan density of the image scanner.
    ZBarCfgYDensity     = 0x101,
}

impl ZBarConfig {
    /// Returns the name ZBar gives this setting, such as `ENABLE`.
    #[inline]
    pub fn name(self) -> &'static str {
        unsafe { CStr::from_ptr(ffi::zbar_get_config_name(self.ordinal())).to_str().unwrap() }
    }
}

/// A decoder modifier that was in effect for a symbol.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(i32)]
pub enum ZBarModifier {
    /// The symbol carries GS1 data.
    ZBarModGS1,
    /// The symbol carries AIM data.
    ZBarModAIM,
    /// The number of modifiers.
    ZBarModNum,
}

impl ZBarModifier {
    /// Returns the name ZBar gives this modifier, such as `GS1`.
    #[inline]
    pub fn name(self) -> &'static str {
        unsafe { CStr::from_ptr(ffi::zbar_get_modifier_name(self.ordinal())).to_str().unwrap() }
    }
}

/// The kind of a video control.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ordinalize)]
#[repr(i32)]
pub enum VideoControlType {
    /// An integer value.
    VideoCntlInteger   = 1,
    /// A choice out of a menu.
    VideoCntlMenu      = 2,
    /// A button that triggers an action.
    VideoCntlButton    = 3,
    /// A 64-bit integer value.
    VideoCntlInteger64 = 4,
    /// A string value.
    VideoCntlString    = 5,
    /// A boolean value.
    VideoCntlBoolean   = 6,
}

/// An error returned by the safe API of this crate.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ZBarError {
    /// The image buffer is too small for the given width and height.
    InsufficientData {
        /// How many bytes the image needs.
        expected: u64,
        /// How many bytes the buffer holds.
        actual:   u64,
    },
    /// ZBar cannot scan this image format. It only scans [`FOURCC_Y800`] and [`FOURCC_GREY`].
    UnsupportedImageFormat(u32),
    /// ZBar reported a symbol type this crate does not know about.
    UnknownSymbolType(i32),
    /// ZBar rejected the configuration, because it does not apply to the symbology or is out of range.
    InvalidConfig,
    /// ZBar cannot convert the image to the requested format.
    ImageConversionFailed,
}

impl Display for ZBarError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientData {
                expected,
                actual,
            } => {
                write!(f, "the image needs {expected} bytes of data but only {actual} were given")
            },
            Self::UnsupportedImageFormat(format) => {
                let code = format.to_le_bytes();

                write!(
                    f,
                    "ZBar cannot scan images in the {} format",
                    String::from_utf8_lossy(&code)
                )
            },
            Self::UnknownSymbolType(symbol_type) => {
                write!(f, "ZBar reported the unknown symbol type {symbol_type}")
            },
            Self::InvalidConfig => f.write_str("ZBar rejected the configuration"),
            Self::ImageConversionFailed => {
                f.write_str("ZBar cannot convert the image to the requested format")
            },
        }
    }
}

impl Error for ZBarError {}

/// Returns the version of the ZBar library as `(major, minor, patch)`.
#[inline]
pub fn version() -> (u32, u32, u32) {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;

    unsafe {
        ffi::zbar_version(&mut major, &mut minor, &mut patch);
    }

    (major, minor, patch)
}

/// Sets how much diagnostic output ZBar writes to stderr. 0 turns it off.
///
/// # Safety
///
/// No other thread may call ZBar functions while this function runs, because ZBar reads and writes its global verbosity level without a lock.
#[inline]
pub unsafe fn set_verbosity(verbosity: i32) {
    unsafe {
        ffi::zbar_set_verbosity(verbosity);
    }
}

/// Raises the verbosity level of ZBar.
///
/// # Safety
///
/// No other thread may call ZBar functions while this function runs, because ZBar reads and writes its global verbosity level without a lock.
/// The current level must be nonnegative, and doubling it must fit in `i32`.
#[inline]
pub unsafe fn increase_verbosity() {
    unsafe {
        ffi::zbar_increase_verbosity();
    }
}

/// ZBar calls this when it releases the sample data of an image. The data is borrowed from Rust, so nothing must be freed here.
unsafe extern "C" fn keep_borrowed_data(_image: *mut ffi::zbar_image_t) {}

// The buffer was passed to ZBar with `Box::into_raw`, and its length fits in `c_ulong`.
unsafe extern "C" fn free_owned_data(image: *mut ffi::zbar_image_t) {
    unsafe {
        let data = ffi::zbar_image_get_data(image).cast_mut().cast::<u8>();
        let len = ffi::zbar_image_get_data_length(image) as usize;

        drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(data, len)));
    }
}

/// An image that ZBar can scan or convert.
///
/// The lifetime is that of the sample data the image borrows, so the buffer cannot be dropped while the image still points at it.
pub struct ZBarImage<'a> {
    image: *mut ffi::zbar_image_t,
    data:  PhantomData<&'a [u8]>,
}

// A `ZBarImage` owns its handle, and ZBar keeps no global mutable state for images.
unsafe impl Send for ZBarImage<'_> {}

impl<'a> ZBarImage<'a> {
    /// Creates an image in the `Y800` format, which is 8-bit grayscale, one byte per pixel.
    ///
    /// It fails when `data` holds fewer than `width * height` bytes.
    #[inline]
    pub fn new_y800(data: &'a [u8], width: u32, height: u32) -> Result<Self, ZBarError> {
        Self::new_grayscale(data, width, height, FOURCC_Y800)
    }

    /// Creates an image in the `GREY` format, which has the same layout as `Y800`.
    ///
    /// It fails when `data` holds fewer than `width * height` bytes.
    ///
    /// Prefer the `Y800` format: ZBar decodes both, but its SQ Code decoder writes `Unexpected image format` to stderr for `GREY`.
    #[inline]
    pub fn new_grey(data: &'a [u8], width: u32, height: u32) -> Result<Self, ZBarError> {
        Self::new_grayscale(data, width, height, FOURCC_GREY)
    }

    fn new_grayscale(
        data: &'a [u8],
        width: u32,
        height: u32,
        format: u32,
    ) -> Result<Self, ZBarError> {
        let expected = u64::from(width) * u64::from(height);
        let actual = data.len() as u64;

        if actual < expected {
            return Err(ZBarError::InsufficientData {
                expected,
                actual,
            });
        }

        // SAFETY: the buffer is at least `width * height` bytes, which is all ZBar reads for these formats.
        Ok(unsafe { Self::new_with_format(data, width, height, format) })
    }

    /// Creates an image in any format ZBar understands.
    ///
    /// # Safety
    ///
    /// `data` must be large enough to hold a `width` by `height` image in `format`. ZBar reads the buffer from its size and format alone and never looks at the length that was given to it, so a buffer that is too small is read out of bounds.
    ///
    /// Prefer [`ZBarImage::new_y800`] or [`ZBarImage::new_grey`], which check the length for you.
    pub unsafe fn new_with_format(data: &'a [u8], width: u32, height: u32, format: u32) -> Self {
        let image = unsafe { ffi::zbar_image_create() };

        assert!(!image.is_null(), "ZBar cannot allocate an image");

        unsafe {
            ffi::zbar_image_set_format(image, c_ulong::from(format));
            ffi::zbar_image_set_size(image, width, height);
            ffi::zbar_image_set_data(
                image,
                data.as_ptr().cast(),
                data.len() as c_ulong,
                Some(keep_borrowed_data),
            );
        }

        ZBarImage {
            image,
            data: PhantomData,
        }
    }

    /// Returns the width of the image in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        unsafe { ffi::zbar_image_get_width(self.image) }
    }

    /// Returns the height of the image in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        unsafe { ffi::zbar_image_get_height(self.image) }
    }

    /// Returns the fourcc format code of the image.
    #[inline]
    pub fn format(&self) -> u32 {
        unsafe { ffi::zbar_image_get_format(self.image) as u32 }
    }

    /// Returns the sample data of the image.
    #[inline]
    pub fn data(&self) -> &[u8] {
        unsafe {
            let data = ffi::zbar_image_get_data(self.image);

            if data.is_null() {
                &[]
            } else {
                slice::from_raw_parts(
                    data.cast::<u8>(),
                    ffi::zbar_image_get_data_length(self.image) as usize,
                )
            }
        }
    }

    /// Returns the crop rectangle as `(x, y, width, height)`.
    #[inline]
    pub fn crop(&self) -> (u32, u32, u32, u32) {
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;

        unsafe {
            ffi::zbar_image_get_crop(self.image, &mut x, &mut y, &mut width, &mut height);
        }

        (x, y, width, height)
    }

    /// Restricts scanning to a rectangle of the image.
    /// The rectangle is clamped to the image.
    #[inline]
    pub fn set_crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let image_width = self.width();
        let image_height = self.height();
        let x = x.min(image_width);
        let y = y.min(image_height);
        let width = width.min(image_width - x);
        let height = height.min(image_height - y);

        unsafe {
            ffi::zbar_image_set_crop(self.image, x, y, width, height);
        }
    }

    /// Converts the image to another format.
    ///
    /// The result may share the sample data of this image, so it borrows for the same lifetime.
    #[inline]
    pub fn convert(&self, format: u32) -> Result<ZBarImage<'a>, ZBarError> {
        let image = unsafe { ffi::zbar_image_convert(self.image, c_ulong::from(format)) };

        Self::from_converted(image)
    }

    /// Converts the image to another format and crops or pads it to the requested size.
    ///
    /// Extra rows and columns are dropped from the bottom and right, or filled by repeating the last row and column.
    /// The image is not scaled.
    ///
    /// A nonempty grayscale image cannot be made from an empty source image.
    ///
    /// The result may share the sample data of this image, so it borrows for the same lifetime.
    #[inline]
    pub fn convert_resize(
        &self,
        format: u32,
        width: u32,
        height: u32,
    ) -> Result<ZBarImage<'a>, ZBarError> {
        if matches!(self.format(), FOURCC_Y800 | FOURCC_GREY)
            && matches!(format, FOURCC_Y800 | FOURCC_GREY)
            && (width != self.width() || height != self.height())
        {
            // ZBar 0.23.93 writes through a null pointer when resizing between grayscale formats.
            return self.resize_grayscale(format, width, height);
        }

        let image = unsafe {
            ffi::zbar_image_convert_resize(self.image, c_ulong::from(format), width, height)
        };

        Self::from_converted(image)
    }

    fn resize_grayscale(
        &self,
        format: u32,
        width: u32,
        height: u32,
    ) -> Result<ZBarImage<'a>, ZBarError> {
        let len = (width as usize)
            .checked_mul(height as usize)
            .filter(|&len| len <= c_ulong::MAX as usize)
            .ok_or(ZBarError::ImageConversionFailed)?;
        let source_width = self.width() as usize;
        let source_height = self.height() as usize;

        if len != 0 && (source_width == 0 || source_height == 0) {
            return Err(ZBarError::ImageConversionFailed);
        }

        let mut data = Vec::new();
        data.try_reserve_exact(len).map_err(|_| ZBarError::ImageConversionFailed)?;

        if len != 0 {
            let width = width as usize;
            let copy_width = width.min(source_width);
            let copy_height = (height as usize).min(source_height);

            for row in self.data().chunks_exact(source_width).take(copy_height) {
                data.extend_from_slice(&row[..copy_width]);
                data.resize(data.len() + width - copy_width, row[copy_width - 1]);
            }

            let last_row = data.len() - width..data.len();

            while data.len() < len {
                data.extend_from_within(last_row.clone());
            }
        }

        let image = unsafe { ffi::zbar_image_create() };

        if image.is_null() {
            return Err(ZBarError::ImageConversionFailed);
        }

        let data = Box::into_raw(data.into_boxed_slice()).cast::<u8>();

        unsafe {
            ffi::zbar_image_set_format(image, c_ulong::from(format));
            ffi::zbar_image_set_size(image, width, height);
            ffi::zbar_image_set_data(image, data.cast(), len as c_ulong, Some(free_owned_data));
        }

        let mut image = ZBarImage {
            image,
            data: PhantomData,
        };
        let (x, y, width, height) = self.crop();
        image.set_crop(x, y, width, height);

        Ok(image)
    }

    #[inline]
    fn from_converted(image: *mut ffi::zbar_image_t) -> Result<ZBarImage<'a>, ZBarError> {
        if image.is_null() {
            Err(ZBarError::ImageConversionFailed)
        } else {
            Ok(ZBarImage {
                image,
                data: PhantomData,
            })
        }
    }
}

impl Drop for ZBarImage<'_> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::zbar_image_destroy(self.image);
        }
    }
}

/// One symbol that a scanner decoded out of an image.
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct ZBarImageScanResult {
    /// The symbology of the symbol.
    pub symbol_type: ZBarSymbolType,
    /// The decoded data, which may contain any byte including NUL.
    pub data:        Vec<u8>,
    /// The corners of the symbol, in image coordinates.
    pub points:      Vec<(i32, i32)>,
    /// A relative quality metric. Larger values mean a more confident decode.
    pub quality:     i32,
    /// The coarse orientation of the symbol.
    pub orientation: ZBarOrientation,
}

/// A scanner that reads symbols out of images.
pub struct ZBarImageScanner {
    scanner: *mut ffi::zbar_image_scanner_t,
}

// A `ZBarImageScanner` owns its handle, and ZBar keeps no global mutable state for scanners.
unsafe impl Send for ZBarImageScanner {}

impl ZBarImageScanner {
    /// Creates a scanner with the default configuration.
    ///
    /// # Panics
    ///
    /// Panics when ZBar cannot allocate the scanner.
    #[inline]
    pub fn new() -> ZBarImageScanner {
        let scanner = unsafe { ffi::zbar_image_scanner_create() };

        assert!(!scanner.is_null(), "ZBar cannot allocate an image scanner");

        ZBarImageScanner {
            scanner,
        }
    }

    /// Sets one configuration value for a symbology, or for every symbology when `symbology` is [`ZBarSymbolType::ZBarNone`].
    #[inline]
    pub fn set_config(
        &mut self,
        symbology: ZBarSymbolType,
        config: ZBarConfig,
        value: i32,
    ) -> Result<(), ZBarError> {
        if matches!(
            symbology,
            ZBarSymbolType::ZBarSymbol
                | ZBarSymbolType::ZBarAddOn2
                | ZBarSymbolType::ZBarAddOn5
                | ZBarSymbolType::ZBarAddOn
        ) {
            return Err(ZBarError::InvalidConfig);
        }

        let result = unsafe {
            ffi::zbar_image_scanner_set_config(
                self.scanner,
                symbology.ordinal(),
                config.ordinal(),
                value,
            )
        };

        if result == 0 { Ok(()) } else { Err(ZBarError::InvalidConfig) }
    }

    /// Reads back one configuration value.
    #[inline]
    pub fn get_config(
        &mut self,
        symbology: ZBarSymbolType,
        config: ZBarConfig,
    ) -> Result<i32, ZBarError> {
        let mut value = 0;

        let result = unsafe {
            ffi::zbar_image_scanner_get_config(
                self.scanner,
                symbology.ordinal(),
                config.ordinal(),
                &mut value,
            )
        };

        if result == 0 { Ok(value) } else { Err(ZBarError::InvalidConfig) }
    }

    /// Applies a configuration string of the form `[symbology.]config[=value]`, such as `qrcode.enable=1`.
    pub fn parse_config(&mut self, config_string: &str) -> Result<(), ZBarError> {
        let config_string = CString::new(config_string).map_err(|_| ZBarError::InvalidConfig)?;

        let mut symbology = 0;
        let mut config = 0;
        let mut value = 0;

        // This mirrors the `zbar_image_scanner_parse_config` inline function in `zbar.h`, which is not an exported symbol.
        let result = unsafe {
            ffi::zbar_parse_config(config_string.as_ptr(), &mut symbology, &mut config, &mut value)
        };

        if result != 0 {
            return Err(ZBarError::InvalidConfig);
        }

        let result =
            unsafe { ffi::zbar_image_scanner_set_config(self.scanner, symbology, config, value) };

        if result == 0 { Ok(()) } else { Err(ZBarError::InvalidConfig) }
    }

    /// Turns the inter-image result cache on or off, and clears it either way.
    ///
    /// The cache filters duplicate results out of consecutive images, which is what you want when scanning video frames.
    #[inline]
    pub fn enable_cache(&mut self, enable: bool) {
        unsafe {
            ffi::zbar_image_scanner_enable_cache(self.scanner, c_int::from(enable));
        }
    }

    /// Scans an 8-bit grayscale image in the `Y800` format, one byte per pixel.
    ///
    /// It fails when `data` holds fewer than `width * height` bytes.
    #[inline]
    pub fn scan_y800<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
    ) -> Result<Vec<ZBarImageScanResult>, ZBarError> {
        let mut image = ZBarImage::new_y800(data.as_ref(), width, height)?;

        self.scan_image(&mut image)
    }

    /// Scans an 8-bit grayscale image in the `GREY` format, which has the same layout as `Y800`.
    ///
    /// It fails when `data` holds fewer than `width * height` bytes.
    ///
    /// Prefer the `Y800` format: ZBar decodes both, but its SQ Code decoder writes `Unexpected image format` to stderr for `GREY`.
    #[inline]
    pub fn scan_grey<D: AsRef<[u8]>>(
        &mut self,
        data: D,
        width: u32,
        height: u32,
    ) -> Result<Vec<ZBarImageScanResult>, ZBarError> {
        let mut image = ZBarImage::new_grey(data.as_ref(), width, height)?;

        self.scan_image(&mut image)
    }

    /// Scans an image that is already in the `Y800` or `GREY` format.
    ///
    /// Use [`ZBarImage::convert`] first for anything else, because ZBar only scans those two formats.
    pub fn scan_image(
        &mut self,
        image: &mut ZBarImage<'_>,
    ) -> Result<Vec<ZBarImageScanResult>, ZBarError> {
        let n = unsafe { ffi::zbar_scan_image(self.scanner, image.image) };

        if n < 0 {
            return Err(ZBarError::UnsupportedImageFormat(image.format()));
        }

        let symbols = unsafe { ffi::zbar_image_get_symbols(image.image) };

        let mut results = if symbols.is_null() {
            Vec::new()
        } else {
            let size = unsafe { ffi::zbar_symbol_set_get_size(symbols) };

            Vec::with_capacity(size.max(0) as usize)
        };

        let mut symbol = unsafe { ffi::zbar_image_first_symbol(image.image) };

        while !symbol.is_null() {
            let raw_symbol_type = unsafe { ffi::zbar_symbol_get_type(symbol) };
            let symbol_type = ZBarSymbolType::from_ordinal(raw_symbol_type)
                .ok_or(ZBarError::UnknownSymbolType(raw_symbol_type))?;

            let data = unsafe {
                let data = ffi::zbar_symbol_get_data(symbol);

                if data.is_null() {
                    Vec::new()
                } else {
                    let data_length = ffi::zbar_symbol_get_data_length(symbol) as usize;

                    slice::from_raw_parts(data.cast::<u8>(), data_length).to_vec()
                }
            };

            // extract bounding box
            let loc_size = unsafe { ffi::zbar_symbol_get_loc_size(symbol) };

            let mut points = Vec::with_capacity(loc_size as usize);

            for i in 0..loc_size {
                let x = unsafe { ffi::zbar_symbol_get_loc_x(symbol, i) };
                let y = unsafe { ffi::zbar_symbol_get_loc_y(symbol, i) };

                points.push((x, y));
            }

            let quality = unsafe { ffi::zbar_symbol_get_quality(symbol) };

            let orientation = unsafe { ffi::zbar_symbol_get_orientation(symbol) };
            let orientation = ZBarOrientation::from_ordinal(orientation)
                .unwrap_or(ZBarOrientation::ZBarOrientUnknown);

            results.push(ZBarImageScanResult {
                symbol_type,
                data,
                points,
                quality,
                orientation,
            });

            symbol = unsafe { ffi::zbar_symbol_next(symbol) };
        }

        Ok(results)
    }
}

impl Default for ZBarImageScanner {
    #[inline]
    fn default() -> Self {
        ZBarImageScanner::new()
    }
}

impl Drop for ZBarImageScanner {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::zbar_image_scanner_destroy(self.scanner);
        }
    }
}
