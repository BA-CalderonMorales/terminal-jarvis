//! Session contract: one call per turn, alternating speakers, a hard turn
//! cap, and failures that end the session with a note instead of retrying.

use crate::converse::sanitize::clean;
use crate::converse::{advance, Live, DEFAULT_TURNS};

/// A speaker that always answers with its script prefix, so tests can see
/// exactly which side ran and in what order.
fn scripted(
    replies: Vec<Result<String, String>>,
) -> impl FnMut(&str, &str) -> Result<String, String> {
    let mut replies = replies.into_iter();
    move |_speaker: &str, _prompt: &str| {
        replies
            .next()
            .unwrap_or_else(|| Err("script exhausted".to_string()))
    }
}

#[test]
fn turns_alternate_sides_and_honour_the_word_cap_in_prompts() {
    let mut live = Live::new("opencode", "hermes", "workspace layout");
    let mut speak = scripted(vec![Ok("alpha one".into()), Ok("bravo one".into())]);
    let first = advance(&mut live, &mut speak);
    assert!(first
        .lines
        .iter()
        .any(|line| line == "[opencode] alpha one"));
    assert!(!first.over);
    let second = advance(&mut live, &mut speak);
    assert!(second.lines.iter().any(|line| line == "[hermes] bravo one"));
    assert!(!second.over, "the default budget is {DEFAULT_TURNS} turns");
}

#[test]
fn the_reply_prompt_carries_the_topic_and_the_other_reply() {
    let mut live = Live::new("opencode", "hermes", "workspace layout");
    live.transcript.push("hermes", "start with two crates");
    let prompt = crate::converse::reply(&live.topic, "opencode", "hermes", "start with two crates");
    assert!(prompt.contains("workspace layout"));
    assert!(prompt.contains("start with two crates"));
    assert!(prompt.contains("80 words or fewer"));
}

#[test]
fn a_failed_invocation_ends_the_session_with_a_note() {
    let mut live = Live::new("opencode", "hermes", "topic");
    let mut speak = scripted(vec![Err("exit 1".into())]);
    let turn = advance(&mut live, &mut speak);
    assert!(turn.over);
    assert!(turn
        .lines
        .iter()
        .any(|line| line.contains("[opencode] stopped: exit 1")));
    assert!(turn.lines.iter().any(|line| line.contains("ended early")));
}

#[test]
fn clean_strips_ansi_and_controls_but_keeps_text() {
    assert_eq!(clean("\x1b[0mREADY\r\n"), "READY");
    assert_eq!(clean("a\x1b[1;32mb"), "ab");
    assert_eq!(clean("keep ✓ wide"), "keep ✓ wide");
}

#[test]
fn transcript_lines_carry_badges_and_the_chapter_header() {
    let mut transcript = crate::converse::Transcript::new("opencode", "hermes", "t");
    transcript.push("opencode", "hi");
    let lines = transcript.lines();
    assert!(lines[0].contains("opencode ⇄ hermes"));
    assert!(lines.iter().any(|line| line == "topic: t"));
    assert!(lines.iter().any(|line| line == "[opencode] hi"));
}

#[test]
fn the_turn_budget_ends_the_session_after_the_default() {
    let mut live = Live::new("a", "b", "t");
    let mut speak = scripted(vec![
        Ok("1".into()),
        Ok("2".into()),
        Ok("3".into()),
        Ok("4".into()),
    ]);
    for expected in [false, false, false, true] {
        let turn = advance(&mut live, &mut speak);
        assert_eq!(turn.over, expected);
    }
    assert_eq!(live.transcript.turns.len(), DEFAULT_TURNS);
}
