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
    let Some(bytes) = read_entry() else {
        print!("{}", retire("", ansi, hint, indicator));
        io::stdout().flush().ok()?;
        return None;
    };
    let text = text_of(&bytes);
    print!("{}", retire(&text, ansi, hint, indicator));
    io::stdout().flush().ok()?;
    Some(text)
}

/// Reads one entered line without printing anything -- the viewport paints
/// its own frame and positions the cursor first. A resize returns an empty
/// line so the loop repaints the frame at the new geometry.
pub fn raw_line() -> Option<String> {
    Some(text_of(&read_entry()?))
}

/// One entered line, byte by byte, from the unbuffered reader. `None` is EOF;
/// an empty line is a resize, which the caller turns into a repaint. A signal
/// must never end the session silently.
fn read_entry() -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    loop {
        if !super::readiness::wait_for_input() {
            return Some(bytes);
        }
        match super::raw_stdin::read_byte() {
            None => return None,
            Some(b'\n' | b'\r') => return Some(bytes),
            Some(byte) => bytes.push(byte),
        }
    }
}

fn text_of(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .trim_end_matches(['\n', '\r'])
        .to_string()
}
