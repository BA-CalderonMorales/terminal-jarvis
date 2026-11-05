use super::*;

#[test]
fn direct_commands_preserve_the_resolved_capability_and_arguments() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("harnesses");
    let harnesses = crate::catalog::load(&root).unwrap();
    let home = std::env::temp_dir();
    let mut state = super::super::state::LoopState {
        body: vec![],
        history: vec![],
        converse: None,
        pending_converse: None,
        offset: 0,
        hint: String::new(),
        bottom: false,
        options: args::Options::default(),
        debug: false,
        indicator: crate::tui::input::Indicator {
            active: "codex".into(),
            debug: false,
        },
    };
    for capability in [Capability::Version, Capability::Headless, Capability::Ui] {
        let action = args::Action::Direct {
            harness: "codex".into(),
            extra: vec![capability.as_str().into(), "payload".into()],
        };
        let plan = for_action(&action, &mut state, &harnesses, &home).unwrap();
        assert_eq!(plan.invocation.capability, capability);
        assert_eq!(plan.invocation.extra, ["payload"]);
    }
    let invalid = args::Action::Direct {
        harness: "not-in-the-catalog".into(),
        extra: vec![],
    };
    assert!(for_action(&invalid, &mut state, &harnesses, &home).is_none());
    assert_eq!(state.body.len(), 1);
}
