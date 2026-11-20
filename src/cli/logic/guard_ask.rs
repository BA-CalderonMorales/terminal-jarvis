//! GuardAsk: the consent strategies. The terminal asks on stderr and reads
//! one line; the streaming surface resolves one key in-frame. The answer
//! words live in `guard_answers`.

use super::args::Options;
use super::error;

#[path = "guard_answers.rs"]
mod answers;
pub(crate) use answers::confirm_error;
pub use answers::{bracket, consent, in_frame};

/// The add/update direction defaults to yes; a destructive one to no.
pub fn default_yes(token: &str) -> bool {
    token.starts_with("download:") || token.starts_with("update:")
}

/// The terminal strategy: prompt on stderr, read one line from stdin. The lead
/// wraps to the REAL terminal width (the table width never reports below 40,
/// so a narrow window would be wrapped by the terminal instead of by us).
pub fn ask_in_terminal(lead: &str, token: &str, default_yes: bool) -> error::Result<()> {
    use std::io::Write;
    let width =
        crate::tui::term::size().map_or_else(super::table::terminal_width, |(cols, _)| cols);
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

/// Lifecycle options on a read-only capability are not applicable -- except
/// `--dry-run`, the "show me first" request, which runs nothing.
pub(crate) fn reject_irrelevant(options: &Options) -> error::Result<()> {
    if options.no_input || options.confirm.is_some() || options.allow_dangerous {
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
