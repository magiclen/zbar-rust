use zbar_rust::{FOURCC_GREY, FOURCC_Y800, ZBarImage};

#[test]
fn image_create_destroy() {
    let data = vec![0u8; 4 * 3];

    let image = ZBarImage::new_y800(&data, 4, 3).unwrap();

    assert_eq!(4, image.width());
    assert_eq!(3, image.height());
    assert_eq!(FOURCC_Y800, image.format());
    assert_eq!(data.as_slice(), image.data());
}

#[test]
fn crop() {
    let data = vec![0u8; 8 * 8];

    let mut image = ZBarImage::new_grey(&data, 8, 8).unwrap();

    assert_eq!((0, 0, 8, 8), image.crop());

    image.set_crop(2, 2, 4, 4);

    assert_eq!((2, 2, 4, 4), image.crop());
}

#[test]
fn convert() {
    let data = vec![0u8; 4 * 3];

    let image = ZBarImage::new_y800(&data, 4, 3).unwrap();

    let converted = image.convert(FOURCC_GREY).unwrap();

    assert_eq!(FOURCC_GREY, converted.format());
    assert_eq!(4, converted.width());
    assert_eq!(3, converted.height());
}

#[test]
fn insufficient_data() {
    let data = [0u8; 4];

    assert!(ZBarImage::new_y800(&data, 1000, 1000).is_err());
}
