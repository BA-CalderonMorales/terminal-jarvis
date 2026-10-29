use super::*;
#[test]
fn embedded_catalog_is_ready_without_filesystem_data_and_missing_overrides_are_not() {
    let dir = tempdir("embedded");
    let path = Path::new(crate::context::EMBEDDED_CATALOG);
    let (records, valid) = collected(path, &dir, &[harness()]);
    let catalog = record(&records, "state.catalog");
    assert!(valid);
    assert_eq!(catalog.code, Code::Ready);
    assert_eq!(catalog.value, "embedded");
    assert!(catalog.action.is_none());
    let (records, valid) = collected(&dir.join("missing"), &dir, &[harness()]);
    assert!(!valid);
    assert_eq!(record(&records, "state.catalog").code, Code::Missing);
    let (records, _) = collected(path, &dir, &[]);
    assert_eq!(record(&records, "state.catalog").code, Code::Empty);
    fs::remove_dir_all(dir).unwrap();
}
