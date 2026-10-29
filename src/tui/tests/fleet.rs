use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

fn home() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "terminal-jarvis-fleet-{}-{}",
        std::process::id(),
        TEMP_ID.fetch_add(1, Ordering::Relaxed)
    ))
}

fn harness(name: &str) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: "probe".into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

#[test]
fn nothing_is_ready_before_a_readiness_record_exists() {
    let home = home();
    let harnesses = [harness("alpha"), harness("beta")];
    assert!(ready(&harnesses, &home, &home).is_empty());
}

#[test]
fn active_follows_the_saved_session() {
    let home = home();
    assert_eq!(active(&home), None);
    crate::context::save(&home, "beta").unwrap();
    assert_eq!(active(&home), Some("beta".to_string()));
    let _ = std::fs::remove_dir_all(&home);
}
