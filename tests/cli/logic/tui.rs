//! Shared machinery for the tui acceptance oracle: a hermetically seeded
//! fixture, the pty launch, and serialization (pty scenarios are timing-
//! and signal-sensitive against each other; other tests run untouched).

use super::cli_driver::{prepend_to_path, Fixture};
use std::process::Command;

#[cfg(unix)]
pub const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";
#[cfg(not(unix))]
pub const MARKER_SCRIPT: &str = "@echo off\r\ntype nul > \"%TJ_FIXTURE_MARKER%\"\r\n";
pub const FRAME: &[u8] = b"[>_]::[tj:";
pub const EXIT: (&[u8], Option<&[u8]>) = (b"\x04", None);

pub static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// A fixture whose active harness is already `fixture`, and its tui launch.
pub fn booted() -> Fixture {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    fixture.run(&["--plain", "use", "fixture"]);
    fixture
}

pub fn launch(fixture: &Fixture, extra: &[(&str, &str)]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
    command.arg("tui");
    command.env("PATH", prepend_to_path(&fixture.root.join("bin")));
    command.env("TERMINAL_JARVIS_CATALOG", fixture.root.join("catalog"));
    command.env("TERMINAL_JARVIS_GATE", "acceptance");
    command.env("TERMINAL_JARVIS_GATES", fixture.root.join("gates"));
    command.env("TERMINAL_JARVIS_HOME", fixture.root.join("home"));
    command.env("TJ_FIXTURE_MARKER", fixture.root.join("spawned"));
    command.env("TJ_FIXTURE_GATE_MARKER", fixture.root.join("gate-spawned"));
    for (key, value) in extra {
        command.env(key, value);
    }
    command
}
