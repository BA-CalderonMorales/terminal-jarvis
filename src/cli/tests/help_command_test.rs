use super::{entry, style, text};

const SUB: &str = "terminal-jarvis";

const USAGE: [(&str, &str); 20] = [
    ("help", "help [command]"),
    ("version", "version [--verbose]"),
    ("list", "list"),
    ("check", "check [--verbose]"),
    ("tui", "tui"),
    ("current", "current"),
    ("use", "use <harness>"),
    ("show", "show <harness>"),
    ("plan", "plan [harness] <capability>"),
    (
        "run",
        "run [harness] [capability] [args...] [-- child-args...]",
    ),
    ("install", "install <harness> [lifecycle options]"),
    ("update", "update [harness] [lifecycle options]"),
    ("self-update", "self-update [lifecycle options]"),
    ("auth", "auth [help|set] <harness>"),
    ("config", "config [show|path|reset]"),
    ("cache", "cache [status|clear|refresh]"),
    ("security", "security [status|audit|harness]"),
    (
        "gate",
        "gate [status|list|enable [name]|disable|run [name]]",
    ),
    ("templates", "help"),
    ("db", "help"),
];

const PURPOSE: [(&str, &str); 20] = [
    ("help", "Show top-level or command help."),
    ("version", "Show version and optional provenance."),
    ("list", "List catalog harnesses and support summaries."),
    (
        "check",
        "Diagnose local Terminal Jarvis and harness readiness.",
    ),
    (
        "tui",
        "Open the interactive harness switcher on a terminal.",
    ),
    ("current", "Show the active harness."),
    ("use", "Select the active harness."),
    ("show", "Show one harness and its capability truth."),
    ("plan", "Preview an exact command without running it."),
    ("run", "Run a guarded harness capability."),
    ("install", "Preview or confirm a harness install."),
    ("update", "Inspect or confirm a harness update."),
    (
        "self-update",
        "Preview or confirm a Terminal Jarvis update.",
    ),
    (
        "auth",
        "Inspect credential-name requirements without values.",
    ),
    (
        "config",
        "Inspect configuration state and recovery guidance.",
    ),
    (
        "cache",
        "Inspect wrapper cache state and recovery guidance.",
    ),
    ("security", "Inspect local security and support plans."),
    ("gate", "Inspect or control optional local gates."),
    (
        "templates",
        "This removed command returns migration guidance.",
    ),
    ("db", "This removed command returns migration guidance."),
];

#[test]
fn entry_maps_every_command_to_exact_text() {
    for (name, rest) in USAGE {
        assert_eq!(entry(name).0, format!("{SUB} {rest}"));
    }
    for (name, purpose) in PURPOSE {
        assert_eq!(entry(name).1, purpose);
    }
    let fallback = entry("unknown");
    assert_eq!(fallback.0, "terminal-jarvis help");
    assert_eq!(fallback.1, "Show Terminal Jarvis help.");
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
