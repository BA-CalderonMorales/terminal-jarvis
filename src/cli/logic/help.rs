use super::{help_text, style, table};

pub fn text() -> String {
    if style::plain() {
        return help_text::PLAIN.to_string();
    }
    let rows = vec![
        vec![
            "list | show <harness> | check".into(),
            "Inspect harnesses and readiness".into(),
        ],
        vec![
            "use <harness> | current".into(),
            "Select and inspect the active harness".into(),
        ],
        vec![
            "plan [harness] <capability>".into(),
            "Preview a command without running it".into(),
        ],
        vec![
            "run | install | update <harness>".into(),
            "Execute a harness capability".into(),
        ],
        vec![
            "auth | config | cache | security".into(),
            "Inspect local setup and security posture".into(),
        ],
        vec![
            "gate [status|enable|disable|run]".into(),
            "Control the optional Trivy gate".into(),
        ],
        vec![
            "version | --update [--dry-run]".into(),
            "Inspect or update Terminal Jarvis".into(),
        ],
    ];
    let mut out = style::banner(
        "Terminal Jarvis",
        "Headless command center for coding-agent harnesses",
    );
    out.push_str(&table::render("Commands", &["COMMAND", "PURPOSE"], &rows));
    out.push('\n');
    out.push_str(&table::fields(
        "Global Flags",
        &[
            (
                "--PLAIN",
                "Stable line-oriented output for automation".into(),
            ),
            ("--NO-COLOR", "Disable terminal color".into()),
            ("--JSON", "Emit one versioned JSON document".into()),
            ("--VERBOSE", "Expand check or version diagnostics".into()),
            (
                "--DRY-RUN",
                "Preview lifecycle operations without effects".into(),
            ),
            ("--NO-INPUT", "Guarantee that no prompt is opened".into()),
            ("--CONFIRM=OP:TARGET", "Bind noninteractive intent".into()),
            (
                "--ALLOW-DANGEROUS",
                "Separately opt in to dangerous execution".into(),
            ),
            ("--", "Forward following flags to a child".into()),
            ("--INFO", "Show version provenance".into()),
            (
                "--UPDATE --DRY-RUN",
                "Print the selected package-manager command".into(),
            ),
        ],
    ));
    out.push_str("\nExamples\n  terminal-jarvis use opencode\n  terminal-jarvis plan codex headless\n  terminal-jarvis gate enable trivy\n");
    out
}
