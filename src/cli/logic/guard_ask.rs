//! GuardAsk: the consent strategies. The terminal asks on stderr and reads
//! one line; the streaming surface resolves one key in-frame. Both funnel
//! into the same answer grammar, so the consent matrix stays one truth.

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

fn prompt_failed(cause: std::io::Error) -> error::Failure {
    error::Failure::state(
        "prompt_failed",
        cause.to_string(),
        "retry with --no-input and --confirm",
    )
}
