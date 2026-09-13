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

    image.set_crop(1, 1, u32::MAX, u32::MAX);

    assert_eq!((1, 1, 7, 7), image.crop());
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

#[test]
fn convert_resize() {
    let data = [1, 2, 3, 4, 5, 6];

    for source_format in [FOURCC_Y800, FOURCC_GREY] {
        let image = match source_format {
            FOURCC_Y800 => ZBarImage::new_y800(&data, 3, 2).unwrap(),
            _ => ZBarImage::new_grey(&data, 3, 2).unwrap(),
        };

        for format in [FOURCC_Y800, FOURCC_GREY] {
            let smaller = image.convert_resize(format, 2, 1).unwrap();

            assert_eq!(format, smaller.format());
            assert_eq!(2, smaller.width());
            assert_eq!(1, smaller.height());
            assert_eq!([1, 2], smaller.data());
            assert_eq!((0, 0, 2, 1), smaller.crop());

            let larger = image.convert_resize(format, 4, 3).unwrap();

            assert_eq!(format, larger.format());
            assert_eq!(4, larger.width());
            assert_eq!(3, larger.height());
            assert_eq!([1, 2, 3, 3, 4, 5, 6, 6, 4, 5, 6, 6], larger.data());
            assert_eq!((0, 0, 3, 2), larger.crop());
        }
    }
}

#[test]
fn converted_resize_keeps_data() {
    let data = [1, 2, 3, 4, 5, 6];
    let converted = {
        let image = ZBarImage::new_y800(&data, 3, 2).unwrap();
        let resized = image.convert_resize(FOURCC_Y800, 4, 3).unwrap();

        // ZBar must keep the owned buffer alive after the source handles are dropped.
        resized.convert(FOURCC_GREY).unwrap()
    };

    assert_eq!([1, 2, 3, 3, 4, 5, 6, 6, 4, 5, 6, 6], converted.data());
}
