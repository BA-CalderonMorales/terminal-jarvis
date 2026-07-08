use std::fs;
use std::path::PathBuf;

use super::*;
use crate::contracts::{Capability, EnvMode, Harness};

fn tmp_home(active: &str) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("tjresolve-{}-{}-{}", std::process::id(), active, n));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("session.toml"),
        format!("active_harness = \"{active}\"\n"),
    )
    .unwrap();
    dir
}

fn harness(name: &str) -> Harness {
    Harness {
        name: name.to_string(),
        display: name.to_string(),
        description: String::new(),
        binary: name.to_string(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

fn s(value: &str) -> String {
    value.to_string()
}

#[test]
fn prompt_starting_with_capability_runs_headless_not_capability() {
    let home = tmp_home("opencode");
    let harnesses = vec![harness("opencode")];
    let inv = run(&[s("update"), s("my"), s("database")], &harnesses, &home).unwrap();
    assert_eq!(inv.harness, "opencode");
    assert_eq!(inv.capability, Capability::Headless);
    assert_eq!(inv.extra, vec![s("update"), s("my"), s("database")]);
}

#[test]
fn dangerous_capability_prompt_is_not_executed() {
    let home = tmp_home("opencode");
    let harnesses = vec![harness("opencode")];
    let inv = run(&[s("yolo"), s("clean"), s("tmp")], &harnesses, &home).unwrap();
    assert_eq!(inv.capability, Capability::Headless);
    assert_eq!(inv.extra, vec![s("yolo"), s("clean"), s("tmp")]);
}

#[test]
fn single_capability_word_still_runs_that_capability() {
    let home = tmp_home("opencode");
    let harnesses = vec![harness("opencode")];
    let inv = run(&[s("version")], &harnesses, &home).unwrap();
    assert_eq!(inv.capability, Capability::Version);
    assert!(inv.extra.is_empty());
}

#[test]
fn headless_keyword_form_runs_headless_capability() {
    let home = tmp_home("opencode");
    let harnesses = vec![harness("opencode")];
    let inv = run(&[s("headless"), s("summarize")], &harnesses, &home).unwrap();
    assert_eq!(inv.capability, Capability::Headless);
    assert_eq!(inv.extra, vec![s("summarize")]);
}

#[test]
fn explicit_harness_capability_is_preserved() {
    let home = tmp_home("opencode");
    let harnesses = vec![harness("opencode")];
    let inv = run(&[s("opencode"), s("version")], &harnesses, &home).unwrap();
    assert_eq!(inv.harness, "opencode");
    assert_eq!(inv.capability, Capability::Version);
}
