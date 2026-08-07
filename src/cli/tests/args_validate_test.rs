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
    let mut options = Options::default();
    options.confirm = Some("install:all".to_string());
    assert!(validate_options(&Action::Run(vec![]), &options).is_ok());
    assert!(validate_options(&Action::List, &options).is_err());
    options.confirm = None;
    options.allow_dangerous = true;
    assert!(validate_options(&Action::Run(vec![]), &options).is_ok());
    assert!(validate_options(&Action::List, &options).is_err());
}
