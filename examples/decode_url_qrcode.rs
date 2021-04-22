extern crate image;
extern crate zbar_rust;

use zbar_rust::ZBarImageScanner;

use image::GenericImageView;

#[cfg(windows)]
const INPUT_PATH: &str = r"examples\data\magiclen.org.png";

#[cfg(not(windows))]
const INPUT_PATH: &str = "examples/data/magiclen.org.png";

fn main() {
    let img = image::open(INPUT_PATH).unwrap();

    let (width, height) = img.dimensions();

    let luma_img = img.to_luma8();

    let luma_img_data: Vec<u8> = luma_img.to_vec();

    let mut scanner = ZBarImageScanner::new();

    let mut results = scanner.scan_y800(&luma_img_data, width, height).unwrap();

    assert_eq!(1, results.len());

    println!("{}", String::from_utf8(results.remove(0).data).unwrap());
}
