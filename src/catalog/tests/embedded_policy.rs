use super::*;

fn embedded_sentinel() -> &'static Path {
    Path::new(crate::context::EMBEDDED_CATALOG)
}

#[test]
fn sentinel_root_always_uses_embedded_data() {
    assert!(embedded_for(embedded_sentinel(), false));
    assert!(embedded_for(embedded_sentinel(), true));
}

#[test]
fn absent_dev_relative_root_falls_back_to_embedded_data() {
    assert!(embedded_for(Path::new("harnesses"), false));
}

#[test]
fn present_dev_relative_root_is_read_from_disk() {
    assert!(!embedded_for(Path::new("harnesses"), true));
}

#[test]
fn any_other_root_is_read_from_disk() {
    assert!(!embedded_for(Path::new("vendor/catalog"), false));
    assert!(!embedded_for(Path::new("vendor/catalog"), true));
}
