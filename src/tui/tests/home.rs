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
fn cwd_label_roots_at_home() {
    assert_eq!(
        cwd_label_for("/home/caldo/work/terminal-jarvis", Some("/home/caldo")),
        "~/work/terminal-jarvis"
    );
    assert_eq!(cwd_label_for("/usr/local/bin", Some("/home/caldo")), "/usr/local/bin");
}

#[test]
fn cwd_label_ellipsizes_long_paths_at_component_boundaries() {
    let home = Some("/home/caldo");
    let dotted = cwd_label_for(
        "/home/caldo/world/repositories/working/terminal-jarvis",
        home,
    );
    assert!(dotted.starts_with(".../"));
    assert!(dotted.ends_with("terminal-jarvis"));
    assert!(dotted.chars().count() <= 32);
    assert!(dotted.starts_with(".../working/terminal-jarvis"));
}
