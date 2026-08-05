use std::process::Command;

fn output(enabled: bool) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
    command.args(["--plain", "experimental", "dashboard"]);
    if enabled {
        command.env("TERMINAL_JARVIS_EXPERIMENTAL_UI", "1");
    }
    command.output().expect("terminal-jarvis runs")
}

#[test]
fn dashboard_requires_the_feature_wall() {
    let output = output(false);
    assert_eq!(output.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&output.stderr).contains("EXPERIMENTAL_UI=1"));
}

#[test]
fn dashboard_is_headless_when_explicitly_enabled() {
    let output = output(true);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Terminal Jarvis"));
    assert!(stdout.contains("experimental dashboard"));
    assert!(stdout.contains("mode: headless command center"));
}
