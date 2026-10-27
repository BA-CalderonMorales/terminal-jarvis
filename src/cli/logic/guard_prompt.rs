//! In-frame consent shared by streaming commands and conversation turns.
use super::{error, guard_ask};
pub fn ask(
    paint: &mut dyn FnMut(&str),
    lead: &str,
    token: &str,
    default_yes: bool,
) -> error::Result<()> {
    let question = format!("Continue with {token}? {}", guard_ask::bracket(default_yes));
    let short = format!("Continue? {}", guard_ask::bracket(default_yes));
    let _ask = crate::tui::input::ask(&question, &short);
    for line in lead.lines() {
        paint(line);
    }
    let _raw = crate::tui::term::enable_raw();
    let mut key = crate::tui::input::read_key();
    while matches!(key, Some(crate::tui::input::Key::Resized)) {
        paint("");
        key = crate::tui::input::read_key();
    }
    crate::tui::input::drain_answer(std::time::Duration::from_millis(150));
    let (row, answer) = guard_ask::in_frame(key, default_yes);
    paint(row);
    guard_ask::consent(answer)
}
