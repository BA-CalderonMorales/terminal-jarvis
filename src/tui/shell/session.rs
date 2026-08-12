//! Session: brackets a run/direct action with a dim header before the child
//! and a recap after it (exit + elapsed), so agent output sits in a clear
//! frame inside the chat scrollback. All frame bytes go through the caller's
//! sink so tests assert order and content without a terminal. The child
//! itself always inherits the real stdout/stderr, exactly like headless.

use crate::cli::{args, dispatch, style};
use crate::contracts::Harness;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

pub fn run(
    action: args::Action,
    options: &args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> bool {
    match action {
        args::Action::Run(_) | args::Action::Direct { .. } => frame(
            action,
            options,
            harnesses,
            catalog_root,
            state_home,
            &mut std::io::stdout(),
        ),
        other => super::canonical::run(other, options, harnesses, catalog_root, state_home),
    }
}

fn frame(
    action: args::Action,
    options: &args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    out: &mut dyn Write,
) -> bool {
    let started = Instant::now();
    let label = command_label(&action);
    let _ = writeln!(out, "{}", style::dim(chapter(&label).as_str()));
    crate::tui::sigint::child_running(true);
    let outcome = dispatch(action, options, harnesses, catalog_root, state_home);
    crate::tui::sigint::child_running(false);
    match &outcome {
        Ok((_, body)) if !body.is_empty() => {
            let _ = out.write_all(body.as_bytes());
            let _ = out.write_all(b"\n");
        }
        Err(message) => {
            let _ = writeln!(out, "{}", style::error(message));
        }
        _ => {}
    }
    let code = outcome.as_ref().map(|(code, _)| *code).ok();
    let _ = writeln!(
        out,
        "{}",
        style::dim(recap(&label, code, started.elapsed()).as_str())
    );
    false
}

fn command_label(action: &args::Action) -> String {
    let parts = match action {
        args::Action::Run(words) => std::iter::once("run".to_string())
            .chain(words.iter().cloned())
            .collect(),
        args::Action::Direct { harness, extra } => std::iter::once(harness.clone())
            .chain(extra.iter().cloned())
            .collect(),
        _ => vec!["action".to_string()],
    };
    parts.join(" ")
}

pub fn chapter(label: &str) -> String {
    format!("── {label} ──")
}

pub fn recap(label: &str, code: Option<i32>, elapsed: Duration) -> String {
    let verdict = match code {
        Some(0) => "exited 0".to_string(),
        Some(actual) => format!("exited {actual}"),
        None => "failed".to_string(),
    };
    format!(
        "── {label} {verdict} · {} ──",
        super::verdict::human(elapsed)
    )
}

#[cfg(test)]
#[path = "../tests/session.rs"]
mod tests;
