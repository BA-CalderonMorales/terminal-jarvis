use crate::logic::tui::{booted, launch, EXIT, FRAME, SERIAL};
use crate::logic::{pty, screen};
use std::os::unix::fs::PermissionsExt;

fn slow_gate(fixture: &crate::logic::cli_driver::Fixture, name: &str, script: &str) {
    let gates = fixture.root.join("slow-gates");
    std::fs::create_dir_all(gates.join("acceptance")).unwrap();
    std::fs::write(
        gates.join("acceptance/index.toml"),
        format!("name = \"acceptance\"\ndisplay = \"Slow gate\"\ndescription = \"sticky\"\nbinary = \"{name}\"\nargs = []\ninstall_hint = \"fixture\"\n"),
    )
    .unwrap();
    let slow = fixture.root.join("bin").join(name);
    std::fs::write(&slow, script).unwrap();
    let mut permissions = std::fs::metadata(&slow).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&slow, permissions).unwrap();
}

#[test]
fn a_stuck_scanner_dies_fast_on_ctrl_c_and_the_run_continues_on_skip() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = booted();
    slow_gate(&fixture, "fixture-gate-slow", "#!/bin/sh\nsleep 60\n");
    let started = std::time::Instant::now();
    let (_, bytes) = pty::run_pty_probe(
        launch(
            &fixture,
            &[(
                "TERMINAL_JARVIS_GATES",
                fixture.root.join("slow-gates").to_str().unwrap(),
            )],
        ),
        &[
            (b"", Some(FRAME)),
            (b"/debug on\n", None),
            (b"run fixture ui\n", Some(b"debug view on".as_slice())),
            (b"y\n", Some(b"Continue with ui:fixture? [y/N]".as_slice())),
            (b"", Some(b"running security gate".as_slice())),
            (b"\x03", None),
            (
                b"",
                Some(b"Skip the scan and continue with ui:fixture? [y/N]".as_slice()),
            ),
            (b"y\n", None),
            (b"", Some(b"exited 0".as_slice())),
            (b"/debug off\n", None),
            EXIT,
        ],
    );
    let elapsed = started.elapsed();
    let screen = screen::render(&bytes);
    assert!(
        screen.contains("Skip the scan and continue with ui:fixture? [y/N]"),
        "no skip consent prompt:\n{}",
        screen.displayed()
    );
    assert!(
        fixture.spawned(),
        "skipping the scan must let the run proceed"
    );
    assert!(
        elapsed.as_secs() < 10,
        "scanner survived Ctrl+C for {elapsed:?}; the kill is missing"
    );
}

#[test]
fn a_slow_but_finishing_scan_redraws_a_patient_heartbeat_and_passes() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = booted();
    slow_gate(&fixture, "fixture-gate-tick", "#!/bin/sh\nsleep 8\n");
    let (_, bytes) = pty::run_pty_probe(
        launch(
            &fixture,
            &[(
                "TERMINAL_JARVIS_GATES",
                fixture.root.join("slow-gates").to_str().unwrap(),
            )],
        ),
        &[
            (b"", Some(FRAME)),
            (b"run fixture ui\n", None),
            (b"y\n", Some(b"Continue with ui:fixture? [y/N]".as_slice())),
            (b"", Some(b"security scan (acceptance) ...".as_slice())),
            (b"", Some(b"can take a minute or more".as_slice())),
            (b"", Some(b"security scan (acceptance): passed".as_slice())),
            (b"", Some(b"exited 0".as_slice())),
            EXIT,
        ],
    );
    let screen = screen::render(&bytes);
    assert!(
        screen.contains("exited 0"),
        "the slow scan still passes and the run completes:\n{}",
        screen.displayed()
    );
}
