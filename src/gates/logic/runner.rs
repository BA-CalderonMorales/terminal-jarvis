use crate::gates::logic::heartbeat::live_width;
use crate::gates::logic::interrupt::{memo_clear, memo_hit, memo_set};
use crate::gates::logic::loader::load;
use crate::gates::logic::verdict::{verdict_for, Verdict};
use crate::gates::structs::state::selected;
use crate::{context, security};
use std::path::Path;

/// Runs the enabled gate before a guarded action. A passed scan is memoized
/// per scanned workspace; blocked carries its message.
pub fn preflight(home: &Path, narrate: bool) -> Result<Verdict, String> {
    let Some(selection) = selected(home).map_err(|error| error.to_string())? else {
        return Ok(Verdict::Passed);
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
    let workspace = std::env::current_dir().ok();
    let workspace = workspace.map(|path| path.display().to_string());
    match workspace.as_deref() {
        Some(workspace) if memo_hit(&gate.name, workspace) => return Ok(Verdict::Passed),
        _ => {}
    }
    if !security::command_on_path(&gate.binary) {
        eprintln!(
            "warning: optional gate '{}' is enabled but '{}' is not on PATH; {} Run `terminal-jarvis gate disable` to stop the warning, or install the scanner to start scanning.",
            gate.name, gate.binary, gate.install_hint
        );
        return Ok(Verdict::Passed);
    }
    if narrate {
        eprintln!("running security gate '{}' ...", gate.name);
    } else {
        eprint!("security scan ({}) ...", gate.name);
    }
    let scan = super::stream::run(gate, narrate)?;
    if scan.code == 0 {
        if let Some(workspace) = &workspace {
            memo_set(&gate.name, workspace);
        }
        if narrate {
            eprintln!("security gate '{}' passed", gate.name);
        } else if let Some(line) = outcome_line_for(&gate.name, "passed", narrate, scan.heartbeat) {
            eprintln!("{line}");
        }
        return Ok(Verdict::Passed);
    }
    memo_clear();
    let verdict = verdict_for(&gate.name, scan.code, &scan.output);
    let state = match &verdict {
        Verdict::Interrupted { .. } => "interrupted",
        Verdict::Blocked(_) => "blocked",
        Verdict::Passed => unreachable!("handled above"),
    };
    if let Some(line) = outcome_line_for(&gate.name, state, narrate, scan.heartbeat) {
        eprintln!("{line}");
    }
    Ok(verdict)
}

/// The clean-view outcome line, or none when narrating (the narrated view
/// has its own prose elsewhere). Pure decision helper, unit-tested on its
/// own so the values the tui prints are witnessed.
pub(crate) fn outcome_line_for(
    gate: &str,
    state: &str,
    narrate: bool,
    heartbeat: bool,
) -> Option<String> {
    (!narrate).then(|| outcome_line(gate, state, heartbeat))
}

/// Rewrites the live "… running …" line in place: CR, the outcome, then
/// spaces out-padding past the running text (and past any heartbeat ticks
/// the user saw, so no redraw residue survives). Pure CR/padding, no ESC.
fn outcome_line(gate: &str, state: &str, heartbeat: bool) -> String {
    let prefix = format!("security scan ({gate}) ...");
    let result = format!("security scan ({gate}): {state}");
    let floor = if heartbeat {
        live_width(&prefix)
    } else {
        prefix.len() + 1
    };
    let pad = floor.saturating_sub(result.len()) + 1;
    format!("\r{result}{}", " ".repeat(pad))
}

#[cfg(test)]
#[path = "../tests/runner.rs"]
mod gates_runner_tests;
