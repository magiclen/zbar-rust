use qrcode_generator::{
    Renderer,
    qr::{Encoder, ErrorCorrection},
};
use zbar_rust::{
    ZBarConfig, ZBarError, ZBarImage, ZBarImageScanner, ZBarOrientation, ZBarSymbolType,
};

const URL: &str = "https://magiclen.org";

const SIZE: usize = 512;

fn qrcode_luma8() -> Vec<u8> {
    let symbol = Encoder::new(ErrorCorrection::Low).encode_text(URL).unwrap();

    Renderer::new(&symbol, SIZE).to_luma8().unwrap()
}

#[test]
fn scanner_create_destroy() {
    let _scanner = ZBarImageScanner::new();
}

#[test]
fn set_config() {
    let mut scanner = ZBarImageScanner::new();
    scanner.set_config(ZBarSymbolType::ZBarNone, ZBarConfig::ZBarCfgEnable, 0).unwrap();
    scanner.set_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgEnable, 1).unwrap();
    scanner.set_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgUncertainty, 2).unwrap();

    for symbology in [
        ZBarSymbolType::ZBarSymbol,
        ZBarSymbolType::ZBarAddOn2,
        ZBarSymbolType::ZBarAddOn5,
        ZBarSymbolType::ZBarAddOn,
    ] {
        assert_eq!(
            Err(ZBarError::InvalidConfig),
            scanner.set_config(symbology, ZBarConfig::ZBarCfgUncertainty, 2)
        );
    }
}

#[test]
fn get_config() {
    let mut scanner = ZBarImageScanner::new();
    scanner.set_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgEnable, 1).unwrap();

    assert_eq!(
        1,
        scanner.get_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgEnable).unwrap()
    );
}

#[test]
fn parse_config() {
    let mut scanner = ZBarImageScanner::new();
    scanner.parse_config("qrcode.enable=1").unwrap();
}

#[test]
fn decode_qrcode() {
    let mut scanner = ZBarImageScanner::new();

    let data = qrcode_luma8();

    let mut result = scanner.scan_y800(&data, SIZE as u32, SIZE as u32).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(ZBarSymbolType::ZBarQRCode, result[0].symbol_type);
    assert_eq!("QR-Code", result[0].symbol_type.name());
    assert_eq!(ZBarOrientation::ZBarOrientUp, result[0].orientation);
    assert_eq!(71, result[0].points.iter().map(|(x, _)| *x).min().unwrap()); // left
    assert_eq!(71, result[0].points.iter().map(|(_, y)| *y).min().unwrap()); // top
    assert_eq!(441, result[0].points.iter().map(|(x, _)| *x).max().unwrap()); // right
    assert_eq!(441, result[0].points.iter().map(|(_, y)| *y).max().unwrap()); // bottom
    assert_eq!(URL, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}

#[test]
fn decode_qrcode_grey() {
    let mut scanner = ZBarImageScanner::new();

    let data = qrcode_luma8();

    let mut result = scanner.scan_grey(&data, SIZE as u32, SIZE as u32).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(ZBarSymbolType::ZBarQRCode, result[0].symbol_type);
    assert_eq!(URL, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}

#[test]
fn decode_qrcode_image() {
    let mut scanner = ZBarImageScanner::new();

    let data = qrcode_luma8();

    let mut image = ZBarImage::new_y800(&data, SIZE as u32, SIZE as u32).unwrap();

    let mut result = scanner.scan_image(&mut image).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(URL, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}

#[test]
fn insufficient_data() {
    let mut scanner = ZBarImageScanner::new();

    assert!(scanner.scan_y800([0u8; 4], 1000, 1000).is_err());
}
