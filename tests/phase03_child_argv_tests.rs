#![cfg(unix)]

#[path = "phase02_fixture/mod.rs"]
mod phase02_fixture;

use phase02_fixture::Fixture;

const ARGV_RECORDER: &str = r#"#!/bin/sh
: > "$TJ_PHASE02_MARKER"
printf 'cwd=<%s>\n' "$PWD"
printf 'argc=<%s>\n' "$#"
for arg in "$@"; do
    printf 'arg=<%s>\n' "$arg"
done
printf 'env-name=<TJ_PHASE02_MARKER>\n'
"#;

#[test]
fn fake_child_receives_exact_boundary_argv_cwd_and_allowlisted_env_name() {
    let fixture = Fixture::new("expected", "expected", ARGV_RECORDER);
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
    let expected = format!(
        "cwd=<{}>\nargc=<3>\narg=<alpha>\narg=<two words>\narg=<--json>\n\
         env-name=<TJ_PHASE02_MARKER>\n",
        env!("CARGO_MANIFEST_DIR")
    );
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, expected.as_bytes());
    assert!(output.stderr.is_empty());
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}
