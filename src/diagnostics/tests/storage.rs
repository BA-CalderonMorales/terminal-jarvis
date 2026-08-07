use super::*;
use crate::diagnostics::{Environment, HarnessInput, PlatformInput, RuntimeInput};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn tempdir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tj-storage-{}-{tag}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn input(catalog: PathBuf, home: PathBuf, harnesses: Vec<HarnessInput>) -> DiagnosticInput {
    DiagnosticInput {
        version: "v0.1.13".into(),
        executable: None,
        catalog,
        home: home.clone(),
        config: home.join("config.toml"),
        home_prefix: None,
        temp_prefix: None,
        active_harness: None,
        harnesses,
        platform: PlatformInput {
            os: "linux".into(),
            arch: "x86_64".into(),
            libc: "gnu".into(),
            wsl: "no".into(),
        },
        environment: Environment::process(),
        runtime: RuntimeInput::default(),
        now: SystemTime::now(),
        stale_after: Duration::from_secs(0),
    }
}

fn harness(name: &str) -> HarnessInput {
    HarnessInput {
        name: name.into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        support: vec![],
        version: None,
    }
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
fn empty_harness_list_marks_the_catalog_the_big_hole() {
    let dir = tempdir("empty");
    let catalog = dir.join("catalog");
    fs::create_dir_all(&catalog).unwrap();
    let (records, _, _) = collect(
        &input(catalog, dir.clone(), vec![]),
        &Redactor::new(None, None),
    );
    let record = records.iter().find(|r| r.key == "state.catalog").unwrap();
    assert_eq!(record.code, Code::Empty);
}

#[test]
fn ready_state_reports_valid_without_spurious_actions() {
    let dir = tempdir("ready");
    let catalog = dir.join("catalog");
    fs::create_dir_all(&catalog).unwrap();
    fs::write(catalog.join("index.toml"), "harness = \"opencode\"\n").unwrap();
    fs::write(dir.join("config.toml"), "active_harness = \"opencode\"\n").unwrap();
    let (records, _, valid) = collect(
        &input(catalog, dir.clone(), vec![harness("opencode")]),
        &Redactor::new(None, None),
    );
    assert!(valid);
    let record = records.iter().find(|r| r.key == "state.catalog").unwrap();
    assert_eq!(record.code, Code::Ready);
    assert!(record.action.is_none());
}

#[test]
fn absent_cache_state_is_not_applicable() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("TERMINAL_JARVIS_CACHE");
    let dir = tempdir("cache");
    let (records, _, _) = collect(
        &input(dir.join("catalog"), dir, vec![]),
        &Redactor::new(None, None),
    );
    let record = records.iter().find(|r| r.key == "state.cache").unwrap();
    assert_eq!(record.code, Code::Ready);
    assert_eq!(record.value, "not-applicable");
}
