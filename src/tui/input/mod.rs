//! Input: the prompt widget surface. `Indicator` renders the context
//! prefix; the line readers live in `logic/`.

#[path = "logic/editor.rs"]
mod editor;
#[path = "logic/keys.rs"]
mod keys;
#[path = "logic/line.rs"]
mod line;

pub use editor::{Editor, Feed, Move};
pub use keys::read_key;
pub use line::{compose, raw_line, read_line, retire};

use crate::cli::style;

pub const PROMPT: &str = "[>_]";

const TJ_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Indicator {
    pub active: String,
    pub debug: bool,
}

impl Indicator {
    /// The prompt prefix: `[>_]::[tj:{v}]::[harness:{name}]:` + one space
    /// for the input area.
    pub fn render(&self, ansi: bool) -> String {
        let on = ansi && !style::plain();
        let debug = if self.debug {
            format!("::{}", painted("[debug]", on, style::dim))
        } else {
            String::new()
        };
        format!(
            "{}::[tj:{}]::[harness:{}]{}: ",
            PROMPT,
            painted(TJ_VERSION, on, style::dim),
            painted(&self.active, on, style::heading),
            debug
        )
    }

    /// Stable comparison form (colors never leak in).
    pub fn raw(&self) -> String {
        self.render(false).trim_end().to_string()
    }
}

fn painted(value: &str, on: bool, paint: fn(&str) -> String) -> String {
    if on {
        paint(value)
    } else {
        value.to_string()
    }
}

#[cfg(test)]
#[path = "../tests/input.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/input_keys.rs"]
mod keys_tests;
