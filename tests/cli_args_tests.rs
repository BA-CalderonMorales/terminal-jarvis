use terminal_jarvis::cli::args::{parse, Action};
use terminal_jarvis::contracts::Capability;
#[rustfmt::skip]
#[test] fn empty_args_show_help() { assert_eq!(parse(["tj"]).unwrap(), Action::Help); }
#[rustfmt::skip]
#[test] fn parses_plan_with_explicit_harness() { assert_eq!(parse(["tj", "plan", "codex", "headless"]).unwrap(), Action::Plan { harness: Some("codex".to_string()), capability: Capability::Headless }); }
#[rustfmt::skip]
#[test] fn parses_plan_for_active_harness() { assert_eq!(parse(["tj", "plan", "models"]).unwrap(), Action::Plan { harness: None, capability: Capability::Models }); }
#[rustfmt::skip]
#[test] fn parses_run_with_extra_args() { assert_eq!(parse(["tj", "run", "codex", "headless", "fix", "tests"]).unwrap(), Action::Run(vec!["codex".into(), "headless".into(), "fix".into(), "tests".into()])); }
#[rustfmt::skip]
#[test] fn parses_legacy_and_direct_commands() {
    assert_eq!(parse(["tj", "opencode", "--help"]).unwrap(), Action::Direct { harness: "opencode".to_string(), extra: vec!["--help".to_string()] });
    assert_eq!(parse(["tj", "install", "opencode"]).unwrap(), Action::Install("opencode".to_string()));
    assert_eq!(parse(["tj", "status"]).unwrap(), Action::Check);
}
#[rustfmt::skip]
#[test] fn parses_version_flags() {
    assert_eq!(parse(["tj", "--version"]).unwrap(), Action::Version { verbose: false });
    assert_eq!(parse(["tj", "version"]).unwrap(), Action::Version { verbose: false });
    assert_eq!(parse(["tj", "version", "--verbose"]).unwrap(), Action::Version { verbose: true });
    assert_eq!(parse(["tj", "--info"]).unwrap(), Action::Version { verbose: true });
}
#[rustfmt::skip]
#[test] fn version_help_and_last_flag_wins() {
    assert_eq!(parse(["tj", "version", "--help"]).unwrap(), Action::Help);
    assert_eq!(parse(["tj", "version", "-v", "--verbose"]).unwrap(), Action::Version { verbose: true });
}
#[rustfmt::skip]
#[test] fn rejects_unknown_version_flag() {
    assert!(parse(["tj", "version", "-v"]).is_ok());
    let error = parse(["tj", "version", "--unknown"]).unwrap_err();
    assert!(error.contains("usage"));
}
#[rustfmt::skip]
#[test] fn rejects_unknown_capability() { let error = parse(["tj", "plan", "codex", "launch"]).unwrap_err(); assert!(error.contains("unknown capability")); }
#[rustfmt::skip]
#[test] fn rejects_unknown_flags_before_catalog_load() { let error = parse(["tj", "--v"]).unwrap_err(); assert!(error.contains("unknown flag '--v'")); }
#[rustfmt::skip]
#[test] fn parses_every_core_capability() { for capability in Capability::ALL { assert_eq!(Capability::parse(capability.as_str()), Some(capability)); } }
#[rustfmt::skip]
#[test] fn subcommand_help_routes_to_action_help() {
    for cmd in &["version", "list", "tools", "check", "status", "current", "use", "show", "info", "plan", "install", "update", "run", "auth", "config", "cache", "security", "templates", "db"] {
        assert_eq!(parse(["tj", cmd, "--help"]).unwrap(), Action::Help);
        assert_eq!(parse(["tj", cmd, "-h"]).unwrap(), Action::Help);
    }
}
#[rustfmt::skip]
#[test] fn rejects_unknown_version_flag_reports_flag_name() {
    let error = parse(["tj", "version", "--bogus"]).unwrap_err();
    assert!(error.contains("--bogus"));
    assert!(error.contains("usage"));
}
#[rustfmt::skip]
#[test] fn help_in_any_position_after_subcommand() {
    assert_eq!(parse(["tj", "plan", "yolo", "--help"]).unwrap(), Action::Help);
    assert_eq!(parse(["tj", "plan", "download", "--help"]).unwrap(), Action::Help);
    assert_eq!(parse(["tj", "show", "opencode", "--help"]).unwrap(), Action::Help);
    assert_eq!(parse(["tj", "use", "opencode", "--help"]).unwrap(), Action::Help);
    assert_eq!(parse(["tj", "install", "opencode", "--help"]).unwrap(), Action::Help);
    assert_eq!(parse(["tj", "security", "status", "--help"]).unwrap(), Action::Help);
}
#[rustfmt::skip]
#[test] fn run_forwards_help_to_harness_when_not_at_position_1() {
    assert_eq!(parse(["tj", "run", "codex", "--help"]).unwrap(), Action::Run(vec!["codex".into(), "--help".into()]));
    assert_eq!(parse(["tj", "run", "codex", "-h"]).unwrap(), Action::Run(vec!["codex".into(), "-h".into()]));
}
#[rustfmt::skip]
#[test] fn handles_global_flag_with_subcommand() {
    assert_eq!(parse(["tj", "-v", "version"]).unwrap(), Action::Version { verbose: false });
    let err = parse(["tj", "--version", "list"]).unwrap_err();
    assert!(err.contains("--version"));
    let err = parse(["tj", "--info", "show", "opencode"]).unwrap_err();
    assert!(err.contains("--info"));
}
#[rustfmt::skip]
#[test] fn run_help_without_harness_shows_help() {
    assert_eq!(parse(["tj", "run", "--help"]).unwrap(), Action::Help);
}
