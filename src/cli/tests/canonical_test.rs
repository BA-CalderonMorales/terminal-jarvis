use super::*;
use crate::cli::args;

fn local() -> (std::path::PathBuf, std::path::PathBuf) {
    (std::env::temp_dir(), std::env::temp_dir())
}

#[test]
fn command_help_returns_the_dedicated_text() {
    let (catalog, home) = local();
    let body = text(args::Action::CommandHelp("run".into()), &catalog, &home).unwrap();
    assert!(body.contains("run"));
}

#[test]
fn version_returns_the_version_line() {
    let (catalog, home) = local();
    let body = text(args::Action::Version { verbose: false }, &catalog, &home).unwrap();
    assert!(body.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn self_update_advises_a_normal_shell_and_never_applies() {
    let (catalog, home) = local();
    for dry_run in [true, false] {
        let body = text(args::Action::SelfUpdate { dry_run }, &catalog, &home).unwrap();
        assert!(body.contains("--update"));
        assert!(!body.contains("does not swap"));
    }
}

#[test]
fn other_actions_have_no_canonical_form() {
    let (catalog, home) = local();
    assert!(text(args::Action::List, &catalog, &home).is_err());
}
