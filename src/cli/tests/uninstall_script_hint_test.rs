use super::super::uninstall_plan;

#[test]
fn a_script_install_points_at_the_vendor_source() {
    let failure = uninstall_plan::unavailable("tool", Some("https://vendor.example/docs"));
    let text = format!("{failure:?}");
    assert!(text.contains("installed by a script") && text.contains("https://vendor.example/docs"));
    let bare = format!(
        "{:?}",
        uninstall_plan::unavailable("tool", Some("internal:x"))
    );
    assert!(!bare.contains("script"));
}
