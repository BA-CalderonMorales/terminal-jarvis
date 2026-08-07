use std::path::Path;
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::Capability;

fn load() -> Vec<terminal_jarvis::contracts::Harness> {
    catalog::load(Path::new("harnesses")).expect("catalog loads")
}

fn expected_names() -> Vec<String> {
    let names = "aider amp claude code codex copilot crush cursor-agent droid \
        eca forge gemini goose hermes jules kilocode letta llxprt nanocoder \
        ollama openclaw opencode pi qwen vibe";
    names.split_whitespace().map(str::to_string).collect()
}

#[test]
fn harnesses_expose_every_core_capability() {
    let harnesses = load();
    assert_eq!(harnesses.len(), 25);
    assert!(catalog::validate(&harnesses).is_empty());
    for harness in harnesses {
        for capability in Capability::ALL {
            assert!(
                harness.plan(capability).is_some(),
                "{} missing {}",
                harness.name,
                capability
            );
        }
    }
}

#[test]
fn catalog_exposes_expected_initial_harnesses() {
    let mut promoted = load()
        .into_iter()
        .map(|harness| harness.name)
        .collect::<Vec<_>>();
    promoted.sort();
    assert_eq!(promoted, expected_names());
}

#[test]
fn update_commands_are_non_interactive() {
    for harness in load() {
        let update = harness.plan(Capability::Update).unwrap();
        let command = update.command.render().to_ascii_lowercase();
        for word in ["login", "auth", "configure", "wizard", "prompt"] {
            assert!(
                !command.contains(word),
                "{} update contains {word}",
                harness.name
            );
        }
    }
}

#[test]
fn yolo_capabilities_are_visibly_dangerous() {
    for harness in load() {
        let yolo = harness.plan(Capability::Yolo).unwrap();
        assert!(
            yolo.summary.to_ascii_lowercase().contains("danger"),
            "{} yolo summary must warn users",
            harness.name
        );
    }
}

#[test]
fn npm_installs_do_not_use_sudo() {
    for harness in load() {
        let download = harness.plan(Capability::Download).unwrap();
        let rendered = download.command.render();
        assert!(!rendered.contains("sudo"), "{} uses sudo", harness.name);
    }
}
