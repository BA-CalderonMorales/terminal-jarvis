use super::*;

#[test]
fn gate_pair_requires_enable_or_run() {
    assert!(valid_gate(&[]).is_ok());
    assert!(valid_gate(&["enable".into(), "opencode".into()]).is_ok());
    assert!(valid_gate(&["run".into(), "opencode".into()]).is_ok());
    assert!(valid_gate(&["status".into(), "opencode".into()]).is_err());
}

#[test]
fn lifecycle_flags_only_on_lifecycle_actions() {
    let options = Options {
        confirm: Some("install:all".to_string()),
        ..Default::default()
    };
    assert!(validate_options(&Action::Run(vec![]), &options).is_ok());
    assert!(validate_options(&Action::List, &options).is_err());
    let options = Options {
        confirm: None,
        allow_dangerous: true,
        ..Default::default()
    };
    assert!(validate_options(&Action::Run(vec![]), &options).is_ok());
    assert!(validate_options(&Action::List, &options).is_err());
}
