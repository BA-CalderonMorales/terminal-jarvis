#[path = "logic/consent.rs"]
pub mod consent;

#[path = "logic/prompt.rs"]
mod prompt;

#[path = "logic/render.rs"]
pub mod render;

#[path = "logic/sanitize.rs"]
pub mod sanitize;

#[path = "logic/wire.rs"]
pub mod wire;

#[path = "logic/session.rs"]
mod session;

#[path = "structs/transcript.rs"]
pub mod transcript;

use crate::contracts::{Capability, Harness, SupportState};
pub use prompt::{reply, seed, WORD_CAP};
pub use session::{advance, Live, Step};
pub use transcript::Transcript;

pub const MAX_TURNS: usize = 12;

pub enum Parsed {
    Continue,
    Start {
        turns: usize,
        allow_dangerous: bool,
        a: String,
        b: String,
        topic: String,
    },
    Error(String),
}

pub fn parse(input: &str, harnesses: &[Harness]) -> Parsed {
    let mut words: Vec<&str> = input.split_whitespace().skip(1).collect();
    let allow_dangerous = words.first() == Some(&"--allow-dangerous");
    if allow_dangerous {
        words.remove(0);
    }
    match words.as_slice() {
        [] => Parsed::Continue,
        [turns, a, b, topic @ ..] if !topic.is_empty() => match turns.parse::<usize>() {
            Ok(turns) if (1..=MAX_TURNS).contains(&turns) => {
                for side in [*a, *b] {
                    if let Err(message) = headless_ready(harnesses, side) {
                        return Parsed::Error(message);
                    }
                }
                Parsed::Start {
                    turns,
                    allow_dangerous,
                    a: (*a).to_string(),
                    b: (*b).to_string(),
                    topic: topic.join(" "),
                }
            }
            _ => Parsed::Error(format!("converse turns must be 1..={MAX_TURNS}")),
        },
        _ => Parsed::Error("converse [--allow-dangerous] <turns> <a> <b> <topic...>".to_string()),
    }
}

#[cfg(test)]
#[path = "tests/session.rs"]
mod session_tests;

#[cfg(test)]
#[path = "tests/bubble.rs"]
mod bubble_tests;

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

/// A bounded conversation request, including its explicit dangerous opt-in.
pub type Seed = (usize, String, String, String, bool);
