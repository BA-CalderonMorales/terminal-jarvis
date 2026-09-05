//! Converse: two coding harnesses talking to each other in a tui tab. Each
//! turn is one one-shot headless invocation (`opencode run`, `hermes -z`),
//! captured and painted into the frame between turns so the exchange is
//! watchable and Ctrl-C always stops the current child, never the session
//! scaffold. The token bill is explicit: a fixed turn cap per conversation.

#[path = "logic/consent.rs"]
pub mod consent;

#[path = "logic/prompt.rs"]
mod prompt;

#[path = "logic/sanitize.rs"]
pub mod sanitize;

#[path = "logic/wire.rs"]
pub mod wire;

#[path = "logic/session.rs"]
mod session;

#[path = "structs/transcript.rs"]
pub mod transcript;

pub use prompt::{reply, seed, WORD_CAP};
pub use session::{advance, Live, Turned, DEFAULT_TURNS};
pub use transcript::Transcript;

#[cfg(test)]
#[path = "tests/session.rs"]
mod session_tests;

use crate::contracts::{Capability, Harness, SupportState};

/// Seed-time validation: the harness must exist and expose a real (non-stub)
/// headless plan, or the conversation cannot hold both sides up.
pub fn headless_ready(harnesses: &[Harness], name: &str) -> Result<(), String> {
    let Some(harness) = harnesses.iter().find(|harness| harness.name == name) else {
        return Err(format!(
            "unknown harness '{name}'; use list to see the fleet"
        ));
    };
    let Some(plan) = harness.plan(Capability::Headless) else {
        return Err(format!("{name} has no headless plan"));
    };
    if plan.support == SupportState::Stub {
        return Err(format!(
            "{name}:headless is a stub; it cannot hold up its side of a conversation"
        ));
    }
    Ok(())
}
