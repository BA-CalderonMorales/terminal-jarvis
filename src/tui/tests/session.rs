use super::*;

#[test]
fn chapter_and_recap_render_the_frame() {
    assert_eq!(chapter("run codex --monitor"), "── run codex --monitor ──");
    assert_eq!(
        recap("codex", Some(0), std::time::Duration::from_millis(12_400)),
        "── codex exited 0 · 12.4s ──"
    );
    assert_eq!(
        recap("codex", Some(3), std::time::Duration::from_millis(500)),
        "── codex exited 3 · 0.5s ──"
    );
    assert_eq!(
        recap("codex", None, std::time::Duration::from_millis(100)),
        "── codex failed · 0.1s ──"
    );
}

#[test]
fn frame_wraps_a_direct_harness_with_header_and_recap() {
    let start = std::time::Instant::now();
    let mut sink = Vec::new();
    frame(
        args::Action::Direct {
            harness: "vibe".into(),
            extra: vec![],
        },
        &options(),
        &harness(),
        std::path::Path::new("."),
        std::path::Path::new("."),
        &mut sink,
    );
    let rendered = String::from_utf8(sink).unwrap();
    let lines = rendered
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(lines.first().copied(), Some("── vibe ──"));
    assert!(
        lines.last().unwrap().starts_with("── vibe exited 0 · "),
        "last: {rendered:?}"
    );
    assert!(lines.last().unwrap().ends_with(" ──"), "last: {rendered:?}");
    assert!(start.elapsed().as_secs() < 10, "frame must not hang");
}

fn options() -> args::Options {
    args::Options {
        narrate: false,
        ..args::Options::default()
    }
}

fn harness() -> Vec<Harness> {
    vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "test fixture".into(),
        binary: "true".into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![plan()],
    }]
}

fn plan() -> crate::contracts::CapabilityPlan {
    crate::contracts::CapabilityPlan {
        capability: crate::contracts::Capability::Ui,
        summary: "run the fixture".into(),
        command: crate::contracts::CommandPlan::new("true".into(), vec![]),
        support: crate::contracts::SupportState::Verified,
        evidence: crate::contracts::EvidenceMode::Deterministic,
        effect: crate::contracts::Effect::ReadOnly,
        network: false,
        interaction: crate::contracts::Interaction::Noninteractive,
        platforms: vec![],
        executable: "true".into(),
        source: "internal:test-fixture".into(),
        verified_at: "2026-08-06T00:00:00Z".to_string(),
        package: None,
    }
}
