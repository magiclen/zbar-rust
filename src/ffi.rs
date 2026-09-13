//! Raw bindings to the ZBar C library.
//!
//! Every item here mirrors a declaration in `zbar.h`. Calling any of these functions is unsafe because ZBar does not validate its arguments.
//!
//! The handle types are opaque, so the compiler stops you from passing, say, a scanner where an image is expected. Use [`crate::ZBarImage`] and [`crate::ZBarImageScanner`] instead unless you need something these bindings expose and the safe API does not.

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/// The opaque struct behind [`zbar_image_t`].
#[repr(C)]
pub struct zbar_image_s {
    _private: [u8; 0],
}

/// An image, holding sample data plus its format and size.
pub type zbar_image_t = zbar_image_s;

/// The opaque struct behind [`zbar_symbol_t`].
#[repr(C)]
pub struct zbar_symbol_s {
    _private: [u8; 0],
}

/// One decoded symbol. Owned by the image or scanner that produced it.
pub type zbar_symbol_t = zbar_symbol_s;

/// The opaque struct behind [`zbar_symbol_set_t`].
#[repr(C)]
pub struct zbar_symbol_set_s {
    _private: [u8; 0],
}

/// A container of decoded symbols.
pub type zbar_symbol_set_t = zbar_symbol_set_s;

/// The opaque struct behind [`zbar_image_scanner_t`].
#[repr(C)]
pub struct zbar_image_scanner_s {
    _private: [u8; 0],
}

/// A scanner that reads symbols out of images.
pub type zbar_image_scanner_t = zbar_image_scanner_s;

/// Called to free the sample data of an image when the image is destroyed.
pub type zbar_image_cleanup_handler_t = unsafe extern "C" fn(image: *mut zbar_image_t);

/// Called when decoded results become available for an image.
pub type zbar_image_data_handler_t =
    unsafe extern "C" fn(image: *mut zbar_image_t, userdata: *const c_void);

// ----- General interface -----

unsafe extern "C" {
    /// Writes the ZBar version into `major`, `minor` and `patch`. Returns 0 on success.
    pub fn zbar_version(major: *mut c_uint, minor: *mut c_uint, patch: *mut c_uint) -> c_int;

    /// Sets how much diagnostic output ZBar writes to stderr. 0 disables it.
    pub fn zbar_set_verbosity(verbosity: c_int);

    /// Raises the verbosity level by one.
    pub fn zbar_increase_verbosity();

    /// Returns the static name of a symbol type, or `"UNKNOWN"`.
    pub fn zbar_get_symbol_name(sym: c_int) -> *const c_char;

    /// Returns the static name of an add-on, or an empty string. Deprecated since ZBar 0.11.
    pub fn zbar_get_addon_name(sym: c_int) -> *const c_char;

    /// Returns the static name of a configuration setting.
    pub fn zbar_get_config_name(config: c_int) -> *const c_char;

    /// Returns the static name of a modifier.
    pub fn zbar_get_modifier_name(modifier: c_int) -> *const c_char;

    /// Returns the static name of an orientation.
    pub fn zbar_get_orientation_name(orientation: c_int) -> *const c_char;

    /// Parses a config string of the form `[symbology.]config[=value]`. Returns 0 on success.
    pub fn zbar_parse_config(
        config_string: *const c_char,
        symbology: *mut c_int,
        config: *mut c_int,
        value: *mut c_int,
    ) -> c_int;
}

// ----- Image interface -----

