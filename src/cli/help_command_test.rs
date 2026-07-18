use super::{style, text};

const COMMANDS: [&str; 20] = [
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
    "self-update",
    "auth",
    "config",
    "cache",
    "security",
    "gate",
    "experimental",
    "templates",
    "unknown",
];

#[test]
fn every_command_has_direct_plain_help() {
    let previous = style::set(true, true);
    for command in COMMANDS {
        let help = text(command);
        assert!(help.starts_with(&format!("terminal-jarvis {command}\n")));
        assert!(help.contains("\n\nusage: terminal-jarvis "));
    }
    style::restore(previous);
}

#[test]
fn rich_command_help_uses_the_same_contract() {
    let previous = style::set(false, true);
    let help = text("self-update");
    style::restore(previous);
    assert!(help.contains("terminal-jarvis self-update"));
    assert!(help.contains("Preview or confirm a Terminal Jarvis update."));
    assert!(help.contains("Usage"));
}
