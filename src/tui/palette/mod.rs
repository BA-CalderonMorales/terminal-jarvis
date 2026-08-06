//! Palette: the clap-grounded slash-command surface. A slash line is parsed
//! with the exact same `Action` grammar as argv, after prepending a dummy
//! program name -- the tui never maintains a parallel command language.

use crate::cli::args;

pub fn parse(line: &str) -> Result<args::Action, String> {
    let mut tokens = vec![String::from("terminal-jarvis")];
    tokens.extend(line.split_whitespace().map(String::from));
    args::parse_cli(tokens)
        .map(|parsed| parsed.action)
        .map_err(|error| format!("{error}; type /help for commands"))
}

#[cfg(test)]
#[path = "../tests/palette.rs"]
mod tests;
