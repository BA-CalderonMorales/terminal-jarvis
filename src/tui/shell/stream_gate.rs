//! StreamGate: which actions flow through the in-frame streaming runner,
//! and the bare-verb normalization that speaks for the active harness.

use crate::cli::args;
use std::path::Path;

/// Bare `install`/`update` speak for the active harness; `None` with no
/// active selection is a friendly error, not a fatal one.
pub fn normalized(action: args::Action, state_home: &Path) -> Option<args::Action> {
    match action {
        args::Action::Install(None) | args::Action::Update(None) => {
            let name = crate::context::load(state_home)
                .ok()
                .flatten()?
                .active_harness;
            Some(match action {
                args::Action::Install(None) => args::Action::Install(Some(name)),
                _ => args::Action::Update(Some(name)),
            })
        }
        other => Some(other),
    }
}

/// In-frame streaming: one-shot invocations (installs, updates, prompted
/// runs). Interactive agent uis keep the suspended full-terminal frame.
pub fn eligible(
    action: &args::Action,
    harnesses: &[crate::contracts::Harness],
    home: &Path,
) -> bool {
    use crate::contracts::{Capability, Interaction};
    let invocation = match action {
        args::Action::Install(_) | args::Action::Update(_) => return true,
        args::Action::Run(words) => crate::cli::resolve::run(words, harnesses, home),
        args::Action::Direct { harness, extra } if !extra.is_empty() => {
            crate::cli::resolve::direct(harness, extra, harnesses)
        }
        _ => return false,
    };
    invocation
        .ok()
        .and_then(|invocation| {
            harnesses
                .iter()
                .find(|h| h.name == invocation.harness)
                .and_then(|h| h.plan(invocation.capability))
        })
        .is_some_and(|plan| {
            plan.capability != Capability::Ui && plan.interaction != Interaction::Interactive
        })
}

#[cfg(test)]
#[path = "../tests/stream_gate.rs"]
mod tests;
