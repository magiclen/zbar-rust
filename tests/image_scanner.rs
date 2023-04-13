use qrcode_generator::QrCodeEcc;
use zbar_rust::{ZBarConfig, ZBarImageScanner, ZBarSymbolType};

#[test]
fn image_create_destroy() {
    let _scanner = ZBarImageScanner::new();
}

#[test]
fn set_config() {
    let mut scanner = ZBarImageScanner::new();
    scanner.set_config(ZBarSymbolType::ZBarNone, ZBarConfig::ZBarCfgEnable, 0).unwrap();
    scanner.set_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgEnable, 1).unwrap();
}

#[test]
fn decode_qrcode() {
    let mut scanner = ZBarImageScanner::new();

    let url = "https://magiclen.org";

    let size = 512;

    let data = qrcode_generator::to_image_from_str(url, QrCodeEcc::Low, size).unwrap();

    let mut result = scanner.scan_y800(&data, size as u32, size as u32).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(ZBarSymbolType::ZBarQRCode, result[0].symbol_type);
    assert_eq!(34, result[0].points.iter().map(|(x, _)| *x).min().unwrap()); // left
    assert_eq!(34, result[0].points.iter().map(|(_, y)| *y).min().unwrap()); // top
    assert_eq!(479, result[0].points.iter().map(|(x, _)| *x).max().unwrap()); // right
    assert_eq!(479, result[0].points.iter().map(|(_, y)| *y).max().unwrap()); // bottom
    assert_eq!(url, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}
