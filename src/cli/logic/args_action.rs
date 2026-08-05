use super::child_parser;
use super::help_parser::{command_help, help};
use super::validators::{at_most_one, exact_value, valid_auth, valid_choice, valid_gate};
use super::values::{exact, one, optional_one, plan, version};
use super::Action;

pub(super) fn parse(
    words: &[String],
    child: Vec<String>,
    boundary: bool,
) -> Result<Action, String> {
    if words.is_empty() {
        return no_command(child, boundary);
    }
    if let Some(help) = command_help(words)? {
        return no_child(help, child, boundary);
    }
    let action = match words[0].as_str() {
        "help" => help(words)?,
        "--help" | "-h" => exact(words, Action::Help, "terminal-jarvis --help")?,
        "version" => version(&words[1..])?,
        "--version" | "-v" if words.len() == 1 => Action::Version { verbose: false },
        "-v" if words == ["-v", "version"] => Action::Version { verbose: false },
        "--info" => exact(
            words,
            Action::Version { verbose: true },
            "terminal-jarvis --info",
        )?,
        "list" | "tools" => exact(words, Action::List, "terminal-jarvis list")?,
        "check" | "status" => exact(words, Action::Check, "terminal-jarvis check")?,
        "current" => exact(words, Action::Current, "terminal-jarvis current")?,
        "use" => one(words, "use").map(Action::Use)?,
        "show" | "info" => one(words, words[0].as_str()).map(Action::Show)?,
        "plan" => plan(&words[1..])?,
        "run" => return child_parser::run(&words[1..], child, boundary),
        "install" => one(words, "install").map(Action::Install)?,
        "update" => optional_one(words, "update").map(Action::Update)?,
        "self-update" => exact(
            words,
            Action::SelfUpdate { dry_run: false },
            "terminal-jarvis self-update",
        )?,
        "--update" => exact(
            words,
            Action::SelfUpdate { dry_run: false },
            "terminal-jarvis --update",
        )?,
        "auth" => Action::Auth(valid_auth(&words[1..])?),
        "config" => Action::Config(valid_choice(
            &words[1..],
            "config",
            &["show", "path", "reset"],
        )?),
        "cache" => Action::Cache(valid_choice(
            &words[1..],
            "cache",
            &["status", "clear", "refresh"],
        )?),
        "security" => Action::Security(at_most_one(&words[1..], "security")?),
        "gate" => Action::Gate(valid_gate(&words[1..])?),
        "experimental" => {
            Action::Experimental(exact_value(&words[1..], "dashboard", "experimental")?)
        }
        "templates" | "db" => exact(
            words,
            Action::Legacy(words[0].clone()),
            "terminal-jarvis templates|db",
        )?,
        flag if flag.starts_with('-') => return Err(format!("unknown flag '{flag}'")),
        harness => return child_parser::direct(harness, &words[1..], child, boundary),
    };
    no_child(action, child, boundary)
}

fn no_command(child: Vec<String>, boundary: bool) -> Result<Action, String> {
    if boundary || !child.is_empty() {
        Err("`--` requires run or direct harness invocation".into())
    } else {
        Ok(Action::Help)
    }
}

fn no_child(action: Action, child: Vec<String>, boundary: bool) -> Result<Action, String> {
    if boundary || !child.is_empty() {
        Err("`--` is valid only with run or direct harness invocation".into())
    } else {
        Ok(action)
    }
}
