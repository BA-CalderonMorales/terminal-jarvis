//! The canonical-mode line widget (keystrokes land after Enter; std has no
//! raw mode): `compose` draws the hint *below* the prompt, `retire` clears
//! it, results print above -- a chat-style layout. The prompt carries the
//! context indicator `[>_]::[tj:0.1.13]::[harness:codex]:` (names bold cyan,
//! versions dim -- ANSI-16 core, readable on any theme).
use super::term;
use crate::cli::style;
use std::io::{self, Write};

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

pub fn compose(ansi: bool, indicator: &Indicator, hint: &str) -> String {
    let prefix = indicator.render(ansi);
    if !ansi {
        return format!("{prefix}{hint}\n{prefix}");
    }
    format!("{prefix}\n{}\x1b[1A\r{prefix}", style::dim(hint))
}

pub fn retire(text: &str, ansi: bool, hint: &str, indicator: &Indicator) -> String {
    if !ansi {
        return if text.is_empty() {
            compose(false, indicator, hint)
        } else {
            "\n".to_string()
        };
    }
    if text.is_empty() {
        return format!("\r\x1b[2K{}", indicator.render(ansi));
    }
    "\x1b[1B\x1b[2K\x1b[1A\n".to_string()
}

pub fn read_line(indicator: &Indicator, hint: &str) -> Option<String> {
    let ansi = term::ansi_enabled();
    print!("{}", compose(ansi, indicator, hint));
    io::stdout().flush().ok()?;
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => {
            print!("{}", retire("", ansi, hint, indicator));
            io::stdout().flush().ok()?;
            None
        }
        Ok(_) => {
            let text = line.trim_end_matches(['\n', '\r']).to_string();
            print!("{}", retire(&text, ansi, hint, indicator));
            io::stdout().flush().ok()?;
            Some(text)
        }
    }
}

#[cfg(test)]
#[path = "../tests/input.rs"]
mod tests;
