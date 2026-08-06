//! Shell: the chat-style read-prompt-respond loop -- results print above the
//! input box. Bare verbs, numbers, and names switch tools.

use crate::cli::args;
use crate::contracts::Harness;
use std::path::Path;

#[path = "./canonical.rs"]
mod canonical;
#[path = "./handle.rs"]
mod handle;
#[path = "./status.rs"]
mod status;
pub use handle::handle;

pub fn run(harnesses: &[Harness], catalog_root: &Path, state_home: &Path, options: &args::Options) {
    let mut hint = modeline(state_home, false);
    while let Some(input) = super::input::read_line(&hint) {
        let next =
            super::sigint::guarded(|| handle(harnesses, catalog_root, state_home, options, &input));
        match next {
            Next::Exit => break,
            Next::Again { picker_shown } => {
                hint = modeline(state_home, picker_shown);
                println!();
            }
        }
    }
    println!();
}

pub enum Resolved {
    Empty,
    Exit,
    Home,
    Run(args::Action),
    Error(String),
}

pub enum Next {
    Exit,
    Again { picker_shown: bool },
}

pub fn resolve(input: &str, harnesses: &[Harness]) -> Resolved {
    let input = input.trim();
    if input.is_empty() {
        return Resolved::Empty;
    }
    match input {
        "/exit" | "/quit" | "exit" | "quit" => return Resolved::Exit,
        "/home" | "/clear" | "home" | "clear" => return Resolved::Home,
        _ => {}
    }
    if let Some(rest) = input.strip_prefix('/') {
        return match super::palette::parse(rest) {
            Ok(action) => Resolved::Run(action),
            Err(message) => Resolved::Error(message),
        };
    }
    if let Ok(number) = input.parse::<usize>() {
        return match super::switcher::select(input, harnesses) {
            Some(selection) => Resolved::Run(selection),
            None => Resolved::Error(format!(
                "no harness at position {number}; /list shows the numbered tools"
            )),
        };
    }
    if harnesses.iter().any(|harness| harness.name == input) {
        return Resolved::Run(args::Action::Use(input.to_string()));
    }
    match super::palette::parse(input) {
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

fn modeline(state_home: &Path, picker_shown: bool) -> String {
    if picker_shown {
        return "pick a number to switch agents, or type 'home' to go back".to_string();
    }
    let active = crate::context::load(state_home)
        .ok()
        .flatten()
        .map(|session| session.active_harness)
        .unwrap_or_else(|| "none".to_string());
    format!("active: {active} | a number or name switches, list, status, help, home, exit")
}

#[cfg(test)]
#[path = "../tests/shell_props.rs"]
mod props;
#[cfg(test)]
#[path = "../tests/shell.rs"]
mod tests;
