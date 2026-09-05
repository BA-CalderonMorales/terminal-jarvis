//! Wire: the tui-side application of the conversation state machine. The
//! shell loop calls `pending` once per frame; each call runs at most one
//! one-shot turn so the frame repaints between every exchange.

use super::session::{advance, Live, Speaker};
use std::path::Path;

/// Seeds a conversation (post-consent) and returns its opening body lines.
pub fn seed(a: &str, b: &str, topic: &str) -> Live {
    Live::new(a, b, topic)
}

/// Runs at most one turn for the active session; `None` when no session is
/// live or the budget is spent. The returned lines are the body delta.
pub fn pending(live: &mut Option<Live>, speak: &mut Speaker<'_>) -> Option<Vec<String>> {
    let active = live.as_mut()?;
    if active.turns_left == 0 {
        return None;
    }
    let turned = advance(active, speak);
    if turned.over {
        *live = None;
    }
    Some(turned.lines)
}

/// The modeline hint while a conversation is live.
pub fn hint(live: &Option<Live>) -> Option<String> {
    let live = live.as_ref()?;
    Some(format!(
        "converse {}⇄{} · {} turns left · experimental",
        live.transcript.a, live.transcript.b, live.turns_left
    ))
}

/// First-run gate: `Ok(live)` seeds; `Err(lines)` shows the warning once.
pub fn consent(
    seed: Option<(String, String, String)>,
    state_home: &Path,
) -> Result<Live, Vec<String>> {
    let (a, b, topic) = seed.ok_or_else(|| vec!["no conversation is running".to_string()])?;
    let live = Live::new(&a, &b, &topic);
    if super::consent::seen(state_home) {
        Ok(live)
    } else {
        super::consent::mark(state_home);
        let mut lines = vec![
            format!("── converse: {a} ⇄ {b} ──"),
            format!("topic: {topic}"),
        ];
        lines.extend(super::consent::opening_lines(false));
        Err(lines)
    }
}
