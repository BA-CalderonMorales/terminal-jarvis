use super::*;
use crate::contracts::Harness;
use crate::diagnostics::RuntimeInput;
use std::fs;
use std::path::{Path, PathBuf};

fn tempdir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tj-storage-{}-{tag}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn catalog(dir: &Path) -> PathBuf {
    let catalog = dir.join("catalog");
    fs::create_dir_all(&catalog).unwrap();
    fs::write(catalog.join("index.toml"), "harness = \"opencode\"\n").unwrap();
    catalog
}

fn input(catalog: &Path, dir: &Path, harnesses: &[Harness]) -> DiagnosticInput {
    DiagnosticInput::local(catalog, dir, None, harnesses, RuntimeInput::default())
}

fn harness() -> Harness {
    Harness {
        name: "opencode".into(),
        display: "opencode".into(),
        description: "probe".into(),
        binary: "opencode".into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

fn collected(catalog: &Path, dir: &Path, harnesses: &[Harness]) -> (Vec<Record>, bool) {
    let (records, _, valid) = collect(&input(catalog, dir, harnesses), &Redactor::new(None, None));
    (records, valid)
}

fn raw(catalog: &Path, dir: &Path, harnesses: &[Harness]) -> (DiagnosticInput, Redactor<'static>) {
    let input = input(catalog, dir, harnesses);
    (input, Redactor::new(None, None))
}

fn record<'r>(records: &'r [Record], key: &str) -> &'r Record {
    records.iter().find(|r| r.key == key).unwrap()
}

#[test]
fn severity_maps_every_code_class() {
    assert_eq!(severity(Code::Ready), Severity::Info);
    for code in [Code::Missing, Code::Empty, Code::Stale, Code::Unknown] {
        assert_eq!(severity(code), Severity::Warning, "{code:?}");
    }
    assert_eq!(severity(Code::PermissionDenied), Severity::Error);
}

#[test]
fn catalog_empty_without_harnesses_and_ready_with_one() {
    let dir = tempdir("cat");
    let path = catalog(&dir);
    let (records, _) = collected(&path, &dir, &[]);
    assert_eq!(record(&records, "state.catalog").code, Code::Empty);
    let (records, valid) = collected(&path, &dir, &[harness()]);
    assert!(valid);
    assert_eq!(record(&records, "state.catalog").code, Code::Ready);
    assert!(record(&records, "state.catalog").action.is_none());
}

#[test]
fn ready_state_reports_valid_without_spurious_actions() {
    let dir = tempdir("ready");
    let path = catalog(&dir);
    fs::write(dir.join("config.toml"), "active_harness = \"opencode\"\n").unwrap();
    let (records, valid) = collected(&path, &dir, &[harness()]);
    assert!(valid);
    assert_eq!(record(&records, "state.home").code, Code::Ready);
    assert!(record(&records, "state.home").action.is_none());
    assert!(record(&records, "state.catalog").code != Code::Empty);
}

#[test]
fn absent_cache_state_is_not_applicable() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // The not-applicable verdict requires the source channel: distribution
    // tests run in parallel and would flip it to the npm branch otherwise.
    std::env::remove_var("TERMINAL_JARVIS_DISTRIBUTION");
    std::env::remove_var("TERMINAL_JARVIS_WRAPPER");
    let dir = tempdir("cache");
    let (mut input, redact) = raw(&dir.join("catalog"), &dir, &[]);
    // Mutate the snapshot, not the process env: parallel tests can never
    // make the cache "present" between here and the collect.
    input.environment.remove("TERMINAL_JARVIS_CACHE");
    let (records, _, _) = collect(&input, &redact);
    let cache = record(&records, "state.cache");
    assert_eq!(cache.code, Code::Ready);
    assert_eq!(cache.value, "not-applicable");
}
