use super::{style, table};

pub fn text(name: &str) -> String {
    let (usage, purpose) = entry(name);
    if style::plain() {
        return format!("terminal-jarvis {name}\n{purpose}\n\nusage: {usage}\n");
    }
    format!(
        "{}{}",
        style::banner(&format!("terminal-jarvis {name}"), purpose),
        table::fields("Usage", &[("COMMAND", usage.to_string())])
    )
}

fn entry(name: &str) -> (&'static str, &'static str) {
    match name {
        "help" => (
            "terminal-jarvis help [command]",
            "Show top-level or command help.",
        ),
        "version" => (
            "terminal-jarvis version [--verbose]",
            "Show version and optional provenance.",
        ),
        "list" => (
            "terminal-jarvis list",
            "List catalog harnesses and support summaries.",
        ),
        "check" => (
            "terminal-jarvis check [--verbose]",
            "Diagnose local Terminal Jarvis and harness readiness.",
        ),
        "current" => ("terminal-jarvis current", "Show the active harness."),
        "use" => (
            "terminal-jarvis use <harness>",
            "Select the active harness.",
        ),
        "show" => (
            "terminal-jarvis show <harness>",
            "Show one harness and its capability truth.",
        ),
        "plan" => (
            "terminal-jarvis plan [harness] <capability>",
            "Preview an exact command without running it.",
        ),
        "run" => (
            "terminal-jarvis run [harness] [capability] [args...] [-- child-args...]",
            "Run a guarded harness capability.",
        ),
        "install" => (
            "terminal-jarvis install <harness> [lifecycle options]",
            "Preview or confirm a harness install.",
        ),
        "update" => (
            "terminal-jarvis update [harness] [lifecycle options]",
            "Inspect or confirm a harness update.",
        ),
        "self-update" => (
            "terminal-jarvis self-update [lifecycle options]",
            "Preview or confirm a Terminal Jarvis update.",
        ),
        "auth" => (
            "terminal-jarvis auth [help|set] <harness>",
            "Inspect credential-name requirements without values.",
        ),
        "config" => (
            "terminal-jarvis config [show|path|reset]",
            "Inspect configuration state and recovery guidance.",
        ),
        "cache" => (
            "terminal-jarvis cache [status|clear|refresh]",
            "Inspect wrapper cache state and recovery guidance.",
        ),
        "security" => (
            "terminal-jarvis security [status|audit|harness]",
            "Inspect local security and support plans.",
        ),
        "gate" => (
            "terminal-jarvis gate [status|list|enable [name]|disable|run [name]]",
            "Inspect or control optional local gates.",
        ),
        "experimental" => (
            "terminal-jarvis experimental dashboard",
            "Use an explicitly enabled experimental surface.",
        ),
        "templates" | "db" => (
            "terminal-jarvis help",
            "This removed command returns migration guidance.",
        ),
        _ => ("terminal-jarvis help", "Show Terminal Jarvis help."),
    }
}

#[cfg(test)]
#[path = "help_command_test.rs"]
mod tests;
