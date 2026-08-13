use crate::gates::logic::interrupt::{active_pid, memo_clear, memo_hit, memo_set, track};
use crate::gates::logic::runner::{outcome_line_for, preflight};

use crate::gates::tests_util::*;

#[test]
fn interrupt_track_and_active_pid_round_trip() {
    track(7);
    assert_eq!(active_pid(), 7);
    track(-2);
    assert_eq!(active_pid(), -2);
    track(0);
    assert_eq!(active_pid(), 0);
}

#[test]
fn scan_memo_remembers_and_clears() {
    memo_clear();
    assert!(!memo_hit("acceptance"));
    memo_set("acceptance");
    assert!(memo_hit("acceptance"));
    assert!(!memo_hit("other"));
    memo_clear();
    assert!(!memo_hit("acceptance"));
}

#[test]
fn outcome_helpers_report_lines_only_when_clean() {
    let blocked = outcome_line_for("scan", "blocked", false, false).unwrap_or_default();
    let interrupted = outcome_line_for("scan", "interrupted", false, false).unwrap_or_default();
    assert!(
        blocked.contains("security scan (scan): blocked"),
        "{blocked:?}"
    );
    assert!(
        interrupted.contains("security scan (scan): interrupted"),
        "{interrupted:?}"
    );
    assert_eq!(outcome_line_for("scan", "blocked", true, false), None);
    assert_eq!(outcome_line_for("scan", "interrupted", true, false), None);
}

#[test]
fn outcome_line_overpads_past_seen_heartbeat_ticks() {
    let plain = outcome_line_for("scan", "blocked", false, false).unwrap();
    let ticked = outcome_line_for("scan", "blocked", false, true).unwrap();
    assert!(
        ticked.len() > plain.len(),
        "a heartbeat-aware outcome must pad past the ticks"
    );
    assert!(plain.ends_with("blocked "), "{plain:?}");
    assert!(ticked.ends_with(' '), "{ticked:?}");
}

#[cfg(unix)]
#[test]
fn preflight_warns_and_continues_when_binary_is_missing() {
    let _guard = lock();
    let root = std::env::temp_dir().join(format!("tj-preflight-missing-{}", std::process::id()));
    let home = root.join("home");
    let catalog = root.join("catalog");
    let _ = std::fs::remove_dir_all(&root);
    write_gate(&catalog, "phantom", "definitely-not-a-real-binary-xyz");
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    crate::gates::enable(&home, "phantom").unwrap();
    assert_eq!(
        preflight(&home, true).unwrap(),
        crate::gates::logic::verdict::Verdict::Passed
    );
    restore_gates_env(previous);
    let _ = std::fs::remove_dir_all(root);
}
