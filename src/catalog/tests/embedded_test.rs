use super::*;

#[test]
fn embedded_catalog_loads() {
    let harnesses = load().expect("embedded catalog present");
    assert!(!harnesses.is_empty());
}
