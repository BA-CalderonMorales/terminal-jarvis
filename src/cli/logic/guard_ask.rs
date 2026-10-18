//! GuardAsk: the consent strategies. The terminal asks on stderr and reads
//! one line; the streaming surface resolves one key in-frame. Both funnel
//! into the same answer grammar, so the consent matrix stays one truth.

use super::args::Options;
use super::error;

/// The add/update direction defaults to yes; a destructive one to no.
pub fn default_yes(token: &str) -> bool {
    token.starts_with("download:") || token.starts_with("update:")
}

/// The terminal strategy: prompt on stderr, read one line from stdin. The lead
/// wraps to the terminal so a narrow window never wraps a line onto the next,
/// and the bracket is the real default -- Enter means what it says here too.
pub fn ask_in_terminal(lead: &str, token: &str, default_yes: bool) -> error::Result<()> {
    use std::io::Write;
    let width = super::table::terminal_width();
    for line in crate::cli::wrap(lead.trim_end(), width) {
        eprintln!("{line}");
    }
    let full = format!("Continue with {token}? {}", bracket(default_yes));
    let short = format!("Continue? {}", bracket(default_yes));
    let cells: usize = full.chars().map(crate::cli::char_cells).sum();
    let question = if cells <= width { full } else { short };
    eprintln!("{question}");
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

/// The bracket that tells the truth about the default: [Y/n] or [y/N].
pub fn bracket(default_yes: bool) -> &'static str {
    if default_yes {
        "[Y/n]"
    } else {
        "[y/N]"
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
