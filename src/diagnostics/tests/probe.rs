//! Probe fixtures: one POSIX snippet and one `cmd` snippet per assertion,
//! so the probe's line-selection rules are exercised on every platform.

use super::*;

#[cfg(windows)]
fn run(_posix: &str, windows: &str) -> Option<String> {
    version("cmd", &["/c".to_string(), windows.to_string()])
}

#[cfg(not(windows))]
fn run(posix: &str, _windows: &str) -> Option<String> {
    version("sh", &["-c".to_string(), posix.to_string()])
}

#[test]
fn prefers_the_digit_bearing_line() {
    assert_eq!(
        run(
            "printf 'Warning: no daemon\\nclient version is 0.30.11\\n'",
            "echo Warning: no daemon& echo client version is 0.30.11"
        )
        .as_deref(),
        Some("client version is 0.30.11")
    );
}

#[test]
fn falls_back_to_the_first_non_empty_line_without_digits() {
    assert_eq!(
        run(
            "printf '\\nMistral Vibe CLI\\nusage: open with--version\\n'",
            "echo.& echo Mistral Vibe CLI& echo usage: open with--version"
        )
        .as_deref(),
        Some("Mistral Vibe CLI")
    );
}

#[test]
fn empty_or_unstartable_probes_report_none() {
    assert_eq!(run("exit 0", "exit 0"), None);
    assert_eq!(version("definitely-missing-binary-xyz", &[]), None);
}

#[test]
fn long_lines_are_truncated() {
    let long = "x".repeat(200);
    assert_eq!(
        run(&format!("printf '{long}\\n'"), &format!("echo {long}")).map(|v| v.len()),
        Some(48)
    );
}

#[test]
fn slow_probes_time_out() {
    assert_eq!(run("echo hi; sleep 4", "ping -n 6 127.0.0.1 >nul"), None);
}

#[test]
fn probes_finishing_within_the_timeout_still_report_output() {
    assert_eq!(
        run(
            "sleep 1; printf 'v1.2.3\\n'",
            "ping -n 2 127.0.0.1 >nul& echo v1.2.3"
        )
        .as_deref(),
        Some("v1.2.3")
    );
}
