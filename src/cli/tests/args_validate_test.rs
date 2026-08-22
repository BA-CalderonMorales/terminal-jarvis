use super::*;

#[test]
fn gate_pair_requires_enable_or_run() {
    assert!(valid_gate(&[]).is_ok());
    assert!(valid_gate(&["enable".into(), "opencode".into()]).is_ok());
    assert!(valid_gate(&["run".into(), "opencode".into()]).is_ok());
    assert!(valid_gate(&["status".into(), "opencode".into()]).is_err());
}

#[test]
fn gate_accepts_no_input_as_a_no_op() {
    let options = Options {
        no_input: true,
        ..Default::default()
    };
    assert!(validate_options(&Action::Gate(vec!["enable".into()]), &options).is_ok());
    assert!(validate_options(&Action::Gate(vec!["run".into(), "trivy".into()]), &options).is_ok());
    assert!(validate_options(&Action::List, &options).is_err());
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

#[test]
fn bare_update_summary_previews_under_dry_run() {
    let options = Options {
        dry_run: true,
        ..Default::default()
    };
    assert!(validate_options(&Action::Update(None), &options).is_ok());
}

#[test]
fn bare_update_summary_still_rejects_effectful_lifecycle_flags() {
    let options = Options {
        allow_dangerous: true,
        ..Default::default()
    };
    assert!(validate_options(&Action::Update(None), &options).is_err());
    let options = Options {
        confirm: Some("update:codex".to_string()),
        ..Default::default()
    };
    assert!(validate_options(&Action::Update(None), &options).is_err());
}

#[test]
fn security_usage_error_names_the_valid_choices() {
    let error = at_most_one(
        &["a".into(), "b".into()],
        "security [status|audit|<harness>]",
    )
    .expect_err("two words must be rejected");
    assert_eq!(
        error,
        "usage: terminal-jarvis security [status|audit|<harness>]"
    );
}
