//! Input: the canonical-mode line widget. Because std has no raw-mode or
//! termios access, keystrokes are only visible after Enter; the widget
//! therefore renders the hint on its own line *below* the prompt, so the
//! type echo never collides with it. `compose` draws the two-line box and
//! parks the cursor on the prompt; `retire` clears the box once a line
//! commits, leaving the committed line in scrollback so result output is
//! printed above the next box -- a chat-style layout, fully dependency-free.
//! The hint is caller-owned: the shell passes a live modeline (active agent,
//! next action) instead of a fixed waiting message.

use super::term;
use crate::cli::style;
use std::io::{self, Write};

pub const PROMPT: &str = "[>_] ";

pub fn compose(ansi: bool, hint: &str) -> String {
    if !ansi {
        return format!("{PROMPT}{hint}\n{PROMPT}");
    }
    format!("{PROMPT}\n{}\x1b[1A\r{PROMPT}", style::dim(hint))
}

pub fn retire(text: &str, ansi: bool, hint: &str) -> String {
    if !ansi {
        return if text.is_empty() {
            compose(false, hint)
        } else {
            "\n".to_string()
        };
    }
    if text.is_empty() {
        return format!("\r{PROMPT}\n{}\x1b[1A\r{PROMPT}", style::dim(hint));
    }
    "\x1b[1B\x1b[2K\x1b[1A\n".to_string()
}

pub fn read_line(hint: &str) -> Option<String> {
    let ansi = term::ansi_enabled();
    print!("{}", compose(ansi, hint));
    io::stdout().flush().ok()?;
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => {
            print!("{}", retire("", ansi, hint));
            io::stdout().flush().ok()?;
            None
        }
        Ok(_) => {
            let text = line.trim_end_matches(['\n', '\r']).to_string();
            print!("{}", retire(&text, ansi, hint));
            io::stdout().flush().ok()?;
            Some(text)
        }
    }
}

#[cfg(test)]
#[path = "../tests/input.rs"]
mod tests;
