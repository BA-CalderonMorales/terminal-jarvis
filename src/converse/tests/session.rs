//! Session contract: one call per turn, alternating speakers, a hard turn
//! cap, and failures that end the session with a note instead of retrying.

use crate::converse::render::{header, turns};
use crate::converse::sanitize::clean;
use crate::converse::{advance, Live, Step, MAX_TURNS};

/// A scripted speaker: tests see which side ran and in what order.
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
fn turns_alternate_sides_until_the_budget_is_spent() {
    let mut live = Live::new("opencode", "hermes", "workspace layout", 2);
    let mut speak = scripted(vec![Ok("alpha one".into()), Ok("bravo one".into())]);
    assert!(matches!(advance(&mut live, &mut speak), Step::Spoke));
    assert_eq!(live.transcript.turns[0].speaker, "opencode");
    assert!(matches!(advance(&mut live, &mut speak), Step::Spoke));
    assert_eq!(live.transcript.turns[1].speaker, "hermes");
    assert_eq!(live.turns_left, 0);
    assert!(matches!(advance(&mut live, &mut speak), Step::Stopped(_)));
}

#[test]
fn the_reply_prompt_carries_the_topic_and_the_other_reply() {
    let prompt = crate::converse::reply("workspace layout", "opencode", "hermes", "two crates");
    assert!(prompt.contains("workspace layout") && prompt.contains("two crates"));
    assert!(prompt.contains("80 words or fewer"));
}

#[test]
fn a_failed_invocation_freezes_the_budget() {
    let mut live = Live::new("opencode", "hermes", "topic", 4);
    let mut speak = scripted(vec![Err("exit 1".into())]);
    assert!(matches!(advance(&mut live, &mut speak), Step::Stopped(_)));
    assert_eq!(live.turns_left, 0);
}

#[test]
fn clean_strips_ansi_and_controls_but_keeps_text() {
    assert_eq!(clean("\x1b[0mREADY\r\n"), "READY");
    assert_eq!(clean("a\x1b[1;32mb"), "ab");
    assert_eq!(clean("keep ✓ wide"), "keep ✓ wide");
}

#[test]
fn bubbles_put_the_first_harness_left_and_replies_right() {
    let mut transcript = crate::converse::Transcript::new("opencode", "hermes", "t");
    transcript.push("opencode", "hi there");
    transcript.push("hermes", "hello");
    let lines = turns(&transcript, 0, 60);
    let top = lines
        .iter()
        .find(|line: &&String| line.contains("╭─") && line.contains("opencode"))
        .unwrap();
    assert!(
        top.contains("╭─") && top.contains("opencode"),
        "the opener bubble names its speaker"
    );
    let reply = lines
        .iter()
        .find(|line: &&String| line.contains("╭─") && line.contains("hermes"))
        .unwrap();
    assert!(reply.starts_with(' '), "reply bubble indents right");
    assert!(header(&transcript).iter().any(|line| line == "topic: t"));
}

#[test]
fn bubble_wrapping_keeps_whole_words_inside_the_box() {
    let mut transcript = crate::converse::Transcript::new("a", "b", "t");
    transcript.push("a", &"word ".repeat(40));
    let lines = turns(&transcript, 0, 60);
    let boxed: Vec<&String> = lines.iter().filter(|line| line.contains('│')).collect();
    assert!(boxed.len() > 1, "long text wraps into multiple box rows");
    for row in &boxed {
        let visible = crate::tui::screen::visible_width(row);
        assert!(visible <= 60, "row {visible} cells overflows: {row}");
        let text = row.replace("\u{1b}", "^");
        assert!(
            text.contains("word word") || text.contains("│ word"),
            "no mid-word splits: {row}"
        );
    }
}

#[test]
fn the_turn_budget_ends_the_session_after_the_selected_turns() {
    let mut live = Live::new("a", "b", "t", MAX_TURNS);
    let replies: Vec<_> = (0..MAX_TURNS).map(|turn| Ok(turn.to_string())).collect();
    let mut speak = scripted(replies);
    for expected in (0..MAX_TURNS).rev() {
        assert!(matches!(advance(&mut live, &mut speak), Step::Spoke));
        assert_eq!(live.turns_left, expected);
    }
    assert!(matches!(advance(&mut live, &mut speak), Step::Stopped(_)));
}