unsafe extern "C" {
    /// Creates an image with uninitialized data and format. Returns null when out of memory.
    pub fn zbar_image_create() -> *mut zbar_image_t;

    /// Releases one reference to an image, destroying it when the last one goes away.
    pub fn zbar_image_destroy(image: *mut zbar_image_t);

    /// Adjusts the reference count of an image by `refs`.
    pub fn zbar_image_ref(image: *mut zbar_image_t, refs: c_int);

    /// Converts an image to a new format. Returns null when the conversion is not supported.
    pub fn zbar_image_convert(image: *const zbar_image_t, format: c_ulong) -> *mut zbar_image_t;

    /// Converts and rescales an image in one step. Returns null when the conversion is not supported.
    pub fn zbar_image_convert_resize(
        image: *const zbar_image_t,
        format: c_ulong,
        width: c_uint,
        height: c_uint,
    ) -> *mut zbar_image_t;

    /// Returns the fourcc format code of the sample data.
    pub fn zbar_image_get_format(image: *const zbar_image_t) -> c_ulong;

    /// Returns the sequence number the caller assigned to the image.
    pub fn zbar_image_get_sequence(image: *const zbar_image_t) -> c_uint;

    /// Returns the width of the image in pixels.
    pub fn zbar_image_get_width(image: *const zbar_image_t) -> c_uint;

    /// Returns the height of the image in pixels.
    pub fn zbar_image_get_height(image: *const zbar_image_t) -> c_uint;

    /// Writes the width and height of the image into `width` and `height`.
    pub fn zbar_image_get_size(image: *const zbar_image_t, width: *mut c_uint, height: *mut c_uint);

    /// Writes the crop rectangle of the image into the output parameters.
    pub fn zbar_image_get_crop(
        image: *const zbar_image_t,
        x: *mut c_uint,
        y: *mut c_uint,
        width: *mut c_uint,
        height: *mut c_uint,
    );

    /// Returns a pointer to the sample data of the image.
    pub fn zbar_image_get_data(image: *const zbar_image_t) -> *const c_void;

    /// Returns the length of the sample data in bytes.
    pub fn zbar_image_get_data_length(image: *const zbar_image_t) -> c_ulong;

    /// Returns the decoded symbols of the image, or null when it has not been scanned.
    pub fn zbar_image_get_symbols(image: *const zbar_image_t) -> *const zbar_symbol_set_t;

    /// Replaces the decoded symbols of the image.
    pub fn zbar_image_set_symbols(image: *mut zbar_image_t, symbols: *const zbar_symbol_set_t);

    /// Returns the first decoded symbol of the image, or null when there is none.
    pub fn zbar_image_first_symbol(image: *const zbar_image_t) -> *const zbar_symbol_t;

    /// Sets the fourcc format code of the sample data.
    pub fn zbar_image_set_format(image: *mut zbar_image_t, format: c_ulong);

    /// Sets a caller-defined sequence number, which also identifies the image in results.
    pub fn zbar_image_set_sequence(image: *mut zbar_image_t, sequence_num: c_uint);

    /// Sets the size of the image and resets its crop rectangle to the whole image.
    pub fn zbar_image_set_size(image: *mut zbar_image_t, width: c_uint, height: c_uint);

    /// Restricts scanning to a rectangle of the image. The rectangle is clamped to the image.
    pub fn zbar_image_set_crop(
        image: *mut zbar_image_t,
        x: c_uint,
        y: c_uint,
        width: c_uint,
        height: c_uint,
    );

    /// Attaches sample data to an image. `cleanup_handler` is called when the data is released.
    pub fn zbar_image_set_data(
        image: *mut zbar_image_t,
        data: *const c_void,
        data_byte_length: c_ulong,
        cleanup_handler: Option<zbar_image_cleanup_handler_t>,
    );

    /// Releases the sample data of an image through its cleanup handler.
    pub fn zbar_image_free_data(image: *mut zbar_image_t);

    /// Associates a caller-defined value with the image.
    pub fn zbar_image_set_userdata(image: *mut zbar_image_t, userdata: *mut c_void);

    /// Returns the value previously associated with the image.
    pub fn zbar_image_get_userdata(image: *const zbar_image_t) -> *mut c_void;

    /// Writes the image to `<filebase>.<fourcc>.zimg`. Returns 0 on success.
    pub fn zbar_image_write(image: *const zbar_image_t, filebase: *const c_char) -> c_int;
}

// ----- Symbol interface -----

unsafe extern "C" {
    /// Adjusts the reference count of a symbol by `refs`.
    pub fn zbar_symbol_ref(symbol: *const zbar_symbol_t, refs: c_int);

    /// Returns the type of the decoded symbol.
    pub fn zbar_symbol_get_type(symbol: *const zbar_symbol_t) -> c_int;

    /// Returns the bitmask of configuration settings that applied to this symbol.
    pub fn zbar_symbol_get_configs(symbol: *const zbar_symbol_t) -> c_uint;

    /// Returns the bitmask of modifiers that applied to this symbol.
    pub fn zbar_symbol_get_modifiers(symbol: *const zbar_symbol_t) -> c_uint;

    /// Returns the decoded data. It is NUL-terminated but may also contain embedded NULs.
    pub fn zbar_symbol_get_data(symbol: *const zbar_symbol_t) -> *const c_char;

    /// Returns the length of the decoded data in bytes, not counting the trailing NUL.
    pub fn zbar_symbol_get_data_length(symbol: *const zbar_symbol_t) -> c_uint;

    /// Returns a relative quality metric. Larger values are better.
    pub fn zbar_symbol_get_quality(symbol: *const zbar_symbol_t) -> c_int;

    /// Returns how many times this symbol was seen. Only meaningful when the cache is enabled.
    pub fn zbar_symbol_get_count(symbol: *const zbar_symbol_t) -> c_int;

    /// Returns the number of points in the location polygon of the symbol.
    pub fn zbar_symbol_get_loc_size(symbol: *const zbar_symbol_t) -> c_uint;

    /// Returns the x coordinate of one point of the location polygon.
    pub fn zbar_symbol_get_loc_x(symbol: *const zbar_symbol_t, index: c_uint) -> c_int;

    /// Returns the y coordinate of one point of the location polygon.
    pub fn zbar_symbol_get_loc_y(symbol: *const zbar_symbol_t, index: c_uint) -> c_int;

    /// Returns a coarse, axis-aligned orientation of the symbol.
    pub fn zbar_symbol_get_orientation(symbol: *const zbar_symbol_t) -> c_int;

    /// Returns the next symbol of the set this symbol belongs to, or null at the end.
    pub fn zbar_symbol_next(symbol: *const zbar_symbol_t) -> *const zbar_symbol_t;

    /// Returns the components of a composite result, or null for a physical symbol.
    pub fn zbar_symbol_get_components(symbol: *const zbar_symbol_t) -> *const zbar_symbol_set_t;

    /// Returns the first component of a composite result, or null for a physical symbol.
    pub fn zbar_symbol_first_component(symbol: *const zbar_symbol_t) -> *const zbar_symbol_t;

    /// Prints an XML representation of the symbol into a caller-owned buffer, reallocating it if needed.
    pub fn zbar_symbol_xml(
        symbol: *const zbar_symbol_t,
        buffer: *mut *mut c_char,
        buflen: *mut c_uint,
    ) -> *mut c_char;
}

