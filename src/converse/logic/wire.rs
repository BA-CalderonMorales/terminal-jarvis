//! Wire: the tui-side application of the conversation state machine. The
//! shell loop calls `pending` once per frame; each call runs at most one
//! one-shot turn so the frame repaints between every exchange.

use super::render;
use super::session::{advance, Live, Speaker, Step};
use std::path::Path;

/// First-run gate + opener: `Ok` pairs the live session with its opening
/// bubble body; `Err` shows the warning once and asks for the re-run.
pub fn open(
    seed: Option<(usize, String, String, String)>,
    state_home: &Path,
    width: usize,
) -> Result<(Live, Vec<String>), Vec<String>> {
    let (turns, a, b, topic) =
        seed.ok_or_else(|| vec!["no conversation is running".to_string()])?;
    if super::consent::seen(state_home) {
        let live = Live::new(&a, &b, &topic, turns);
        let lines = render::bubbles(&live.transcript, width);
        Ok((live, lines))
    } else {
        super::consent::mark(state_home);
        let mut lines = vec![
            format!("── converse: {a} ⇄ {b} ──"),
            format!("topic: {topic}"),
            String::new(),
        ];
        lines.extend(super::consent::opening_lines(false));
        Err(lines)
    }
}

/// Runs at most one turn for the active session; `None` when no session is
/// live or the budget is spent. The returned lines are the body delta.
pub fn pending(
    live: &mut Option<Live>,
    speak: &mut Speaker<'_>,
    width: usize,
) -> Option<Vec<String>> {
    let active = live.as_mut()?;
    if active.turns_left == 0 {
        return None;
    }
    let speaker = active.speaker().to_string();
    match advance(active, speak) {
        Step::Stopped(failure) => {
            let mut lines = render::bubbles(&active.transcript, width);
            lines.push(format!("[{speaker}] stopped: {failure}"));
            lines.push("── converse ended early ──".to_string());
            *live = None;
            Some(lines)
        }
        Step::Spoke => {
            let mut lines = render::bubbles(&active.transcript, width);
            if active.turns_left == 0 {
                let count = active.transcript.turns.len();
                lines.push(format!("── converse ended · {count} turns ──"));
                *live = None;
            }
            Some(lines)
        }
    }
}

/// The modeline hint while a conversation is live.
pub fn hint(live: &Option<Live>) -> Option<String> {
    let live = live.as_ref()?;
    Some(format!(
        "converse {}⇄{} · {} turns left · experimental",
        live.transcript.a, live.transcript.b, live.turns_left
    ))
}
