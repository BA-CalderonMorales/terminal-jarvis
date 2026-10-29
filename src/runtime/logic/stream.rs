//! Stream: one-shot harness invocations whose output arrives line-by-line,
//! so the tui can paint classified rows while the child works. Std-only:
//! two reader threads pump into a channel; the caller repaints at will.

use crate::contracts::CapabilityPlan;
use std::io;
use std::time::Duration;

/// The code a cancelled child reports: 128 + SIGINT.
pub const CANCELLED: i32 = 130;

/// Spawns the plan with fully piped output and feeds every stdout/stderr
/// line to `on_line` until the child exits. `cancelled` is polled on every
/// tick, so a quiet or hung child can still be aborted. Stdin is null and the
/// child has no terminal: headless runs never prompt, and an installer cannot
/// reach past our pipes to ask.
pub fn run(
    plan: &CapabilityPlan,
    extra: &[String],
    on_line: &mut dyn FnMut(&str),
    cancelled: &mut dyn FnMut() -> bool,
) -> io::Result<i32> {
    let mut child = super::live::spawn(plan, extra)?;
    loop {
        if cancelled() {
            child.kill();
            let _ = child.wait();
            return Ok(CANCELLED);
        }
        match child.next(Duration::from_millis(150)) {
            super::live::Event::Line(super::live::Line::Out(line))
            | super::live::Event::Line(super::live::Line::Err(line)) => on_line(&line),
            super::live::Event::Idle => continue,
            super::live::Event::Done => return Ok(child.wait()),
        }
    }
}

pub(crate) fn status_code(status: std::process::ExitStatus) -> i32 {
    super::runner::exit_code(status)
}

/// The level labels the frame knows, padded to one width so a column of rows
/// stays aligned. Longest first: `ERROR` must not be read as a message that
/// happens to begin with `ERR`.
const LEVELS: [&str; 3] = ["ERROR", "WARN ", "INFO "];

/// Classifies one child line for the live frame: the level the line declares,
/// or one inferred from its text. There is deliberately no wall-clock stamp
/// -- a live view already shows when it is, and the stamp only widened every
/// row while telling the reader nothing.
pub fn classify(line: &str) -> String {
    // A blank line is spacing, not a record: labelling it `INFO` turns the
    // child's paragraph breaks into rows that read as content.
    if line.trim().is_empty() {
        return String::new();
    }
    match declared_level(line) {
        Some((level, message)) => format!("{level} {message}"),
        None => format!("{} {line}", inferred_level(line)),
    }
}

/// The child's own level token, when the line leads with one: the log's own
/// signal beats anything guessed from the text.
fn declared_level(line: &str) -> Option<(&'static str, &str)> {
    for level in LEVELS {
        let token = level.trim_end();
        let lower = token.to_lowercase();
        for candidate in [token, lower.as_str()] {
            let Some(rest) = line.strip_prefix(candidate) else {
                continue;
            };
            if rest.is_empty() || rest.starts_with([' ', '\t', ':', '|']) {
                return Some((level, rest.trim_start_matches([' ', '\t', ':', '|'])));
            }
        }
    }
    None
}

/// The fallback level: the whole lowercase line, as before, but without the
/// timestamp that used to make every row look like a log record.
fn inferred_level(line: &str) -> &'static str {
    let lower = line.to_lowercase();
    if lower.contains("error") || lower.contains("failed") || lower.contains("err!") {
        "ERROR"
    } else if lower.contains("warn") || lower.contains("deprecated") {
        "WARN "
    } else {
        "INFO "
    }
}

#[cfg(test)]
#[path = "../tests/stream_test.rs"]
mod tests;
