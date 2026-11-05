use super::*;
use crate::contracts::{
    CapabilityPlan, CommandPlan, Effect, EnvMode, EvidenceMode, Interaction, SupportState,
};

fn harness() -> Harness {
    Harness {
        name: "alpha".into(),
        display: "Alpha".into(),
        description: "probe \"agent\"".into(),
        binary: "alpha".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![CapabilityPlan {
            capability: Capability::Download,
            summary: "npm install".into(),
            command: CommandPlan::new("alpha".into(), vec!["install".into()]),
            support: SupportState::Expected,
            evidence: EvidenceMode::Deterministic,
            effect: Effect::StateChanging,
            network: true,
            interaction: Interaction::Noninteractive,
            platforms: vec!["linux".into()],
            executable: "alpha".into(),
            source: "internal:catalog".into(),
            verified_at: "2026-10-01".into(),
            package: Some("alpha-cli".into()),
            uninstall: None,
        }],
        sunset: None,
    }
}

fn missing_home() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("tj-json-catalog-{}", std::process::id()))
}

#[test]
fn list_carries_text_beside_the_structured_rows() {
    let body = list(&[harness()]);
    assert_eq!(body.lines().count(), 1);
    assert!(body.contains("\"command\":\"list\""));
    assert!(body.contains("\"harnesses\":[{\"name\":\"alpha\""));
    assert!(body.contains("\"support\":{\"expected\":1}"));
    assert!(body.contains("\"capabilities\":[\"download\"]"));
    assert!(body.contains("probe \\\"agent\\\""));
}

#[test]
fn show_publishes_each_capability_with_its_truth() {
    let body = show(&harness());
    assert!(body.contains("\"harness\":{\"name\":\"alpha\""));
    assert!(body.contains("\"capability\":\"download\""));
    assert!(body.contains("\"command_line\":\"alpha install\""));
    assert!(body.contains("\"package\":\"alpha-cli\""));
    assert!(body.contains("\"verified_at\":\"2026-10-01\""));
    assert!(body.contains("\"platforms\":[\"linux\"]"));
}

#[test]
fn plan_publishes_the_command_as_argv() {
    let body = plan(&harness(), Capability::Download);
    assert!(body.contains("\"capability\":\"download\""));
    assert!(body.contains("\"command\":[\"alpha\",\"install\"]"));
    assert!(body.contains("\"network\":true"));
    assert!(body.contains("\"effect\":\"state-changing\""));
}

#[test]
fn unresolvable_or_uncatalogued_actions_return_no_document() {
    let harnesses = [harness()];
    let home = missing_home();
    assert!(document(&args::Action::Check, &harnesses, &home).is_none());
    assert!(document(&args::Action::Show(None), &harnesses, &home).is_none());
    assert!(document(&args::Action::Show(Some("beta".into())), &harnesses, &home).is_none());
}
