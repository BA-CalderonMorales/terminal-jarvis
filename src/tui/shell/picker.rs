//! Picker: renders the numbered fleet picker with live readiness. The heading
//! stays the signpost; the rows and the tip come from the switcher, and a
//! filter narrows them without renumbering, so a number typed from the filtered
//! view still selects the tool it names.

use crate::contracts::Harness;
use crate::tui::screen::{paint, size, Role};
use std::io::Write;
use std::path::Path;

pub fn show(
    out: &mut dyn Write,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    filter: Option<&str>,
) {
    let active = super::fleet::active(state_home);
    let ready = super::fleet::ready(harnesses, catalog_root, state_home);
    let body = crate::tui::switcher::pick(
        harnesses,
        active.as_deref(),
        &ready,
        filter,
        size().inner_cols(),
    );
    let _ = writeln!(out, "{}", paint(Role::Accent, "Available Harnesses"));
    let _ = write!(out, "{body}");
}

/// The optional text after `list`, trimmed; empty means the whole fleet.
pub fn filter(rest: &str) -> Option<String> {
    let text = rest.trim_start_matches("list").trim();
    (!text.is_empty()).then(|| text.to_string())
}
