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
) -> Option<(Invocation, String)> {
    match action {
        args::Action::Install(Some(name)) => {
            let invocation = one(name.clone(), Capability::Download);
            Some((invocation, format!("install {name}")))
        }
        args::Action::Update(Some(name)) => {
            let invocation = one(name.clone(), Capability::Update);
            Some((invocation, format!("update {name}")))
        }
        args::Action::Run(words) => match resolve::run(words, harnesses, state_home) {
            Ok(invocation) => {
                let label = words.join(" ");
                Some((invocation, label))
            }
            Err(message) => {
                state.body.push(format!("✗ {message}"));
                None
            }
        },
        args::Action::Direct { harness, extra } => {
            let mut invocation = one(harness.clone(), Capability::Headless);
            invocation.extra = extra.clone();
            Some((invocation, format!("{harness} {}", extra.join(" "))))
        }
        _ => None,
    }
}
