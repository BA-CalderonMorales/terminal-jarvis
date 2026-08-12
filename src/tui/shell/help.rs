//! The tui's own command table. Headless-only flags (`--plain`, `--json`,
//! `--no-input`, ...) and bare-invocation rows that cannot be typed here are
//! omitted; the headless `terminal-jarvis help` keeps the full grammar. The
//! security line anchors what the gate actually guards in this surface.

use crate::cli::style;

pub fn text() -> String {
    let rows = [
        ("list", "numbered picker of every harness"),
        ("show <harness>", "details and readiness of one harness"),
        (
            "plan [harness] <cap>",
            "preview a command without running it",
        ),
        (
            "use <harness> | <number> | <name>",
            "switch the active harness",
        ),
        (
            "run <harness> [args]",
            "open the harness ui in a framed session",
        ),
        (
            "install <harness>",
            "install with the security gate in front",
        ),
        ("update <harness>", "update through the same gate"),
        (
            "gate [status|enable|disable|run]",
            "control the optional Trivy gate",
        ),
        (
            "auth | config | cache | security",
            "inspect local setup and posture",
        ),
        (
            "/debug [on|off]",
            "see the raw, verbose view behind the clean one",
        ),
        ("home | clear", "reset the frame"),
        ("exit | quit", "leave without state changes"),
    ];
    let mut body = String::from("Commands\n");
    for (command, purpose) in rows {
        body.push_str(&format!(
            "  {}  {}\n",
            style::heading(&format!("{command:<32}")),
            purpose
        ));
    }
    body.push_str(&format!(
        "  {}\n",
        style::heading(&format!("{:<32}", "empty line"))
    ));
    body.push_str(&format!(
        "  {}\n",
        style::dim("  every install and run is checked by the enabled Trivy gate")
    ));
    body
}
