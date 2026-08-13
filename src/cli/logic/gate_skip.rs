//! Skip consent: an interrupted security scan may be consciously skipped by
//! an interactive user so a slow scan can never hold an install, update, or
//! run hostage; piped and --no-input runs always abort because there is no
//! one to ask. The decision is a pure function so the consent matrix is
//! property-tested.

use super::{args::Options, error};
use crate::gates::{interrupted_message, Verdict};
use std::io::{IsTerminal, Write};

/// Routes a finished scan: passes move on, blocks refuse with the findings
/// message, interruptions ask the interactive user for conscious consent.
pub fn route(options: &Options, verdict: Verdict, target: &str) -> error::Result<()> {
    match verdict {
        Verdict::Passed => Ok(()),
        Verdict::Blocked(message) => Err(gate_failure("gate_blocked", message)),
        Verdict::Interrupted { gate } => {
            if consent(options, target)? {
                Ok(())
            } else {
                Err(gate_failure("gate_interrupted", interrupted_message(&gate)))
            }
        }
    }
}

/// The pure decision: a scan may be skipped only on a real terminal that is
/// not in --no-input mode, and only when the user says yes.
pub fn allow(no_input: bool, promptable: bool, confirmed: bool) -> bool {
    promptable && !no_input && confirmed
}

/// Asks the interactive user whether to continue without the scan. Returns
/// Ok(false) without prompting when no one is there to answer safely.
pub fn consent(options: &Options, target: &str) -> error::Result<bool> {
    let promptable = std::io::stdin().is_terminal();
    if options.no_input || !promptable {
        return Ok(false);
    }
    eprint!("Security scan interrupted. Skip the scan and continue with {target}? [y/N] ");
    std::io::stderr().flush().map_err(prompt_failed)?;
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .map_err(prompt_failed)?;
    let confirmed = matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes");
    Ok(allow(options.no_input, promptable, confirmed))
}

fn gate_failure(code: &'static str, message: String) -> error::Failure {
    error::Failure::safety(code, message, "run `terminal-jarvis gate status`")
}

fn prompt_failed(cause: std::io::Error) -> error::Failure {
    error::Failure::state(
        "gate_skip_prompt_failed",
        format!("cannot read the skip answer: {cause}"),
        "retry the command",
    )
}

#[cfg(test)]
#[path = "../tests/gate_skip.rs"]
mod gate_skip_tests;
