#[test]
fn version() {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;
    let result = unsafe { zbar_rust::ffi::zbar_version(&mut major, &mut minor, &mut patch) };

    assert_eq!(0, result);

    assert!((0, 22) <= (major, minor));
}

#[test]
fn version_wrapper() {
    let (major, minor, _patch) = zbar_rust::version();

    assert!((0, 22) <= (major, minor));
}
