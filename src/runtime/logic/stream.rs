//! Stream: one-shot harness invocations whose output arrives line-by-line,
//! so the tui can paint splunk-style logs while the child works. Std-only:
//! two reader threads pump into a channel; the caller repaints at will.

use crate::contracts::CapabilityPlan;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

/// Spawns the plan with fully piped output and feeds every stdout/stderr
/// line to `on_line` until the child exits. Stdin is null: headless runs
/// never prompt, so the tui frame above the log stays interactive-safe.
pub fn run(
    plan: &CapabilityPlan,
    extra: &[String],
    on_line: &mut dyn FnMut(&str),
) -> io::Result<i32> {
    let mut command = Command::new(crate::security::resolved(&plan.command.command).as_ref());
    command
        .args(&plan.command.args)
        .args(extra)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    super::runner::reset_sigint_in_child(&mut command);
    let mut child = command.spawn()?;
    let (tx, rx) = mpsc::channel::<String>();
    for stream in child
        .stdout
        .take()
        .map(pump)
        .into_iter()
        .chain(child.stderr.take().map(pump))
    {
        let tx = tx.clone();
        std::thread::spawn(move || {
            for line in stream {
                if tx.send(line).is_err() {
                    return;
                }
            }
        });
    }
    drop(tx);
    while let Ok(line) = rx.recv_timeout(Duration::from_millis(150)) {
        on_line(&line);
    }
    let code = child.wait()?;
    Ok(status_code(code))
}

fn pump<R: std::io::Read + Send + 'static>(pipe: R) -> mpsc::IntoIter<String> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(pipe).lines() {
            if tx.send(line.unwrap_or_default()).is_err() {
                return;
            }
        }
    });
    rx.into_iter()
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
