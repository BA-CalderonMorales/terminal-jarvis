#![cfg(unix)]

use crate::logic::cli_driver::Fixture;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";

#[test]
fn guard_intent_confirm_token_match_with_terminal_succeeds() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "fixture",
        "download",
        "--no-input",
        "--confirm=download:fixture",
    ]);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());

    // Also test with install command (legacy style)
    let output2 = fixture.run(&[
        "--plain",
        "install",
        "fixture",
        "--no-input",
        "--confirm=download:fixture",
    ]);
    eprintln!(
        "install fixture stdout: {}",
        String::from_utf8_lossy(&output2.stdout)
    );
    eprintln!(
        "install fixture stderr: {}",
        String::from_utf8_lossy(&output2.stderr)
    );
    eprintln!("install fixture exit code: {:?}", output2.status.code());

    assert_eq!(output.status.code(), Some(0));
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}
