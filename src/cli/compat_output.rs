use super::super::{style, table};
use crate::contracts::{Capability, Harness};

pub fn updates(version: &str, harnesses: &[Harness]) -> String {
    if style::plain() {
        let mut out = format!("updates are per harness in v{version}\n");
        out.push_str("run `terminal-jarvis update <harness>` to execute one update\n");
        for harness in harnesses {
            let plan = harness.plan(Capability::Update).expect("validated update");
            out.push_str(&format!("{}: {}\n", harness.name, plan.command.render()));
        }
        return out;
    }
    let rows = harnesses
        .iter()
        .map(|harness| {
            let plan = harness.plan(Capability::Update).expect("validated update");
            vec![harness.name.clone(), plan.command.render()]
        })
        .collect::<Vec<_>>();
    format!(
        "{}\n{}",
        table::fields(
            "Harness Updates",
            &[
                ("VERSION", format!("v{version}")),
                ("NEXT STEP", "terminal-jarvis update <harness>".to_string()),
            ],
        ),
        table::render("Update Commands", &["HARNESS", "COMMAND"], &rows)
    )
}

pub fn auth_notice(version: &str) -> String {
    if style::plain() {
        return format!(
            "credential manager is not active in v{version}\nuse `terminal-jarvis auth help <harness>` and export the listed env vars\n"
        );
    }
    format!(
        "{}\n{}",
        style::warning("Credential manager is not active."),
        table::fields(
            "Authentication",
            &[
                ("VERSION", format!("v{version}")),
                (
                    "NEXT STEP",
                    "terminal-jarvis auth help <harness>".to_string()
                ),
            ],
        )
    )
}

pub fn auth_detail(harness: &Harness, status: &str, note: &str) -> String {
    if style::plain() {
        return format!(
            "auth for {} ({})\nsetup: {}\nstatus: {}\n{}\n",
            harness.display,
            harness.name,
            harness.setup_hint(),
            status,
            note
        );
    }
    table::fields(
        &format!("Authentication: {} ({})", harness.display, harness.name),
        &[
            ("SETUP", harness.setup_hint()),
            ("STATUS", status.to_string()),
            ("NOTE", note.to_string()),
        ],
    )
}
