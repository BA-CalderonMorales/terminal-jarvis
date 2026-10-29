//! Filter: what a text filter keeps, and how the picker explains the result.
//! A name or a description phrase both match, so "openai" finds codex and
//! "pair programming" finds the assistants that say so. A narrowed list keeps
//! the original picker numbers -- they stay the switch keys -- and the tip
//! states the count and that promise.

use crate::contracts::Harness;

pub fn matches(harness: &Harness, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    harness.name.to_lowercase().contains(needle)
        || harness.description.to_lowercase().contains(needle)
}

pub fn tip(needle: &str, shown: usize, total: usize, filtered: bool) -> String {
    if !filtered {
        return crate::cli::style::dim(
            "pick a number to switch agents · list <text> filters · home resets",
        );
    }
    if shown == 0 {
        return crate::cli::style::warning(&format!(
            "no harness matches \"{needle}\" · list shows all {total}"
        ));
    }
    crate::cli::style::dim(&format!(
        "{shown} of {total} tools match \"{needle}\" · numbers are stable picker positions"
    ))
}
