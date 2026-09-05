//! GuardAsk: the consent strategies. The terminal asks on stderr and reads
//! one line; the streaming surface resolves one key in-frame. Both funnel
//! into the same answer grammar, so the consent matrix stays one truth.

use super::args::Options;
use super::error;

/// The terminal strategy: prompt on stderr, read one line from stdin.
pub fn ask_in_terminal(lead: &str, token: &str) -> error::Result<()> {
    use std::io::Write;
    eprint!("{lead}");
    eprint!("Continue with {token}? [y/N] ");
    std::io::stderr().flush().map_err(prompt_failed)?;
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .map_err(prompt_failed)?;
    consent(answer.trim())
}

/// The streaming strategy's verdict for one decoded answer.
pub fn consent(answer: &str) -> error::Result<()> {
    if matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
        Ok(())
    } else {
        Err(error::Failure::safety(
            "confirmation_declined",
            "cancelled; nothing was run",
            "review the plan and retry when ready",
        ))
    }
}

/// One raw key in-frame: the human row painted into the transcript and the
/// answer fed to [`consent`] -- the same y/N grammar the terminal asks in.
pub fn in_frame(key: Option<crate::tui::input::Key>) -> (&'static str, &'static str) {
    match key {
        Some(crate::tui::input::Key::Char('y' | 'Y')) => ("confirmed", "y"),
        _ => ("cancelled -- nothing was run", "n"),
    }
}

/// The confirm token mismatch: what a noninteractive run must pass.
pub(crate) fn confirm_error(token: &str) -> error::Failure {
    error::Failure::safety(
        "confirmation_required",
        format!("noninteractive execution requires --no-input --confirm={token}"),
        format!("review the plan, then pass --no-input --confirm={token}"),
    )
}

/// Lifecycle options on a read-only capability are never applicable.
pub(crate) fn reject_irrelevant(options: &Options) -> error::Result<()> {
    if options.dry_run || options.no_input || options.confirm.is_some() || options.allow_dangerous {
        return Err(error::Failure::usage(
            "option_not_applicable",
            "lifecycle options are not valid for a read-only capability",
            "remove the lifecycle option",
        ));
    }
    Ok(())
}

fn prompt_failed(cause: std::io::Error) -> error::Failure {
    error::Failure::state(
        "prompt_failed",
        cause.to_string(),
        "retry with --no-input and --confirm",
    )
}
