//! Verdict: the compact result card for lifecycle actions. In the clean tui
//! mode an install or update answers in one line -- what happened, in how
//! long -- instead of echoing headless machinery. Rendering is pure; the
//! caller measures the action, adopts the installed harness, and styles.

use crate::cli::style;
use std::path::Path;
use std::time::Duration;

/// Returns (adopted, verdict-text, persisted): adopted reports the caller
/// should switch the active harness; persisted reports the executable was
/// found on PATH after a successful install -- the truth the readiness probe
/// will show later.
pub fn text(
    name: &str,
    verb: &str,
    binary_on_path: bool,
    outcome: &Result<(i32, String), String>,
    elapsed: Duration,
) -> (bool, String, bool) {
    match outcome {
        Ok((0, _)) if binary_on_path => {
            let tail = if verb == "installed" {
                " · now active"
            } else {
                ""
            };
            (
                verb == "installed",
                format!("{verb} {name} · {}{tail}", human(elapsed)),
                true,
            )
        }
        Ok((0, _)) => (
            verb == "installed",
            format!("{verb} {name} · {} · binary not on PATH", human(elapsed)),
            false,
        ),
        Ok((code, _)) => (
            false,
            format!("{verb} {name} failed (exit {code}) · {}", human(elapsed)),
            false,
        ),
        Err(_) => (
            false,
            format!("{verb} {name} blocked · {}", human(elapsed)),
            false,
        ),
    }
}

/// Finals an install/update card: renders it, warns when the executable did
/// not land on PATH, and adopts the harness only when it persisted.
pub fn settle(
    name: &str,
    verb: &str,
    binary_on_path: bool,
    outcome: &Result<(i32, String), String>,
    elapsed: Duration,
    state_home: &Path,
) {
    let (adopted, text, persisted) = text(name, verb, binary_on_path, outcome, elapsed);
    let ok = outcome.as_ref().map(|(code, _)| *code).ok() == Some(0);
    println!(
        "{}",
        if ok {
            style::success(&text)
        } else {
            style::error(&text)
        }
    );
    if !persisted && ok {
        eprintln!(
            "{}",
            style::warning(&format!(
                "warning: {name}'s binary was not found on PATH; restart the shell or add its install directory"
            ))
        );
    }
    if adopted && persisted {
        let _ = crate::context::save(state_home, name);
    }
}

pub fn human(elapsed: Duration) -> String {
    let seconds = elapsed.as_secs_f64();
    if seconds < 60.0 {
        return format!("{seconds:.1}s");
    }
    let total = elapsed.as_secs();
    format!("{}m{:02}s", total / 60, total % 60)
}

#[cfg(test)]
#[path = "../tests/verdict.rs"]
mod tests;
