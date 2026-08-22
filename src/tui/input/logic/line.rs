//! Line readers: the chat widget (hint above, prompt below) and the raw
//! single-line read the viewport uses after painting its frame.

use super::Indicator;
use crate::cli::style;
use crate::tui::term;
use std::io::{self, Write};

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
    let outcome = loop {
        break match io::stdin().read_line(&mut line) {
            // A signal (resize, tick) must never end the session silently.
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            other => other,
        };
    };
    match outcome {
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

/// Reads one entered line without printing anything -- the viewport paints
/// its own frame and positions the cursor first.
pub fn raw_line() -> Option<String> {
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Ok(0) | Err(_) => return None,
            Ok(_) => return Some(line.trim_end_matches(['\n', '\r']).to_string()),
        }
    }
}
