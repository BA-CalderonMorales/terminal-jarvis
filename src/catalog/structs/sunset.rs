//! Retirement metadata: the facade-level pair that marks a tool as retired.
//! Kept out of the harness metadata reader so the lifecycle rule stays one
//! readable function next to the shape it produces.

use crate::catalog::logic::parser::{self, Fields};
use crate::contracts::Sunset;

/// Retirement is a pair plus an optional successor: the day the vendor
/// retired the tool, and what happened. A successor alone, or a date without
/// a note, is an incomplete story rather than a partial retirement.
pub fn parse(fields: &Fields) -> Result<Option<Sunset>, String> {
    let at = parser::optional_string(fields, "sunset_at")?;
    let note = parser::optional_string(fields, "sunset_note")?;
    let replaced_by = parser::optional_string(fields, "sunset_replaced_by")?;
    match (at, note) {
        (None, None) if replaced_by.is_none() => Ok(None),
        (Some(at), Some(note)) => {
            if at.len() != 10 || !crate::catalog::logic::freshness_clock::valid_date(&at) {
                return Err(format!("sunset_at must be a YYYY-MM-DD date, got {at}"));
            }
            if note.trim().is_empty() {
                return Err("sunset_note must say what happened".to_string());
            }
            Ok(Some(Sunset {
                at,
                note,
                replaced_by,
            }))
        }
        _ => Err("sunset_at and sunset_note must appear together".to_string()),
    }
}

#[cfg(test)]
#[path = "../tests/metadata_sunset_test.rs"]
mod tests;
