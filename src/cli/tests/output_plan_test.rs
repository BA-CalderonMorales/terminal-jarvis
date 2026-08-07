use super::*;
use crate::cli::logic::test_support;
use crate::contracts::{Capability, Effect, EnvMode, Harness};

fn harness(effect: Effect) -> Harness {
    let mut plan = test_support::plan(Capability::Download, "sh", vec!["probe".into()]);
    plan.effect = effect;
    Harness {
        name: "opencode".into(),
        display: "opencode".into(),
        description: String::new(),
        binary: "sh".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![plan],
    }
}

#[test]
fn intent_is_exact_per_effect() {
    let cases = [
        (Effect::ReadOnly, "none"),
        (
            Effect::StateChanging,
            "--no-input --confirm=download:opencode",
        ),
        (
            Effect::Dangerous,
            "--allow-dangerous --no-input --confirm=download:opencode",
        ),
    ];
    for (case, expected) in cases {
        let h = harness(case);
        let plan = &h.capabilities[0];
        assert_eq!(intent(&h, plan), expected);
        let fields = fields(&h, plan, &plan.command);
        assert_eq!(
            fields.iter().find(|(name, _)| *name == "INTENT"),
            Some(&("INTENT", expected.to_string()))
        );
    }
}

#[test]
fn plain_renders_stable_readonly_text() {
    let h = harness(Effect::ReadOnly);
    assert_eq!(
        plain(&h, &h.capabilities[0], &h.capabilities[0].command),
        "opencode:download\n\
         summary: download\n\
         support: unknown\n\
         evidence: deterministic\n\
         effect: read-only network=true interaction=noninteractive\n\
         platforms: none\n\
         executable: sh\n\
         source: internal:test-fixture\n\
         verified_at: 2026-07-17T04:59:27Z (policy-reviewed)\n\
         command: sh probe\n\
         env: no API key required\n\
         intent: none\n"
    );
}
