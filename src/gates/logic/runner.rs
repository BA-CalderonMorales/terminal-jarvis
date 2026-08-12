use crate::gates::logic::interrupt::{memo_clear, memo_hit, memo_set};
use crate::gates::logic::loader::load;
use crate::gates::structs::state::selected;
use crate::{context, security};
use std::path::Path;

pub fn preflight(home: &Path, narrate: bool) -> Result<(), String> {
    let Some(selection) = selected(home).map_err(|error| error.to_string())? else {
        return Ok(());
    };
    let gates = load(&context::gates_root()).map_err(|error| error.to_string())?;
    let gate = gates
        .iter()
        .find(|gate| gate.name == selection.name)
        .ok_or_else(|| {
            format!(
                "enabled gate '{}' is not in the gate catalog",
                selection.name
            )
        })?;
    if memo_hit(&gate.name) {
        return Ok(());
    }
    if !security::command_on_path(&gate.binary) {
        eprintln!(
            "warning: optional gate '{}' is enabled but '{}' is not on PATH; {} Run `terminal-jarvis gate disable` to stop the warning, or install the scanner to start scanning.",
            gate.name, gate.binary, gate.install_hint
        );
        return Ok(());
    }
    if narrate {
        eprintln!("running security gate '{}' ...", gate.name);
    } else {
        eprint!("security scan ({}) ...", gate.name);
    }
    let (code, output) = super::stream::run(gate, narrate)?;
    if code == 0 {
        memo_set(&gate.name);
        if narrate {
            eprintln!("security gate '{}' passed", gate.name);
        } else {
            eprintln!("{}", outcome_line(&gate.name, "passed"));
        }
        return Ok(());
    }
    memo_clear();
    if code > 128 {
        if let Some(line) = outcome_line_for(&gate.name, "interrupted", narrate) {
            eprintln!("{line}");
        }
        return Err(format!(
            "security gate '{}' was interrupted (Ctrl+C); scan cancelled",
            gate.name
        ));
    }
    if let Some(line) = outcome_line_for(&gate.name, "blocked", narrate) {
        eprintln!("{line}");
    }
    Err(format!(
        "security gate '{}' blocked harness execution (exit {code})\n{}",
        gate.name,
        super::stream::block_summary(&output)
    ))
}

/// The clean-view outcome line, or none when narrating (the narrated view
/// has its own prose elsewhere). Pure decision helper, unit-tested on its
/// own so the values the tui prints are witnessed.
pub(crate) fn outcome_line_for(gate: &str, state: &str, narrate: bool) -> Option<String> {
    if narrate {
        None
    } else {
        Some(outcome_line(gate, state))
    }
}

/// Rewrites the live "… running …" line in place: CR, the outcome, then
/// spaces out-padding past the running text. Pure CR/padding, no ESC.
fn outcome_line(gate: &str, state: &str) -> String {
    let result = format!("security scan ({gate}): {state}");
    let pad = format!("security scan ({gate}) ...")
        .len()
        .saturating_sub(result.len())
        + 1;
    format!("\r{result}{}", " ".repeat(pad))
}

#[cfg(test)]
#[path = "../tests/runner.rs"]
mod gates_runner_tests;
