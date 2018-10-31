extern crate zbar_rust;
extern crate qrcode_generator;
extern crate png;

use std::io::Cursor;

use zbar_rust::{ZBarConfig, ZBarSymbolType, ZBarImageScanner};

use qrcode_generator::QrCodeEcc;

#[test]
fn decode_qrcode() {
    let mut scanner = ZBarImageScanner::new();
    scanner.set_config(ZBarSymbolType::ZBarNone, ZBarConfig::ZBarCfgEnable, 0).unwrap();
    scanner.set_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgEnable, 1).unwrap();

    let url = "https://magiclen.org";

    let size = 512;

    let png = qrcode_generator::to_png_to_vec(url, QrCodeEcc::Low, size).unwrap();

    let decoder = png::Decoder::new(Cursor::new(png));

    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf: Vec<u8> = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let mut result = scanner.scan_y800(Cursor::new(buf), size as u32, size as u32).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(ZBarSymbolType::ZBarQRCode, result[0].symbol_type);
    assert_eq!(url, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}