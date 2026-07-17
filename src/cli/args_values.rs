use super::Action;
use crate::contracts::Capability;

pub(super) fn version(words: &[String]) -> Result<Action, String> {
    match words {
        [] => Ok(Action::Version { verbose: false }),
        [flag] if flag == "-v" => Ok(Action::Version { verbose: false }),
        [flag] if flag == "--info" => Ok(Action::Version { verbose: true }),
        [flag] if flag.starts_with('-') => Err(format!(
            "unknown flag '{flag}'; usage: terminal-jarvis version [--verbose|--info|-v]"
        )),
        _ => Err("usage: terminal-jarvis version [--verbose|--info|-v]".into()),
    }
}

pub(super) fn exact(words: &[String], action: Action, usage: &str) -> Result<Action, String> {
    (words.len() == 1)
        .then_some(action)
        .ok_or_else(|| format!("usage: {usage}"))
}

pub(super) fn one(words: &[String], command: &str) -> Result<String, String> {
    match words {
        [_, value] => Ok(value.clone()),
        _ => Err(format!("usage: terminal-jarvis {command} <harness>")),
    }
}

pub(super) fn optional_one(words: &[String], command: &str) -> Result<Option<String>, String> {
    match words {
        [_] => Ok(None),
        [_, value] => Ok(Some(value.clone())),
        _ => Err(format!("usage: terminal-jarvis {command} [harness]")),
    }
}

pub(super) fn plan(words: &[String]) -> Result<Action, String> {
    match words {
        [capability] => Ok(Action::Plan {
            harness: None,
            capability: cap(capability)?,
        }),
        [harness, capability] => Ok(Action::Plan {
            harness: Some(harness.clone()),
            capability: cap(capability)?,
        }),
        _ => Err("usage: terminal-jarvis plan [harness] <capability>".into()),
    }
}

fn cap(value: &str) -> Result<Capability, String> {
    Capability::parse(value).ok_or_else(|| format!("unknown capability '{value}'"))
}
