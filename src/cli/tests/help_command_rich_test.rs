use super::{style, text};

#[test]
fn rich_command_help_uses_the_same_contract() {
    let previous = style::set(false, true);
    let help = text("self-update");
    style::restore(previous);
    assert!(help.contains("terminal-jarvis self-update"));
    assert!(help.contains("Preview or confirm a Terminal Jarvis update."));
    assert!(help.contains("Usage"));
}
