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
        }),
        args::Action::Update(Some(name)) => Some(Planned {
            invocation: one(name.clone(), Capability::Update),
            label: format!("update {name}"),
            lifecycle: Some((name.clone(), "updated")),
        }),
        args::Action::Run(words) => match resolve::run(words, harnesses, state_home) {
            Ok(invocation) => Some(Planned {
                invocation,
                label: words.join(" "),
                lifecycle: None,
            }),
            Err(message) => {
                state.body.push(format!("✗ {message}"));
                None
            }
        },
        args::Action::Direct { harness, extra } => {
            let mut invocation = one(harness.clone(), Capability::Headless);
            invocation.extra = extra.clone();
            Some(Planned {
                invocation,
                label: format!("{harness} {}", extra.join(" ")),
                lifecycle: None,
            })
        }
        _ => None,
    }
}

/// One mapped action: its invocation, its row label, and -- for install
/// and update -- the lifecycle pair the verdict card speaks.
pub struct Planned {
    pub invocation: Invocation,
    pub label: String,
    pub lifecycle: Option<(String, &'static str)>,
}
