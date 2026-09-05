//! Transcript: the two-agent exchange record. One turn per harness reply,
//! rendered as badge-prefixed body lines for the centered paint pass.

/// One harness reply inside a conversation.
pub struct Turn {
    pub speaker: String,
    pub text: String,
}

/// The exchange so far: both sides, the topic, and every turn in order.
pub struct Transcript {
    pub a: String,
    pub b: String,
    pub topic: String,
    pub turns: Vec<Turn>,
}

impl Transcript {
    pub fn new(a: &str, b: &str, topic: &str) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            topic: topic.to_string(),
            turns: Vec::new(),
        }
    }

    pub fn push(&mut self, speaker: &str, text: &str) {
        self.turns.push(Turn {
            speaker: speaker.to_string(),
            text: text.to_string(),
        });
    }

    /// The most recent reply from `speaker`, for the reply-side prompt.
    pub fn last_of(&self, speaker: &str) -> Option<&str> {
        self.turns
            .iter()
            .rev()
            .find(|turn| turn.speaker == speaker)
            .map(|turn| turn.text.as_str())
    }

    /// Body lines: one chapter header, the topic, then badge-prefixed turns.
    pub fn lines(&self) -> Vec<String> {
        let mut lines = vec![format!("── converse: {} ⇄ {} ──", self.a, self.b)];
        lines.push(format!("topic: {}", self.topic));
        for turn in &self.turns {
            lines.push(format!("[{}] {}", turn.speaker, turn.text));
        }
        lines
    }
}
