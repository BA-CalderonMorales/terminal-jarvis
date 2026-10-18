//! Gallery: renders each shell layout at a few widths so the composed frame
//! can be eyeballed: `cargo test -- --nocapture gallery`. Not an assertion.
use crate::contracts::Harness;
use crate::tui::screen::{frame, Draft, Size};

fn harness(name: &str, description: &str) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: description.into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}
fn strip(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for p in chars.by_ref() {
                if p.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn show(name: &str, body: &[String], cols: usize) {
    let draft = Draft {
        header: "Terminal Jarvis · ACTIVE codex · READY 1/3 ready".into(),
        cwd: "~/Projects/terminal-jarvis".into(),
        tagline: "context command center".into(),
        body: body.to_vec(),
        prompt: "[>_]::[tj]:: ".into(),
        offset: 0,
        hint: "active: codex | a number or name switches, list, status, help, home, exit".into(),
        bottom: false,
    };
    println!("\n----- {name} @ {cols} cols -----");
    for line in frame(Size { cols, rows: 24 }, &draft).split("\r\n") {
        println!("|{}", strip(line));
    }
}

fn lines(text: &str) -> Vec<String> {
    text.lines().map(String::from).collect()
}

fn install_card() -> Vec<String> {
    use crate::tui::screen::{paint, Role};
    use crate::tui::shell::{session, stream_rows, verdict};
    let chapter = session::chapter("install droid");
    let plan = "  plan  droid download · sh -c 'curl -fsSL https://app.factory.ai/cli | sh'";
    let mut rows = vec![
        paint(Role::Accent, &chapter),
        paint(Role::Dim, plan),
        paint(Role::Ok, "  gate  download:droid  [Y/n]"),
    ];
    for line in [
        "INFO  Downloading Factory CLI v0.244.0 for linux-x64",
        "INFO  Fetching and verifying checksum",
        "INFO  Checksum verification passed",
        "WARN  PATH does not include ~/.local/bin yet",
        "INFO  Installed to /home/bmoe/.local/bin/droid",
    ] {
        rows.push(stream_rows::render(line));
    }
    let elapsed = std::time::Duration::from_millis(3_200);
    let (_, card, _) = verdict::text("droid", "installed", true, &Ok((0, String::new())), elapsed);
    rows.push(paint(Role::Ok, &format!("✓ {card}")));
    rows
}

#[test]
fn gallery() {
    let harnesses = [
        harness("codex", "the CLI coding agent"),
        harness("claude", "Anthropic's coding agent"),
        harness("opencode", "the open source coding agent"),
    ];
    let root = std::env::temp_dir();
    let home = root.join("tj-gallery-home");
    let status = lines(&crate::tui::shell::status::render(&harnesses, &root, &home));
    for cols in [50usize, 80, 120] {
        let inner = cols - 2;
        let welcome = crate::tui::screen::welcome("codex", 1, 3, inner);
        let help = lines(&crate::tui::shell::help::text(inner));
        show("welcome", &welcome, cols);
        show("status", &status, cols);
        show("help", &help, cols);
        show("install card", &install_card(), cols);
    }
}
