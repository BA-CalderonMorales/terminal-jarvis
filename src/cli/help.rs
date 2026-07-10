use super::{style, table};
const PLAIN: &str = "Terminal Jarvis\n\
     Headless command center for coding-agent harnesses\n\n\
     usage:\n\
       terminal-jarvis [harness] [args...]\n\
       terminal-jarvis run [harness] [capability] [args...]\n\
       terminal-jarvis version [--verbose|--info|-v]\n\
       terminal-jarvis list\n\
       terminal-jarvis check\n\
       terminal-jarvis use <harness>\n\
       terminal-jarvis current\n\
       terminal-jarvis show <harness>\n\
       terminal-jarvis plan [harness] <capability>\n\
       terminal-jarvis install <harness>\n\
       terminal-jarvis update [harness]\n\
       terminal-jarvis auth help <harness>\n\
       terminal-jarvis config show\n\
       terminal-jarvis cache status\n\
       terminal-jarvis security [status|audit|harness]\n\
       terminal-jarvis gate [status|list|enable [trivy]|disable|run [trivy]]\n\n\
      global flags:\n\
        --help, -h      show this help\n\
        --version, -v   print the version (plain)\n\
        --info          print version with provenance (same as version --verbose)\n\
        --update [--dry-run]\n\
                        self-update terminal-jarvis or print its package-manager command\n\
        --plain         stable line-oriented output for automation\n\
        --no-color      disable terminal color\n\n\
      capabilities:\n\
       download update headless version stats models security yolo ui\n\n\
     examples:\n\
       terminal-jarvis use opencode\n\
       terminal-jarvis plan codex headless\n\
       terminal-jarvis run opencode fix failing tests\n\
       terminal-jarvis gate enable trivy\n\n\
     experimental:\n\
       TERMINAL_JARVIS_EXPERIMENTAL_UI=1 terminal-jarvis experimental dashboard\n\n\
     legacy aliases:\n\
       tools -> list, status -> check, info <harness> -> show <harness>\n\
       install <harness> -> run <harness> download\n\
       update <harness> -> run <harness> update\n";

pub fn text() -> String {
    if style::plain() {
        return PLAIN.to_string();
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
