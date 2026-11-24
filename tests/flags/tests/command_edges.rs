use crate::logic::sandbox::{err, out, Sandbox};
use std::fs;

const THREE: [(&str, bool, &str); 3] = [
    ("calm-mode", true, "stable"),
    ("quick-look", false, "preview"),
    ("wild-idea", false, "labs"),
];

#[test]
fn unknown_names_in_the_file_are_ignored() {
    let sandbox = Sandbox::new(Some(&THREE));
    fs::write(sandbox.settings_file(), "[flags]\nghost = true\n").unwrap();
    let output = sandbox.run(&["--plain", "settings"], &[]);
    assert!(err(&output).is_empty());
    assert!(!out(&output).contains("ghost"));
}

#[test]
fn json_and_help_follow_the_cli_conventions() {
    let sandbox = Sandbox::new(Some(&THREE));
    let json = out(&sandbox.run(&["--json", "settings"], &[]));
    assert!(json
        .starts_with("{\"schema_version\":1,\"command\":\"settings\",\"ok\":true,\"exit_code\":0"));
    let help = out(&sandbox.run(&["--plain", "settings", "--help"], &[]));
    assert!(help.contains("usage: terminal-jarvis settings"));
    assert!(out(&sandbox.run(&["--plain", "--help"], &[])).contains("terminal-jarvis settings"));
}

#[test]
fn a_broken_override_registry_is_an_unavailable_error() {
    let sandbox = Sandbox::new(Some(&[]));
    fs::create_dir_all(sandbox.flags.as_ref().unwrap().join("bad")).unwrap();
    fs::write(
        sandbox.flags.as_ref().unwrap().join("bad/index.toml"),
        "name = \"bad\"\n",
    )
    .unwrap();
    let output = sandbox.run(&["settings"], &[]);
    assert_eq!(output.status.code(), Some(4));
    assert!(err(&output).contains("flag 'bad'"));
}

#[test]
fn a_malformed_settings_file_warns_once_and_falls_back_to_defaults() {
    let sandbox = Sandbox::new(Some(&THREE));
    fs::write(sandbox.settings_file(), "[flags]\nwild-idea = maybe\n").unwrap();
    let output = sandbox.run(&["--plain", "settings"], &[]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(err(&output).matches("warning: settings.toml").count(), 1);
    assert!(out(&output).contains("flag=wild-idea state=off stage=labs source=default"));
}
