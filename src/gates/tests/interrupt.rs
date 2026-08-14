use crate::gates::logic::interrupt::{active_pid, memo_clear, memo_hit, memo_set, track};
use crate::gates::logic::runner::{outcome_line_for, preflight};
use crate::gates::logic::verdict::Verdict;

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
    assert!(!memo_hit("acceptance", "/one"));
    memo_set("acceptance", "/one");
    assert!(memo_hit("acceptance", "/one"));
    assert!(!memo_hit("other", "/one"));
    assert!(!memo_hit("acceptance", "/two"));
    memo_clear();
    assert!(!memo_hit("acceptance", "/one"));
}

#[test]
fn outcome_helpers_report_lines_only_when_clean() {
    let blocked = outcome_line_for("scan", "blocked", false, false).unwrap_or_default();
    let interrupted = outcome_line_for("scan", "interrupted", false, false).unwrap_or_default();
    assert!(blocked.contains("security scan (scan): blocked"));
    assert!(interrupted.contains("security scan (scan): interrupted"));
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
    assert_eq!(preflight(&home, true).unwrap(), Verdict::Passed);
    restore_gates_env(previous);
    let _ = std::fs::remove_dir_all(root);
}

#[cfg(unix)]
#[test]
fn memo_rescans_after_workspace_change_and_fails_open_without_cwd() {
    let _guard = lock();
    let root = std::env::temp_dir().join(format!("tj-memo-{}", std::process::id()));
    let home = root.join("home");
    let catalog = root.join("catalog");
    let count = root.join("count.txt");
    let workspaces = [root.join("a"), root.join("b"), root.join("lost")];
    for dir in &workspaces {
        std::fs::create_dir_all(dir).unwrap();
    }
    counter_gate(&catalog, "counter", &count);
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    let original_cwd = std::env::current_dir().unwrap();
    crate::gates::enable(&home, "counter").unwrap();
    std::env::set_current_dir(&workspaces[0]).unwrap();
    preflight(&home, true).unwrap();
    preflight(&home, true).unwrap();
    std::env::set_current_dir(&workspaces[1]).unwrap();
    preflight(&home, true).unwrap();
    std::env::set_current_dir(&workspaces[2]).unwrap();
    std::fs::remove_dir(&workspaces[2]).unwrap();
    preflight(&home, true).unwrap();
    preflight(&home, true).unwrap();
    std::env::set_current_dir(&original_cwd).unwrap();
    restore_gates_env(previous);
    assert_eq!(std::fs::read_to_string(&count).unwrap().lines().count(), 4);
    let _ = std::fs::remove_dir_all(root);
}
