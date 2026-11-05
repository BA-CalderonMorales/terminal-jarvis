use crate::{cli::args, contracts::Harness};
#[path = "next.rs"]
mod next;
pub use next::{Next, Resolved};

pub fn resolve(input: &str, harnesses: &[Harness]) -> Resolved {
    let input = input.trim();
    match input {
        "" => return Resolved::Empty,
        "help" | "/help" | "?" => return Resolved::Help,
        "/exit" | "/quit" | "exit" | "quit" => return Resolved::Exit,
        "/home" | "/clear" | "home" | "clear" => return Resolved::Home,
        rest if rest == "list" || rest.starts_with("list ") => {
            return Resolved::List(super::picker::filter(rest));
        }
        "/debug" | "debug" => return Resolved::Debug(None),
        "/debug on" => return Resolved::Debug(Some(true)),
        "/debug off" => return Resolved::Debug(Some(false)),
        rest if rest == "converse" || rest.starts_with("converse ") => {
            return match crate::converse::parse(rest, harnesses) {
                crate::converse::Parsed::Continue => Resolved::Converse(None),
                crate::converse::Parsed::Start {
                    turns,
                    a,
                    b,
                    topic,
                    allow_dangerous,
                } => Resolved::Converse(Some((turns, a, b, topic, allow_dangerous))),
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
            None => Resolved::Error(format!("no harness at position {number}; list shows all")),
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
