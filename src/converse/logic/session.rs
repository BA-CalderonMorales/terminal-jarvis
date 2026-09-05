//! Session: the live conversation state machine. One `advance` call runs
//! exactly one one-shot harness invocation, so the tui loop can repaint the
//! frame between turns and Ctrl-C can stop the current child cleanly.

use crate::converse::sanitize::clean;
use crate::converse::transcript::Transcript;

/// How a speaker is invoked. Production wires the headless runner + guard
/// path; tests inject a fake, so no test ever spawns a real child.
pub type Speaker<'a> = dyn FnMut(&str, &str) -> Result<String, String> + 'a;

/// The outcome of one invocation. A failure ends the session -- the user is
/// watching every turn, so retrying silently would hide what happened.
pub enum Step {
    Spoke,
    Stopped(String),
}

/// The live conversation: sides, topic, transcript, and remaining budget.
pub struct Live {
    pub a: String,
    pub b: String,
    pub topic: String,
    pub transcript: Transcript,
    pub turns_left: usize,
    pub next_is_a: bool,
    /// How many transcript turns the frame has already rendered.
    pub rendered: usize,
}

impl Live {
    pub fn new(a: &str, b: &str, topic: &str, turns: usize) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            topic: topic.to_string(),
            transcript: Transcript::new(a, b, topic),
            turns_left: turns,
            next_is_a: true,
            rendered: 0,
        }
    }

    pub fn speaker(&self) -> &str {
        if self.next_is_a {
            &self.a
        } else {
            &self.b
        }
    }

    pub fn other(&self) -> &str {
        if self.next_is_a {
            &self.b
        } else {
            &self.a
        }
    }
}

/// Runs one turn: prompt the current speaker, capture the reply, record it.
/// A failed invocation (binary gone, nonzero exit, Ctrl-C) freezes the
/// budget so the caller ends the session with a note.
pub fn advance(live: &mut Live, speak: &mut Speaker) -> Step {
    let speaker = live.speaker().to_string();
    let other = live.other().to_string();
    let prompt = match live.transcript.last_of(&other) {
        Some(last) => super::prompt::reply(&live.topic, &speaker, &other, last),
        None => super::prompt::seed(&live.topic, &speaker),
    };
    let outcome = speak(&speaker, &prompt);
    live.turns_left = live.turns_left.saturating_sub(1);
    match outcome {
        Ok(reply) => {
            live.transcript.push(&speaker, &clean(&reply));
            live.next_is_a = !live.next_is_a;
            Step::Spoke
        }
        Err(failure) => {
            live.turns_left = 0;
            Step::Stopped(failure)
        }
    }
}
