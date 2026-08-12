#![cfg(unix)]

use crate::logic::cli_driver::Fixture;
use std::os::unix::fs::MetadataExt;

const ARGV_RECORDER: &str = r#"#!/bin/sh
: > "$TJ_FIXTURE_MARKER"
printf 'cwd=<%s>\n' "$PWD"
printf 'argc=<%s>\n' "$#"
for arg in "$@"; do
    printf 'arg=<%s>\n' "$arg"
done
printf 'env-name=<TJ_FIXTURE_MARKER>\n'
"#;

#[test]
fn fake_child_receives_exact_boundary_argv_cwd_and_allowlisted_env_name() {
    let fixture = Fixture::new("expected", "expected", ARGV_RECORDER);
    let cwd = std::env::current_dir().unwrap();
    let output = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "headless",
        "--",
        "alpha",
        "two words",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let (reported_cwd, boundary) = stdout.split_once('\n').unwrap();
    let reported_cwd = reported_cwd
        .strip_prefix("cwd=<")
        .and_then(|value| value.strip_suffix('>'))
        .unwrap();
    let reported = std::fs::metadata(reported_cwd).unwrap();
    let expected = std::fs::metadata(cwd).unwrap();
    assert_eq!(
        (reported.dev(), reported.ino()),
        (expected.dev(), expected.ino())
    );
    assert_eq!(
        boundary,
        "argc=<3>\narg=<alpha>\narg=<two words>\narg=<--json>\n\
         env-name=<TJ_FIXTURE_MARKER>\n"
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("running security gate 'acceptance'"));
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}
