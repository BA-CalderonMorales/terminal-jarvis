//! GuardAnswers: the wording of a consent answer -- the bracket, the decline,
//! and the key a streaming frame resolves. Split from `guard_ask` so the two
//! reading strategies (a terminal line, an in-frame key) sit beside the words
//! they print.

use super::error;

/// The streaming strategy's verdict for one decoded answer.
pub fn consent(answer: &str) -> error::Result<()> {
    if matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
        Ok(())
    } else {
        Err(declined())
    }
}

/// The bracket that tells the truth about the default: [Y/n] or [y/N].
pub fn bracket(default_yes: bool) -> &'static str {
    if default_yes {
        "[Y/n]"
    } else {
        "[y/N]"
    }
}

/// One raw key in-frame: the human row painted into the transcript and the
/// answer fed to [`consent`]. `default_yes` makes Enter confirm -- the
/// add/update direction (install, update) asks with [Y/n]; the destructive
/// direction (uninstall) asks with [y/N] and Enter declines.
pub fn in_frame(
    key: Option<crate::tui::input::Key>,
    default_yes: bool,
) -> (&'static str, &'static str) {
    match key {
        Some(crate::tui::input::Key::Char('y' | 'Y')) => ("confirmed", "y"),
        Some(crate::tui::input::Key::Char('n' | 'N')) => ("cancelled -- nothing was run", "n"),
        Some(crate::tui::input::Key::Enter) if default_yes => ("confirmed", "y"),
        _ => ("cancelled -- nothing was run", "n"),
    }
}

pub(super) fn declined() -> error::Failure {
    error::Failure::safety(
        "confirmation_declined",
        "cancelled; nothing was run",
        "review the plan and retry when ready",
    )
}

/// The confirm token mismatch: what a noninteractive run must pass.
pub(crate) fn confirm_error(token: &str) -> error::Failure {
    error::Failure::safety(
        "confirmation_required",
        format!("noninteractive execution requires --no-input --confirm={token}"),
        format!("review the plan, then pass --no-input --confirm={token}"),
    )
}