// ----- Symbol set interface -----

unsafe extern "C" {
    /// Adjusts the reference count of a symbol set by `refs`.
    pub fn zbar_symbol_set_ref(symbols: *const zbar_symbol_set_t, refs: c_int);

    /// Returns how many symbols the set contains.
    pub fn zbar_symbol_set_get_size(symbols: *const zbar_symbol_set_t) -> c_int;

    /// Returns the first symbol of the set, or null when the set is empty.
    pub fn zbar_symbol_set_first_symbol(symbols: *const zbar_symbol_set_t) -> *const zbar_symbol_t;

    /// Returns the first symbol of the set before filtering, or null when the set is empty.
    pub fn zbar_symbol_set_first_unfiltered(
        symbols: *const zbar_symbol_set_t,
    ) -> *const zbar_symbol_t;
}

// ----- Image scanner interface -----

unsafe extern "C" {
    /// Creates an image scanner. Returns null when out of memory.
    pub fn zbar_image_scanner_create() -> *mut zbar_image_scanner_t;

    /// Destroys an image scanner and everything it owns.
    pub fn zbar_image_scanner_destroy(scanner: *mut zbar_image_scanner_t);

    /// Installs a result callback and returns the previous one. Pass `None` to disable callbacks.
    pub fn zbar_image_scanner_set_data_handler(
        scanner: *mut zbar_image_scanner_t,
        handler: Option<zbar_image_data_handler_t>,
        userdata: *const c_void,
    ) -> Option<zbar_image_data_handler_t>;

    /// Sets one configuration value for a symbology, or for every symbology when it is 0. Returns 0 on success.
    pub fn zbar_image_scanner_set_config(
        scanner: *mut zbar_image_scanner_t,
        symbology: c_int,
        config: c_int,
        value: c_int,
    ) -> c_int;

    /// Reads one configuration value into `value`. Returns 0 on success.
    pub fn zbar_image_scanner_get_config(
        scanner: *mut zbar_image_scanner_t,
        symbology: c_int,
        config: c_int,
        value: *mut c_int,
    ) -> c_int;

    /// Turns the inter-image result cache on or off, and clears it either way.
    pub fn zbar_image_scanner_enable_cache(scanner: *mut zbar_image_scanner_t, enable: c_int);

    /// Drops the decoded results held by the scanner and by the image, keeping the memory for reuse.
    pub fn zbar_image_scanner_recycle_image(
        scanner: *mut zbar_image_scanner_t,
        image: *mut zbar_image_t,
    );

    /// Returns the results of the last scanned image, or null when there are none.
    pub fn zbar_image_scanner_get_results(
        scanner: *const zbar_image_scanner_t,
    ) -> *const zbar_symbol_set_t;

    /// Asks the scanner to publish decoded codes over D-Bus. Returns 0 on success.
    pub fn zbar_image_scanner_request_dbus(
        scanner: *mut zbar_image_scanner_t,
        req_dbus_enabled: c_int,
    ) -> c_int;

    /// Scans an image, which must be in the `Y800` or `GREY` format.
    ///
    /// Returns the number of newly decoded symbols, 0 when nothing was found, or -1 on error.
    pub fn zbar_scan_image(scanner: *mut zbar_image_scanner_t, image: *mut zbar_image_t) -> c_int;
}
