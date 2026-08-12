//! Report helpers for the live package-check line and advisory warnings.

use crate::cli::args::Options;
use crate::contracts::Capability;

pub fn quiet_done(options: &Options, state: &str) {
    if !options.narrate {
        let result = format!("package check: {state}");
        let pad = "package check ...".len().saturating_sub(result.len()) + 1;
        eprintln!("\r{result}{}", " ".repeat(pad));
    }
}

pub fn verb(capability: Capability) -> &'static str {
    if capability == Capability::Download {
        "installing"
    } else {
        "updating"
    }
}

pub fn warn_ok(message: &str) -> crate::cli::logic::error::Result<()> {
    eprintln!("warning: {message}");
    Ok(())
}
