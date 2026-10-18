//! Ask: the question a consent gate is waiting on. It is shown at the prompt
//! row, so the cursor and the decision are the same place -- the user answers
//! where they are already looking, instead of reading the middle of the frame
//! and then finding the prompt again.

use std::sync::Mutex;

static ASK: Mutex<Option<String>> = Mutex::new(None);

/// The question a gate is waiting on, if any.
pub fn question() -> Option<String> {
    ASK.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Shows `text` at the prompt row until the returned guard drops.
pub fn show(text: &str) -> Ask {
    *ASK.lock().unwrap_or_else(|e| e.into_inner()) = Some(text.to_string());
    Ask
}

/// Clears the prompt row when the gate is done with it.
pub struct Ask;

impl Drop for Ask {
    fn drop(&mut self) {
        *ASK.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
}
