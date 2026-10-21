//! Ask: the question a consent gate is waiting on. It is shown at the prompt
//! row, so the cursor and the decision are the same place -- the user answers
//! where they are already looking, instead of reading the middle of the frame
//! and then finding the prompt again.

use std::sync::Mutex;

static ASK: Mutex<Option<(String, String)>> = Mutex::new(None);

/// The prompt row for a pending decision, fitted to `inner`: the full
/// question, or the short form when the frame cannot hold it. The bracket has
/// to survive either way, or the user cannot see what Enter will do.
pub fn prompt_for(inner: usize) -> Option<String> {
    let ask = ASK.lock().unwrap_or_else(|e| e.into_inner());
    let (full, short) = ask.as_ref()?;
    let fits = crate::tui::screen::visible_width(full) <= inner;
    Some(if fits { full.clone() } else { short.clone() })
}

/// Shows `full` at the prompt row -- `short` when the frame is too narrow --
/// until the returned guard drops.
pub fn show(full: &str, short: &str) -> Ask {
    *ASK.lock().unwrap_or_else(|e| e.into_inner()) = Some((full.to_string(), short.to_string()));
    Ask
}

/// Clears the prompt row when the gate is done with it.
pub struct Ask;

impl Drop for Ask {
    fn drop(&mut self) {
        *ASK.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
}
