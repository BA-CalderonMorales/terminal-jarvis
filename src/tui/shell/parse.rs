//! Slash-command and selection resolution for the shell: every line maps
//! onto the same Action grammar the headless cli parses, options included
//! (`--no-input`, `--confirm=...` typed in-frame reach the same guards).

use crate::{cli::args, contracts::Harness};
pub enum Resolved {
    Empty,
    Help,
    Exit,
    Home,
    Run(args::Action, args::Options),
    Debug(Option<bool>),
    Theme(Option<String>),
    Converse(Option<(usize, String, String, String)>),
    Error(String),
}

#[derive(Debug)]
pub enum Next {
    Exit,
    /// `reset` restores the pristine body (the home primer).
    Again {
        picker_shown: bool,
        reset: bool,
    },
    Debug(Option<bool>),
    Converse(Option<(usize, String, String, String)>),
    /// A streaming action plus its typed options (session-overlaid).
    Stream {
        action: args::Action,
        options: args::Options,
    },
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
            return match crate::converse::parse(rest, harnesses) {
                crate::converse::Parsed::Continue => Resolved::Converse(None),
                crate::converse::Parsed::Start { turns, a, b, topic } => {
                    Resolved::Converse(Some((turns, a, b, topic)))
                }
                crate::converse::Parsed::Error(message) => Resolved::Error(message),
            };
        }
        "/theme" | "theme" => return Resolved::Theme(None),
        rest if rest.starts_with("/theme ") | rest.starts_with("theme ") => {
            return Resolved::Theme(Some(rest[rest.find(' ').unwrap() + 1..].trim().to_string()))
        }
        rest if rest.starts_with('/') => {
            return match crate::tui::palette::parse(&rest[1..]) {
                Ok((action, options)) => Resolved::Run(action, options),
                Err(message) => Resolved::Error(message),
            };
        }
        _ => {}
    }
    if let Ok(number) = input.parse::<usize>() {
        return match crate::tui::switcher::select(input, harnesses) {
            Some(selection) => Resolved::Run(selection, args::Options::default()),
            None => Resolved::Error(format!(
                "no harness at position {number}; /list shows the numbered tools"
            )),
        };
    }
    if harnesses.iter().any(|harness| harness.name == input) {
        return Resolved::Run(
            args::Action::Use(input.to_string()),
            args::Options::default(),
        );
    }
    let unknown_harness = |action: &args::Action| match action {
        args::Action::Direct { harness, .. } => !harnesses.iter().any(|h| &h.name == harness),
        _ => false,
    };
    match crate::tui::palette::parse(input) {
        // a bare verb line naming no harness speaks for the active agent
        Ok((action, options)) if unknown_harness(&action) => Resolved::Run(
            args::Action::Run(input.split_whitespace().map(String::from).collect()),
            options,
        ),
        Ok((action, options)) => Resolved::Run(action, options),
        Err(message) => Resolved::Error(message),
    }
}

#[cfg(test)]
#[path = "../tests/shell_props.rs"]
mod props;
#[cfg(test)]
#[path = "../tests/shell.rs"]
mod tests;
