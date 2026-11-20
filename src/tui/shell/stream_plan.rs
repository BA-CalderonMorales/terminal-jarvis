//! StreamPlan: maps a tui action onto its headless invocation, so the
//! streaming runner stays generic over what it executes.

use crate::cli::args;
use crate::cli::resolve::{self, Invocation};
use crate::contracts::{Capability, Harness};
use std::path::Path;

/// Maps the action onto an invocation; bare active-harness forms were
/// normalized in handle. `None` means the error was already reported.
fn one(harness: String, capability: Capability) -> Invocation {
    Invocation {
        harness,
        capability,
        extra: vec![],
    }
}
pub fn for_action(
    action: &args::Action,
    state: &mut super::state::LoopState,
    harnesses: &[Harness],
    state_home: &Path,
) -> Option<Planned> {
    match action {
        args::Action::Install(Some(name)) => Some(Planned {
            invocation: one(name.clone(), Capability::Download),
            label: format!("install {name}"),
            lifecycle: Some((name.clone(), "installed")),
            explicit: true,
        }),
        args::Action::Update(Some(name)) => Some(Planned {
            invocation: one(name.clone(), Capability::Update),
            label: format!("update {name}"),
            lifecycle: Some((name.clone(), "updated")),
            explicit: true,
        }),
        args::Action::Run(words) => match resolve::run(words, harnesses, state_home) {
            Ok(invocation) => Some(Planned {
                lifecycle: postcondition(&invocation),
                invocation,
                label: words.join(" "),
                explicit: crate::cli::explicit_capability(words, harnesses),
            }),
            Err(message) => {
                state.body.push(format!("✗ {message}"));
                None
            }
        },
        args::Action::Direct { harness, extra } => {
            match resolve::direct(harness, extra, harnesses) {
                Ok(invocation) => Some(Planned {
                    lifecycle: postcondition(&invocation),
                    invocation,
                    label: format!("{harness} {}", extra.join(" ")),
                    explicit: false,
                }),
                Err(message) => {
                    state.body.push(format!("✗ {message}"));
                    None
                }
            }
        }
        _ => None,
    }
}

/// One mapped action: its invocation, its row label, and -- for install
/// and update -- the lifecycle pair the verdict card speaks.
pub struct Planned {
    pub invocation: Invocation,
    /// Original request intent, matching the blocking guard path.
    pub explicit: bool,
    pub label: String,
    pub lifecycle: Option<(String, &'static str)>,
}

#[cfg(test)]
#[path = "../tests/stream_plan.rs"]
mod tests;

// Only the top-level install adopts a harness; run/direct merely verify it.
fn postcondition(invocation: &Invocation) -> Option<(String, &'static str)> {
    let verb = match invocation.capability {
        Capability::Download => "downloaded",
        Capability::Update => "updated",
        _ => return None,
    };
    Some((invocation.harness.clone(), verb))
}
