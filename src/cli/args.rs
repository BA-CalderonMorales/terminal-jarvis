use crate::contracts::Capability;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Help,
    List,
    Check,
    Current,
    Use(String),
    Show(String),
    Plan {
        harness: Option<String>,
        capability: Capability,
    },
    Run {
        harness: String,
        capability: Capability,
        extra: Vec<String>,
    },
}

pub fn parse<I>(args: I) -> Result<Action, String>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let words = args.into_iter().map(Into::into).skip(1).collect::<Vec<_>>();
    if words.is_empty() {
        return Ok(Action::Help);
    }
    match words[0].as_str() {
        "help" | "--help" | "-h" => Ok(Action::Help),
        "list" => Ok(Action::List),
        "check" => Ok(Action::Check),
        "current" => Ok(Action::Current),
        "use" => one(&words, "use").map(Action::Use),
        "show" => one(&words, "show").map(Action::Show),
        "plan" => plan(&words[1..]),
        "run" => run(&words[1..]),
        other => Err(format!("unknown command '{other}'")),
    }
}

fn one(words: &[String], command: &str) -> Result<String, String> {
    match words {
        [_, value] => Ok(value.clone()),
        _ => Err(format!("usage: terminal-jarvis {command} <harness>")),
    }
}

fn plan(words: &[String]) -> Result<Action, String> {
    match words {
        [capability] => Ok(Action::Plan {
            harness: None,
            capability: cap(capability)?,
        }),
        [harness, capability] => Ok(Action::Plan {
            harness: Some(harness.clone()),
            capability: cap(capability)?,
        }),
        _ => Err("usage: terminal-jarvis plan [harness] <capability>".to_string()),
    }
}

fn run(words: &[String]) -> Result<Action, String> {
    match words {
        [harness, capability, extra @ ..] => Ok(Action::Run {
            harness: harness.clone(),
            capability: cap(capability)?,
            extra: extra.to_vec(),
        }),
        _ => Err("usage: terminal-jarvis run <harness> <capability> [args...]".to_string()),
    }
}

fn cap(value: &str) -> Result<Capability, String> {
    Capability::parse(value).ok_or_else(|| format!("unknown capability '{value}'"))
}
