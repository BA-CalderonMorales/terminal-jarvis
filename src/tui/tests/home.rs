use super::*;
use crate::contracts::Harness;
use std::path::Path;

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
fn home_renders_with_or_without_an_active_harness() {
    let harnesses = [harness("alpha"), harness("beta")];
    let root = std::env::temp_dir();
    let previous = crate::cli::style::set(true, true);
    render(&harnesses, Path::new("harnesses"), &root);
    render(&[], Path::new("harnesses"), &root);
    crate::cli::style::restore(previous);
}

#[test]
fn a_retired_harness_is_counted_apart_from_the_maintained_fleet() {
    let overview = Overview {
        name: "none".into(),
        cwd: "~".into(),
        ready: 1,
        total: 2,
        sunset: 1,
    };
    let line = plain(&overview);
    assert!(line.contains("READY 1/2 ready"), "{line}");
    assert!(line.contains("1 retired"), "{line}");
}
