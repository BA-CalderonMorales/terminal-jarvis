//! Session: the live conversation state machine. One `advance` call runs
//! exactly one one-shot harness invocation, so the tui loop can repaint the
//! frame between turns and Ctrl-C can stop the current child cleanly.

use crate::converse::sanitize::clean;
use crate::converse::transcript::Transcript;

/// How a speaker is invoked. Production wires the headless runner + guard
/// path; tests inject a fake, so no test ever spawns a real child.
pub type Speaker<'a> = dyn FnMut(&str, &str) -> Result<String, String> + 'a;

/// The live conversation: sides, topic, transcript, and remaining budget.
pub struct Live {
    pub a: String,
    pub b: String,
    pub topic: String,
    pub transcript: Transcript,
    pub turns_left: usize,
    pub next_is_a: bool,
}

pub const DEFAULT_TURNS: usize = 4;

impl Live {
    pub fn new(a: &str, b: &str, topic: &str) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            topic: topic.to_string(),
            transcript: Transcript::new(a, b, topic),
            turns_left: DEFAULT_TURNS,
            next_is_a: true,
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

/// The body lines one turn produces, plus whether the conversation ended.
pub struct Turned {
    pub lines: Vec<String>,
    pub over: bool,
}

/// Runs one turn: prompt the current speaker, capture the reply, record it.
/// A failed invocation (binary gone, nonzero exit, Ctrl-C) ends the session
/// with a note instead of retrying -- the user is watching every turn.
pub fn advance(live: &mut Live, speak: &mut Speaker) -> Turned {
    let speaker = live.speaker().to_string();
    let other = live.other().to_string();
    let prompt = match live.transcript.last_of(&other) {
        Some(last) => super::prompt::reply(&live.topic, &speaker, &other, last),
        None => super::prompt::seed(&live.topic, &speaker),
    };
    let outcome = speak(&speaker, &prompt);
    live.turns_left = live.turns_left.saturating_sub(1);
    let over = live.turns_left == 0;
    match outcome {
        Ok(reply) => {
            let reply = clean(&reply);
            live.transcript.push(&speaker, &reply);
            live.next_is_a = !live.next_is_a;
            let mut lines = live.transcript.lines();
            if over {
                lines.push(format!(
                    "── converse ended · {} turns ──",
                    live.transcript.turns.len()
                ));
            }
            Turned { lines, over }
        }
        Err(failure) => {
            live.turns_left = 0;
            let mut lines = live.transcript.lines();
            lines.push(format!("[{speaker}] stopped: {failure}"));
            lines.push("── converse ended early ──".to_string());
            Turned { lines, over: true }
        }
    }
}
