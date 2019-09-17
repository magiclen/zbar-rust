extern crate zbar_rust;

use zbar_rust::ZBarImage;

#[test]
fn image_create_destroy() {
    let _zbar = ZBarImage::new();
}
