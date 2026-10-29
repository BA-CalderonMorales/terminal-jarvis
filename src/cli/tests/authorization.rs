use super::*;
use crate::cli::logic::test_support;
use crate::contracts::{Capability, Effect, EnvMode, SupportState};

fn harness() -> Harness {
    Harness {
        name: "test".into(),
        display: "test".into(),
        description: "test".into(),
        binary: "unused".into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![],
        sunset: None,
    }
}
#[test]
fn dangerous_opt_in_is_checked_before_any_prompt_or_spawn() {
    let harness = harness();
    let mut plan = test_support::plan(Capability::Headless, "unused", vec![]);
    plan.support = SupportState::Expected;
    plan.verified_at = crate::catalog::today_utc();
    plan.effect = Effect::Dangerous;
    let home = std::env::temp_dir();
    let options = Options::default();
    let request = Request {
        harness: &harness,
        plan: &plan,
        extra: &[],
        options: &options,
        home: &home,
        explicit: true,
    };
    let failure = check(request, &mut |_, _, _| panic!("must reject before asking"))
        .err()
        .unwrap();
    assert_eq!(failure.code, "dangerous_opt_in_required");
    let options = Options {
        dry_run: true,
        ..options
    };
    let request = Request {
        harness: &harness,
        plan: &plan,
        extra: &[],
        options: &options,
        home: &home,
        explicit: true,
    };
    assert!(
        check(request, &mut |_, _, _| panic!("preview must not ask"))
            .unwrap()
            .spawn()
            .is_err()
    );
}
#[cfg(unix)]
#[test]
fn conversation_stream_cannot_bypass_a_blocked_workspace_gate() {
    let _lock = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let root = std::env::temp_dir().join(format!("tj-converse-gate-{}", std::process::id()));
    std::fs::create_dir_all(root.join("gates/blocked")).unwrap();
    std::fs::write(root.join("gates/blocked/index.toml"),
        "name = \"blocked\"\ndisplay = \"blocked\"\ndescription = \"test\"\nbinary = \"sh\"\nargs = [\"-c\", \"echo finding; exit 1\"]\ninstall_hint = \"test\"\n").unwrap();
    let names = ["TERMINAL_JARVIS_GATE", "TERMINAL_JARVIS_GATES"];
    let saved = names.map(std::env::var_os);
    std::env::set_var(names[0], "blocked");
    std::env::set_var(names[1], root.join("gates"));
    let mut plan = test_support::plan(Capability::Headless, "unused", vec![]);
    plan.support = SupportState::Expected;
    plan.verified_at = crate::catalog::today_utc();
    let mut harness = harness();
    harness.capabilities = vec![plan];
    let result =
        crate::cli::headless_stream(&[harness], "test", "topic", &root, false, &mut |_| {
            panic!("read-only fixture must not ask")
        });
    for (name, value) in names.into_iter().zip(saved) {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
    std::fs::remove_dir_all(root).unwrap();
    assert!(result.err().unwrap().contains("gate_blocked"));
}
