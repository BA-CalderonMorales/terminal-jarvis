//! Rows: one picker line -- stable index, name, readiness chip, description.
//! The chip is padded to its widest form so names and descriptions line up
//! whether a tool is ready to run or still one install away. When the frame is
//! too narrow to carry a useful description, the row drops it rather than
//! clipping the verdict.

use super::clip;
use crate::contracts::Harness;

/// Cells reserved for the widest chip, so columns hold across both verdicts.
pub const CHIP: usize = 9;
/// Columns the active marker occupies, including the gap before it.
pub const MARKER: usize = 11;
/// A description narrower than this reads as noise; the row drops it instead.
pub const MIN_DESCRIPTION: usize = 12;

pub fn row(
    index: usize,
    name_width: usize,
    harness: &Harness,
    chip: &str,
    room: Option<usize>,
    active: bool,
) -> String {
    let marker = if active { "  (current)" } else { "" };
    let description = match room {
        Some(room) => format!("  {}", clip(&harness.description, room)),
        None => String::new(),
    };
    format!(
        "{index:>2}  {name:<name_width$}  {chip:<CHIP$}{description}{marker}",
        name = harness.name,
        chip = chip,
    )
}
