//! PickerText: the printed picker for a terminal that cannot host the
//! overlay -- chat mode, or raw mode unavailable. Same rows, no cursor, so the
//! command still answers instead of returning empty-handed.

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
    let active = super::super::fleet::active(state_home);
    let ready = super::super::fleet::ready(harnesses, catalog_root, state_home);
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
