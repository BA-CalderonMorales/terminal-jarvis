//! Verdict: the pure classification of a finished scan plus the exact
//! user-facing messages, split out so the red/green contract (passed,
//! blocked-with-findings, interrupted) is property-tested without process
//! races. Blocked verdicts carry the full message: vulnerabilities found
//! must never be silently downgraded to a skip decision.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Verdict {
    Passed,
    Blocked(String),
    Interrupted { gate: String },
}

/// Exit 0 passes; a signal kill (>= 128) is an interruption; anything else
/// found findings and blocks with a message that names the exact exit.
pub fn verdict_for(gate: &str, code: i32, output: &str) -> Verdict {
    if code == 0 {
        Verdict::Passed
    } else if code > 128 {
        Verdict::Interrupted {
            gate: gate.to_string(),
        }
    } else {
        Verdict::Blocked(blocked_message(gate, code, &block_summary(output)))
    }
}

/// The meaningful tail of a failed scan: drops trivy's INFO chatter and
/// keeps the signal lines.
pub fn block_summary(output: &str) -> String {
    let tail = output
        .lines()
        .filter(|line| !noise(line) && !line.trim().is_empty())
        .rev()
        .take(6)
        .collect::<Vec<_>>();
    let mut lines = tail.into_iter().rev().collect::<Vec<_>>();
    if lines.is_empty() {
        lines.push(output.lines().next_back().unwrap_or("scan failed"));
    }
    lines.join("\n")
}

fn noise(line: &str) -> bool {
    line.contains(" INFO ") || line.contains("?INFO?")
}

pub fn blocked_message(gate: &str, code: i32, summary: &str) -> String {
    format!("security gate '{gate}' blocked harness execution (exit {code})\n{summary}")
}

pub fn interrupted_message(gate: &str) -> String {
    format!("security gate '{gate}' was interrupted (Ctrl+C); scan cancelled")
}

#[cfg(test)]
#[path = "../tests/verdict.rs"]
mod verdict_tests;
