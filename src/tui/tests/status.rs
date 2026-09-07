use super::*;
use crate::contracts::Harness;

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
fn render_groups_the_fleet_into_ready_and_pending() {
    let previous = crate::cli::style::set(true, true);
    let root = std::env::temp_dir();
    let harnesses = [harness("alpha"), harness("beta")];
    let body = render(&harnesses, &root, &root);
    assert!(body.contains("ACTIVE"));
    assert!(body.contains("0 of 2 ready"));
    assert!(body.contains("one install away"));
    assert!(body.contains("alpha, beta"));
    // zero ready skips the "ready now" group entirely
    assert_eq!(body.lines().count(), 5);
    crate::cli::style::restore(previous);
}
