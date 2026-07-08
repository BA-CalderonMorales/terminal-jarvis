use crate::contracts::Capability;

pub use super::action::Action;

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
        "version" => version(&words[1..]),
        "--version" | "-v" => Ok(Action::Version { verbose: false }),
        "--info" => Ok(Action::Version { verbose: true }),
        "list" | "tools" => Ok(Action::List),
        "check" | "status" => Ok(Action::Check),
        "current" => Ok(Action::Current),
        "use" => one(&words, "use").map(Action::Use),
        "show" | "info" => one(&words, words[0].as_str()).map(Action::Show),
        "plan" => plan(&words[1..]),
        "run" => Ok(Action::Run(words[1..].to_vec())),
        "install" => one(&words, "install").map(Action::Install),
        "update" => optional_one(&words, "update").map(Action::Update),
        "auth" => Ok(Action::Auth(words[1..].to_vec())),
        "config" => Ok(Action::Config(words[1..].to_vec())),
        "cache" => Ok(Action::Cache(words[1..].to_vec())),
        "security" => Ok(Action::Security(words[1..].to_vec())),
        "templates" | "db" => Ok(Action::Legacy(words[0].clone())),
        other if other.starts_with('-') => Err(format!(
            "unknown flag '{other}'; use --help, --version, -v, or --info"
        )),
        other => Ok(Action::Direct {
            harness: other.to_string(),
            extra: words[1..].to_vec(),
        }),
    }
}

fn version(words: &[String]) -> Result<Action, String> {
    match words {
        [] => Ok(Action::Version { verbose: false }),
        [flag] if flag == "--verbose" || flag == "--info" => Ok(Action::Version { verbose: true }),
        [flag] if flag == "-v" => Ok(Action::Version { verbose: false }),
        _ => Err("usage: terminal-jarvis version [--verbose|--info|-v]".to_string()),
    }
}

fn one(words: &[String], command: &str) -> Result<String, String> {
    match words {
        [_, value] => Ok(value.clone()),
        _ => Err(format!("usage: terminal-jarvis {command} <harness>")),
    }
}

fn optional_one(words: &[String], command: &str) -> Result<Option<String>, String> {
    match words {
        [_] => Ok(None),
        [_, value] => Ok(Some(value.clone())),
        _ => Err(format!("usage: terminal-jarvis {command} [harness]")),
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

fn cap(value: &str) -> Result<Capability, String> {
    Capability::parse(value).ok_or_else(|| {
        let names = Capability::ALL
            .iter()
            .map(|capability| capability.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("unknown capability '{value}'; expected one of: {names}")
    })
}
