use crate::logic::tui::{booted, launch, EXIT, FRAME, SERIAL};
use crate::logic::{pty, screen};

#[test]
fn a_stuck_scanner_dies_fast_on_ctrl_c_and_the_run_cancels() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = booted();
    let slow_gates = fixture.root.join("slow-gates");
    std::fs::create_dir_all(&slow_gates).unwrap();
    std::fs::create_dir_all(slow_gates.join("acceptance")).unwrap();
    std::fs::write(
        slow_gates.join("acceptance/index.toml"),
        "name = \"acceptance\"\ndisplay = \"Slow gate\"\ndescription = \"sticky\"\nbinary = \"fixture-gate-slow\"\nargs = []\ninstall_hint = \"fixture\"\n",
    )
    .unwrap();
    let slow = fixture.root.join("bin").join("fixture-gate-slow");
    std::fs::write(&slow, "#!/bin/sh\nsleep 60\n").unwrap();
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&slow).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&slow, permissions).unwrap();
    }
    let started = std::time::Instant::now();
    let (_, bytes) = pty::run_pty_probe(
        launch(
            &fixture,
            &[("TERMINAL_JARVIS_GATES", slow_gates.to_str().unwrap())],
        ),
        &[
            (b"", Some(FRAME)),
            (b"/debug on\n", None),
            (b"run fixture ui\n", Some(b"debug view on".as_slice())),
            (b"y\n", Some(b"Continue with ui:fixture? [y/N]".as_slice())),
            (b"", Some(b"running security gate".as_slice())),
            (b"\x03", None),
            (b"", Some(b"scan cancelled".as_slice())),
            (b"/debug off\n", None),
            EXIT,
        ],
    );
    let elapsed = started.elapsed();
    let screen = screen::render(&bytes);
    assert!(
        screen.contains("scan cancelled"),
        "no cancel verdict:\n{}",
        screen.displayed()
    );
    assert!(
        elapsed.as_secs() < 10,
        "scanner survived Ctrl+C for {elapsed:?}; the kill is missing"
    );
}
