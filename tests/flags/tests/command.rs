//! The `settings` command against a fixture registry, through the binary.

use crate::logic::sandbox::{err, out, Sandbox};
use std::fs;

const THREE: [(&str, bool, &str); 3] = [
    ("calm-mode", true, "stable"),
    ("quick-look", false, "preview"),
    ("wild-idea", false, "labs"),
];

#[test]
fn an_empty_registry_prints_a_calm_line() {
    let sandbox = Sandbox::new(Some(&[]));
    for args in [&["--plain", "settings"][..], &["settings"]] {
        let output = sandbox.run(args, &[]);
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(out(&output), "No optional features yet.\n");
    }
}

#[test]
fn rows_group_stable_and_preview_before_labs_with_their_source() {
    let sandbox = Sandbox::new(Some(&THREE));
    let text = out(&sandbox.run(&["--no-color", "settings"], &[]));
    let labs = text.find("Labs").unwrap();
    assert!(text.find("[on ]  calm-mode").unwrap() < text.find("quick-look").unwrap());
    assert!(text.find("quick-look").unwrap() < labs);
    assert!(text.contains("experimental, off by default"));
    assert!(text[labs..].contains("[off]  wild-idea"));
    assert!(text.contains("A fixture flag for calm-mode.  (default)"));
}

#[test]
fn set_reset_and_env_show_their_source() {
    let sandbox = Sandbox::new(Some(&THREE));
    let set = sandbox.run(&["settings", "set", "wild-idea", "on"], &[]);
    assert_eq!(set.status.code(), Some(0), "{}", err(&set));
    let file = fs::read_to_string(sandbox.settings_file()).unwrap();
    assert!(file.contains("[flags]") && file.contains("wild-idea = true"));
    let listed = out(&sandbox.run(&["--plain", "settings"], &[]));
    assert!(listed.contains("flag=wild-idea state=on stage=labs source=settings"));
    let forced = out(&sandbox.run(&["--plain", "settings"], &[("TJ_FLAG_WILD_IDEA", "0")]));
    assert!(forced.contains("flag=wild-idea state=off stage=labs source=env"));
    let junk = out(&sandbox.run(&["--plain", "settings"], &[("TJ_FLAG_WILD_IDEA", "maybe")]));
    assert!(junk.contains("flag=wild-idea state=on stage=labs source=settings"));
    assert_eq!(
        sandbox
            .run(&["settings", "reset", "wild-idea"], &[])
            .status
            .code(),
        Some(0)
    );
    let after = out(&sandbox.run(&["--plain", "settings"], &[]));
    assert!(after.contains("flag=wild-idea state=off stage=labs source=default"));
}

#[test]
fn unknown_flags_and_bad_states_are_usage_errors() {
    let sandbox = Sandbox::new(Some(&THREE));
    let unknown = sandbox.run(&["settings", "set", "nope", "on"], &[]);
    assert_eq!(unknown.status.code(), Some(2));
    assert!(err(&unknown)
        .contains("unknown flag 'nope'; valid flags: calm-mode, quick-look, wild-idea"));
    assert_eq!(
        sandbox
            .run(&["settings", "reset", "nope"], &[])
            .status
            .code(),
        Some(2)
    );
    let state = sandbox.run(&["settings", "set", "calm-mode", "maybe"], &[]);
    assert_eq!(state.status.code(), Some(2));
    assert!(err(&state).contains("usage: terminal-jarvis settings"));
    assert_eq!(
        sandbox
            .run(&["settings", "set", "calm-mode", "1"], &[])
            .status
            .code(),
        Some(2)
    );
    assert!(!sandbox.settings_file().exists());
}

#[test]
fn dry_run_previews_without_writing() {
    let sandbox = Sandbox::new(Some(&THREE));
    let output = sandbox.run(&["settings", "set", "wild-idea", "on", "--dry-run"], &[]);
    assert_eq!(output.status.code(), Some(0));
    assert!(out(&output).contains("dry run: would set wild-idea on"));
    assert!(!sandbox.settings_file().exists());
}
