use super::check;
use super::guard_policy_support::*;
use crate::cli::logic::args::Options;
use crate::contracts::{Capability, CommandPlan, SupportState};

fn installer(support: SupportState) -> crate::contracts::CapabilityPlan {
    let mut plan = dummy_plan(support, "2026-11-15T00:00:00Z", vec![]);
    plan.capability = Capability::Download;
    plan.command = CommandPlan::new(
        "sh".into(),
        vec![
            "-c".into(),
            "curl -fsSL https://vendor.example/install.sh | bash".into(),
        ],
    );
    plan
}

fn with(dry_run: bool, confirm: Option<&str>) -> Options {
    Options {
        dry_run,
        confirm: confirm.map(str::to_string),
        ..Options::default()
    }
}

#[test]
fn a_headless_unknown_installer_names_its_host_and_the_consent_token() {
    let err = check(
        &dummy_harness(),
        &installer(SupportState::Unknown),
        &with(false, None),
        false,
    )
    .unwrap_err();
    assert_eq!(err.exit_code, 5, "an unconsented script is a safety stop");
    assert_eq!(err.code, "capability_unknown");
    assert!(err.message.contains("vendor.example"), "{}", err.message);
    assert!(
        err.next_action.contains("terminal-jarvis install test"),
        "{}",
        err.next_action
    );
    assert!(
        err.next_action
            .contains("--no-input --confirm=download:test"),
        "{}",
        err.next_action
    );
}

#[test]
fn a_bound_confirm_consents_to_an_unknown_claim_headlessly() {
    let plan = installer(SupportState::Unknown);
    let bound = with(false, Some("download:test"));
    assert!(check(&dummy_harness(), &plan, &bound, false).is_ok());
}

#[test]
fn a_preview_of_an_unknown_installer_needs_no_consent() {
    let plan = installer(SupportState::Unknown);
    assert!(check(&dummy_harness(), &plan, &with(true, None), false).is_ok());
}

#[test]
fn a_preview_never_unlocks_an_unclaimed_capability() {
    for (state, code) in [
        (SupportState::Stub, "capability_stub"),
        (SupportState::Manual, "manual_procedure_required"),
        (SupportState::Unsupported, "capability_unsupported"),
        (SupportState::Disabled, "capability_disabled"),
    ] {
        let preview = with(true, Some("download:test"));
        let err = check(&dummy_harness(), &installer(state), &preview, true).unwrap_err();
        assert_eq!(err.code, code);
        assert_eq!(
            err.exit_code, 4,
            "{code} is unavailable, not a consent question"
        );
    }
}

#[test]
fn a_preview_never_unlocks_an_incompatible_platform() {
    let mut plan = installer(SupportState::Unknown);
    plan.platforms = vec!["no-such-platform".into()];
    let err = check(&dummy_harness(), &plan, &with(true, None), false).unwrap_err();
    assert!(err.code.starts_with("platform_"), "{}", err.code);
}
