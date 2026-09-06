//! Stream: one-shot harness invocations whose output arrives line-by-line,
//! so the tui can paint splunk-style logs while the child works. Std-only:
//! two reader threads pump into a channel; the caller repaints at will.

use crate::contracts::CapabilityPlan;
use std::time::Duration;

/// Spawns the plan with fully piped output and feeds every stdout/stderr
/// line to `on_line` until the child exits. Stdin is null: headless runs
/// never prompt, so the tui frame above the log stays interactive-safe.
pub fn run(
    plan: &CapabilityPlan,
    extra: &[String],
    on_line: &mut dyn FnMut(&str),
) -> io::Result<i32> {
    let mut child = super::live::spawn(plan, extra)?;
    loop {
        match child.next(Duration::from_millis(150)) {
            super::live::Event::Line(super::live::Line::Out(line))
            | super::live::Event::Line(super::live::Line::Err(line)) => on_line(&line),
            super::live::Event::Idle => continue,
            super::live::Event::Done => return Ok(child.wait()),
        }
    }
}

pub(crate) fn status_code(status: std::process::ExitStatus) -> i32 {
    status.code().unwrap_or(3)
}

/// The wall-clock stamp splunk rows lead with, UTC, no external crates.
pub fn stamp() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(0);
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600 % 24,
        seconds / 60 % 60,
        seconds % 60
    )
}

/// Classifies one child line into a splunk row: ERROR for failures, WARN
/// for warnings, INFO otherwise -- matched on the whole lowercase line.
pub fn classify(line: &str) -> String {
    let lower = line.to_lowercase();
    let level = if lower.contains("error") || lower.contains("failed") || lower.contains("err!") {
        "ERROR"
    } else if lower.contains("warn") || lower.contains("deprecated") {
        "WARN "
    } else {
        "INFO "
    };
    format!("{} {} {}", stamp(), level, line)
}

use std::io;

#[cfg(test)]
#[path = "../tests/stream_test.rs"]
mod tests;
