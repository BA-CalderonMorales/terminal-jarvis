use super::*;
use crate::diagnostics::RuntimeInput;
use std::path::Path;
use std::time::Duration;

fn run(home: Option<&str>) -> DiagnosticInput {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    std::env::remove_var("USERPROFILE");
    match home {
        Some(value) => std::env::set_var("HOME", value),
        None => std::env::remove_var("HOME"),
    }
    DiagnosticInput::local(
        Path::new("/tmp/tj-catalog"),
        Path::new("/tmp/tj-home"),
        None,
        &[],
        RuntimeInput {
            gate: "/tmp/tj-gate".into(),
            stdout_tty: true,
            stderr_tty: true,
            color: false,
            width: 80,
            update_route: "source".into(),
            checksum: "".into(),
            probes: false,
        },
    )
}

#[test]
fn stale_after_is_thirty_days() {
    assert_eq!(
        run(Some("/tmp/tj-home")).stale_after,
        Duration::from_secs(30 * 24 * 60 * 60)
    );
}

#[test]
fn home_prefix_reads_home_env() {
    let input = run(Some("/tmp/tj-home"));
    assert_eq!(
        input.home_prefix.as_deref(),
        Some(Path::new("/tmp/tj-home"))
    );
}

#[test]
fn home_prefix_is_none_without_home_vars() {
    assert_eq!(run(None).home_prefix, None);
}

#[test]
fn home_prefix_rejects_blank_home() {
    assert_eq!(run(Some("   ")).home_prefix, None);
}
