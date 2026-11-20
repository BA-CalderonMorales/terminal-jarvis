//! The retirement sentence shared by the plan card and the consent lead, so a
//! departing tool reads the same wherever someone is deciding to install it.

use crate::contracts::Harness;

/// `retires 2026-12-31 · replaced by antigravity`, or `None` while the tool is
/// maintained. `today` is a parameter so callers and tests pin the date.
pub fn retirement(harness: &Harness, today: &str) -> Option<String> {
    let record = harness.sunset.as_ref()?;
    let mut line = format!("{} {}", record.verb(today), record.at);
    if let Some(successor) = record.replaced_by.as_deref() {
        line.push_str(&format!(" · replaced by {successor}"));
    }
    Some(line)
}

#[cfg(test)]
#[path = "../tests/retirement_surfaces_test.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/retirement_list_test.rs"]
mod list_tests;
