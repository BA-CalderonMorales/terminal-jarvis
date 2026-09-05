//! Slash-command and selection resolution for the shell: every line maps
//! onto the same Action grammar the headless cli parses.

use crate::{cli::args, contracts::Harness};
pub enum Resolved {
    Empty,
    Help,
    Exit,
    Home,
    Run(args::Action),
    Debug(Option<bool>),
    Theme(Option<String>),
    Converse(Option<(String, String, String)>),
    Error(String),
}

#[derive(Debug)]
pub enum Next {
    Exit,
    /// `reset` asks the surface to restore its pristine body: chat mode
    /// printed the banner through the sink, viewport mode re-shows art.
    Again {
        picker_shown: bool,
        reset: bool,
    },
    Debug(Option<bool>),
    Converse(Option<(String, String, String)>),
}

pub fn resolve(input: &str, harnesses: &[Harness]) -> Resolved {
    let input = input.trim();
    match input {
        "" => return Resolved::Empty,
        "help" | "/help" => return Resolved::Help,
        "/exit" | "/quit" | "exit" | "quit" => return Resolved::Exit,
        "/home" | "/clear" | "home" | "clear" => return Resolved::Home,
        "/debug" | "debug" => return Resolved::Debug(None),
        "/debug on" => return Resolved::Debug(Some(true)),
        "/debug off" => return Resolved::Debug(Some(false)),
        rest if rest == "converse" || rest.starts_with("converse ") => {
            let words: Vec<&str> = rest.split_whitespace().skip(1).collect();
            return match words.as_slice() {
                [] => Resolved::Converse(None),
                [a, b, topic @ ..] if !topic.is_empty() => {
                    for side in [*a, *b] {
                        if let Err(message) = crate::converse::headless_ready(harnesses, side) {
                            return Resolved::Error(message);
                        }
                    }
                    Resolved::Converse(Some(((*a).into(), (*b).into(), topic.join(" "))))
                }
                _ => Resolved::Error("converse <a> <b> <topic...>".to_string()),
            };
        }
        "/theme" | "theme" => return Resolved::Theme(None),
        rest if rest.starts_with("/theme ") => {
            return Resolved::Theme(Some(rest["/theme ".len()..].trim().to_string()))
        }
        rest if rest.starts_with("theme ") => {
            return Resolved::Theme(Some(rest["theme ".len()..].trim().to_string()))
        }
        rest if rest.starts_with('/') => {
            return match crate::tui::palette::parse(&rest[1..]) {
                Ok(action) => Resolved::Run(action),
                Err(message) => Resolved::Error(message),
            };
        }
        _ => {}
    }
    if let Ok(number) = input.parse::<usize>() {
        return match crate::tui::switcher::select(input, harnesses) {
            Some(selection) => Resolved::Run(selection),
            None => Resolved::Error(format!(
                "no harness at position {number}; /list shows the numbered tools"
            )),
        };
    }
    if harnesses.iter().any(|harness| harness.name == input) {
        return Resolved::Run(args::Action::Use(input.to_string()));
    }
    match crate::tui::palette::parse(input) {
        Ok(args::Action::Direct { harness, .. })
            if !harnesses.iter().any(|h| h.name == harness) =>
        {
            Resolved::Run(args::Action::Run(
                input.split_whitespace().map(String::from).collect(),
            ))
        }
        Ok(action) => Resolved::Run(action),
        Err(message) => Resolved::Error(message),
    }
}

#[cfg(test)]
#[path = "../tests/shell_props.rs"]
mod props;
#[cfg(test)]
#[path = "../tests/shell.rs"]
mod tests;
