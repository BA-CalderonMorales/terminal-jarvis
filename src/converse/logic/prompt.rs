//! Prompt protocol: what each harness is asked per turn. Kept short and
//! one-shot; the word cap keeps the token bill visible and bounded.

pub const WORD_CAP: &str = "80 words or fewer";

/// The opening turn: speaker opens the discussion on the topic.
pub fn seed(topic: &str, speaker: &str) -> String {
    format!(
        "You are {speaker}, in a two-agent terminal conversation. \
Topic: {topic}. Open the discussion in {WORD_CAP}. No tool use."
    )
}

/// A responding turn: speaker answers the other side's latest reply.
pub fn reply(topic: &str, speaker: &str, other: &str, last: &str) -> String {
    format!(
        "You are {speaker}, in a two-agent terminal conversation about: {topic}. \
{other} said: \"{last}\" Respond directly in {WORD_CAP}. No tool use."
    )
}
