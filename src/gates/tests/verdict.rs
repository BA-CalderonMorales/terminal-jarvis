use crate::gates::logic::verdict::{
    block_summary, blocked_message, interrupted_message, verdict_for, Verdict,
};

#[test]
fn verdict_classifies_every_exit_exactly_once() {
    quickcheck::quickcheck(verdict_is_exhaustive_and_exclusive as fn(String, i32, String) -> bool);
    fn verdict_is_exhaustive_and_exclusive(gate: String, code: i32, output: String) -> bool {
        match verdict_for(&gate, code, &output) {
            Verdict::Passed => code == 0,
            Verdict::Interrupted { gate: seen } => code > 128 && seen == gate,
            Verdict::Blocked(message) => {
                code != 0
                    && code <= 128
                    && message.contains("blocked harness execution")
                    && message.contains(&format!("(exit {code})"))
                    && !message.is_empty()
            }
        }
    }
}

#[test]
fn blocked_always_reports_the_findings_tail() {
    quickcheck::quickcheck(blocked_carries_findings as fn(String, i32, String) -> bool);
    fn blocked_carries_findings(gate: String, code: i32, output: String) -> bool {
        match verdict_for(&gate, code, &output) {
            Verdict::Blocked(message) => message.ends_with(&block_summary(&output)),
            _ => true,
        }
    }
}

#[test]
fn a_blocked_scan_never_downgrades_to_a_skip() {
    for code in 1..=128 {
        match verdict_for("scan", code, "CRITICAL minimist CVE-2021-44906") {
            Verdict::Blocked(message) => {
                assert!(
                    message.contains("CRITICAL minimist CVE-2021-44906"),
                    "{message}"
                );
            }
            other => panic!("exit {code} must block, got {other:?}"),
        }
    }
}

#[test]
fn block_summary_never_comes_back_empty() {
    quickcheck::quickcheck(never_empty as fn(String) -> bool);
    fn never_empty(output: String) -> bool {
        !block_summary(&output).is_empty()
    }
}

#[test]
fn messages_are_exact_and_self_describing() {
    assert_eq!(
        blocked_message("scan", 1, "CRITICAL minimist"),
        "security gate 'scan' blocked harness execution (exit 1)\nCRITICAL minimist"
    );
    assert_eq!(
        interrupted_message("scan"),
        "security gate 'scan' was interrupted (Ctrl+C); scan cancelled"
    );
    assert!(!interrupted_message("scan").is_empty());
}
