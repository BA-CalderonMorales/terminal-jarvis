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

/// Opens the check: one streaming row under the frame, a narrated line, or
/// the rewrite-me dots line on a plain run.
pub fn announce(package: &str, options: &Options, quiet: bool, row: &mut dyn FnMut(&str)) {
    if quiet {
        row(&format!("checking {package} for known vulnerabilities ..."));
    } else if options.narrate {
        eprintln!("checking {package} for known vulnerabilities ...");
    } else {
        eprint!("package check ...");
    }
}

/// Reports a clean scan: one streaming row, a narrated line, or the
/// in-place rewrite of the dots line.
pub fn clean(package: &str, options: &Options, quiet: bool, row: &mut dyn FnMut(&str)) {
    if quiet {
        row(&format!(
            "package check: clean -- no HIGH/CRITICAL findings for {package}"
        ));
    } else if options.narrate {
        eprintln!("no HIGH/CRITICAL findings for {package}");
    } else {
        quiet_done(options, "clean");
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
