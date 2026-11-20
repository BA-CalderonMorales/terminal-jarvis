use super::*;

#[test]
fn resolved_interaction_owns_transport_not_word_count() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("harnesses");
    let mut harnesses = crate::catalog::load(&root).unwrap();
    let home = std::env::temp_dir();
    for words in [vec!["codex"], vec!["codex", "ui"]] {
        let action = args::Action::Run(words.into_iter().map(String::from).collect());
        assert!(!eligible(&action, &harnesses, &home));
    }
    let headless = args::Action::Run(vec!["codex".into(), "headless".into(), "hello".into()]);
    assert!(eligible(&headless, &harnesses, &home));
    let codex = harnesses.iter_mut().find(|h| h.name == "codex").unwrap();
    let plan = codex
        .capabilities
        .iter_mut()
        .find(|p| p.capability == crate::contracts::Capability::Headless)
        .unwrap();
    plan.interaction = crate::contracts::Interaction::Interactive;
    assert!(!eligible(&headless, &harnesses, &home));
}

#[test]
fn direct_ui_arguments_keep_the_terminal_and_explicit_headless_streams() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("harnesses");
    let harnesses = crate::catalog::load(&root).unwrap();
    let home = std::env::temp_dir();
    for words in [vec!["--help"], vec!["ui", "--help"]] {
        let action = args::Action::Direct {
            harness: "codex".into(),
            extra: words.into_iter().map(String::from).collect(),
        };
        assert!(!eligible(&action, &harnesses, &home));
    }
    let action = args::Action::Direct {
        harness: "codex".into(),
        extra: vec!["headless".into(), "hello".into()],
    };
    assert!(eligible(&action, &harnesses, &home));
}
