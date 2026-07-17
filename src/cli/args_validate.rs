pub(super) fn at_most_one(words: &[String], command: &str) -> Result<Vec<String>, String> {
    (words.len() <= 1)
        .then(|| words.to_vec())
        .ok_or_else(|| format!("usage: terminal-jarvis {command} [value]"))
}

pub(super) fn exact_value(
    words: &[String],
    value: &str,
    command: &str,
) -> Result<Vec<String>, String> {
    (words == [value])
        .then(|| words.to_vec())
        .ok_or_else(|| format!("usage: terminal-jarvis {command} {value}"))
}

pub(super) fn valid_choice(
    words: &[String],
    command: &str,
    choices: &[&str],
) -> Result<Vec<String>, String> {
    match words {
        [] => Ok(Vec::new()),
        [value] if choices.contains(&value.as_str()) => Ok(words.to_vec()),
        _ => Err(format!(
            "usage: terminal-jarvis {command} [{}]",
            choices.join("|")
        )),
    }
}

pub(super) fn valid_auth(words: &[String]) -> Result<Vec<String>, String> {
    match words {
        [] | [_] => Ok(words.to_vec()),
        [action, _] if matches!(action.as_str(), "help" | "set") => Ok(words.to_vec()),
        _ => Err("usage: terminal-jarvis auth [help|set] <harness>".into()),
    }
}

pub(super) fn valid_gate(words: &[String]) -> Result<Vec<String>, String> {
    match words {
        [] => Ok(Vec::new()),
        [action]
            if matches!(
                action.as_str(),
                "status" | "list" | "enable" | "disable" | "run"
            ) =>
        {
            Ok(words.to_vec())
        }
        [action, _] if matches!(action.as_str(), "enable" | "run") => Ok(words.to_vec()),
        _ => {
            Err("usage: terminal-jarvis gate [status|list|enable [name]|disable|run [name]]".into())
        }
    }
}
use super::{Action, Options, OutputMode};

pub(super) fn validate_options(action: &Action, options: &Options) -> Result<(), String> {
    if options.verbose && !matches!(action, Action::Check | Action::Version { .. }) {
        return Err("--verbose is valid only with check or version".into());
    }
    let lifecycle =
        options.dry_run || options.no_input || options.confirm.is_some() || options.allow_dangerous;
    if lifecycle
        && !matches!(
            action,
            Action::Run(_)
                | Action::Direct { .. }
                | Action::Install(_)
                | Action::Update(Some(_))
                | Action::SelfUpdate { .. }
        )
    {
        return Err("lifecycle options require run, install, harness update, or --update".into());
    }
    if options.output == OutputMode::Json && json_spawns_child(action, options.dry_run) {
        return Err("--json is unavailable for child execution; use plan or --dry-run".into());
    }
    Ok(())
}

fn json_spawns_child(action: &Action, dry_run: bool) -> bool {
    match action {
        Action::Run(_) | Action::Direct { .. } => true,
        Action::Gate(words) => words.first().is_some_and(|word| word == "run"),
        Action::Install(_) | Action::Update(Some(_)) | Action::SelfUpdate { .. } => !dry_run,
        _ => false,
    }
}
