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
pub fn eligible(action: &args::Action) -> bool {
    match action {
        args::Action::Install(_) | args::Action::Update(_) => true,
        args::Action::Run(words) => words.len() > 1,
        args::Action::Direct { extra, .. } => !extra.is_empty(),
        _ => false,
    }
}
