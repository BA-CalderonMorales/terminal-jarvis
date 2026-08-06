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
fn render_is_a_three_line_dashboard() {
    let previous = crate::cli::style::set(true, true);
    let root = std::env::temp_dir();
    let harnesses = [harness("alpha"), harness("beta")];
    let body = render(&harnesses, &root, &root);
    assert!(body.contains("ACTIVE"));
    assert!(body.contains("READY"));
    assert!(body.contains("of 2 ready"));
    assert_eq!(body.lines().count(), 2);
    crate::cli::style::restore(previous);
}
