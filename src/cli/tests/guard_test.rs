use super::*;

fn harnesses() -> Vec<Harness> {
    vec![Harness {
        name: "opencode".to_string(),
        display: "opencode".to_string(),
        description: String::new(),
        binary: "opencode".to_string(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }]
}

#[test]
fn explicit_capability_requires_known_harness_and_parsable_capability() {
    let hs = harnesses();
    assert!(!explicit_capability(&[], &hs));
    assert!(!explicit_capability(
        &["other".into(), "install".into()],
        &hs
    ));
    assert!(!explicit_capability(
        &["opencode".into(), "bogus".into()],
        &hs
    ));
    assert!(explicit_capability(
        &["opencode".into(), "download".into()],
        &hs
    ));
}

#[test]
fn resolve_error_distinguishes_active_harness_state_from_unknown() {
    assert_eq!(
        resolve_error("no active harness for this run".into()).exit_code,
        3
    );
    assert_eq!(
        resolve_error("active harness has no confirmed session".into()).exit_code,
        3
    );
    assert_eq!(resolve_error("wow such failure".into()).exit_code, 4);
}

#[test]
fn unknown_direct_command_advises_exactly_once() {
    let failure = resolve_error("unknown command or harness 'bogus'".into());
    let rendered = failure.rendered();
    assert_eq!(rendered.matches("run `terminal-jarvis list`").count(), 1);
}

#[test]
fn streaming_invocation_preserves_the_child_exit_code() {
    let mut plan = crate::cli::logic::test_support::plan(
        Capability::Headless,
        "sh",
        vec!["-c".into(), "printf streamed; exit 7".into()],
    );
    plan.support = crate::contracts::SupportState::Verified;
    plan.verified_at = "2026-08-05T00:00:00Z".into();
    let harnesses = vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "test fixture".into(),
        binary: "sh".into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![plan],
    }];
    let home = std::env::temp_dir().join(format!("tj-stream-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    let mut lines = Vec::new();
    let result = stream_invocation(
        crate::cli::logic::resolve::Invocation {
            harness: "vibe".into(),
            capability: Capability::Headless,
            extra: vec![],
        },
        &Options::default(),
        &harnesses,
        &home,
        &mut |line| lines.push(line.to_string()),
    );
    assert_eq!(result, Ok(7));
    assert!(lines.iter().any(|line| line.contains("streamed")));
    let _ = std::fs::remove_dir_all(home);
}
