use image::GenericImageView;
use zbar_rust::ZBarImageScanner;

#[cfg(windows)]
const INPUT_PATH: &str = r"examples\data\magiclen.org.png";

#[cfg(not(windows))]
const INPUT_PATH: &str = "examples/data/magiclen.org.png";

fn main() {
    let img = image::open(INPUT_PATH).unwrap();

    let (width, height) = img.dimensions();

    let mut scanner = ZBarImageScanner::new();

    let mut results = scanner.scan_y800(img.into_luma8().into_raw(), width, height).unwrap();

    assert_eq!(1, results.len());

    println!("{}", String::from_utf8(results.remove(0).data).unwrap());
}
