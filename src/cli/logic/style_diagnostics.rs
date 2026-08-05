use super::{color_enabled_for, term_is_dumb, OPTIONS};
use std::cell::Cell;
use std::io::IsTerminal;

pub fn decisions() -> (bool, bool, bool) {
    let stdout = std::io::stdout().is_terminal();
    let stderr = std::io::stderr().is_terminal();
    let options = OPTIONS.with(Cell::get);
    let no_color = std::env::var_os("NO_COLOR").is_some();
    let dumb = term_is_dumb(std::env::var("TERM").ok().as_deref());
    (
        stdout,
        stderr,
        color_enabled_for(stdout, options.no_color, no_color, dumb),
    )
}
