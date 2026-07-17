use super::Action;

pub(super) fn command_help(words: &[String]) -> Result<Option<Action>, String> {
    if words.len() == 2 && matches!(words[1].as_str(), "--help" | "-h") {
        return Ok(canonical(&words[0]).map(Action::CommandHelp));
    }
    Ok(None)
}

pub(super) fn help(words: &[String]) -> Result<Action, String> {
    match words {
        [_] => Ok(Action::Help),
        [_, command] => canonical(command)
            .map(Action::CommandHelp)
            .ok_or_else(|| format!("unknown command '{command}'")),
        _ => Err("usage: terminal-jarvis help [command]".into()),
    }
}

fn canonical(value: &str) -> Option<String> {
    let name = match value {
        "tools" => "list",
        "status" => "check",
        "info" => "show",
        other => other,
    };
    const COMMANDS: &[&str] = &[
        "help",
        "version",
        "list",
        "check",
        "current",
        "use",
        "show",
        "plan",
        "run",
        "install",
        "update",
        "auth",
        "config",
        "cache",
        "security",
        "gate",
        "experimental",
        "templates",
        "db",
    ];
    COMMANDS.contains(&name).then(|| name.to_string())
}
