#[test]
fn version() {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;
    let result = unsafe { zbar_rust::zbar_version(&mut major, &mut minor, &mut patch) };

    assert_eq!(0, result);

    assert!(major == 0 && minor >= 10);
}
