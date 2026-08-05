use super::{args::Options, error};
use std::io::{IsTerminal, Write};

const TOKEN: &str = "self-update:terminal-jarvis";

pub fn check(options: &Options, preview: &str) -> error::Result<()> {
    if options.allow_dangerous {
        return Err(error::Failure::usage(
            "option_not_applicable",
            "--allow-dangerous is not valid for self-update",
            "remove --allow-dangerous",
        ));
    }
    if options.dry_run {
        return Ok(());
    }
    let terminal = std::io::stdin().is_terminal();
    if let Some(actual) = options.confirm.as_deref() {
        if actual == TOKEN && (terminal || options.no_input) {
            return Ok(());
        }
        return Err(required());
    }
    if options.no_input || !terminal {
        return Err(required());
    }
    eprint!("{preview}Continue with {TOKEN}? [y/N] ");
    std::io::stderr().flush().map_err(prompt_error)?;
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .map_err(prompt_error)?;
    if matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
        Ok(())
    } else {
        Err(error::Failure::safety(
            "confirmation_declined",
            "self-update was not confirmed",
            "review the update plan and retry when ready",
        ))
    }
}

fn required() -> error::Failure {
    error::Failure::safety(
        "explicit_intent_required",
        format!("noninteractive self-update requires --no-input --confirm={TOKEN}"),
        format!("review --update --dry-run, then pass --no-input --confirm={TOKEN}"),
    )
}

fn prompt_error(cause: std::io::Error) -> error::Failure {
    error::Failure::state(
        "prompt_failed",
        cause.to_string(),
        "retry with --no-input and an exact --confirm token",
    )
}
