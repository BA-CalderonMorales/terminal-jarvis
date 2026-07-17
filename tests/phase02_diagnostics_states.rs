#![cfg(unix)]

#[path = "phase02_diagnostics_support/states.rs"]
mod phase02_diagnostics_states_support;

use phase02_diagnostics_states_support::{assert_code, fixture};
use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::time::{Duration, SystemTime};
use terminal_jarvis::contracts::EnvMode;
use terminal_jarvis::diagnostics::{
    collect, Code, DiagnosticInput, Environment, HarnessInput, PlatformInput, RuntimeInput,
};

#[test]
fn stable_codes_keep_failure_classes_distinct() {
    let root = fixture();
    let catalog = root.join("catalog");
    let home = root.join("home");
    let bin = root.join("bin");
    fs::create_dir_all(&catalog).unwrap();
    fs::create_dir_all(&home).unwrap();
    fs::create_dir_all(&bin).unwrap();
    fs::write(catalog.join("marker"), "catalog").unwrap();
    fs::write(home.join("session.toml"), "active_harness = \"other\"\n").unwrap();
    fs::write(root.join("cache"), "cache").unwrap();
    fs::write(bin.join("tj"), "program").unwrap();
    fs::write(bin.join("fixture"), "not executable").unwrap();
    let mut environment = Environment::default();
    environment.insert("PATH", std::env::join_paths([&bin]).unwrap());
    environment.insert("SHELL", "/bin/sh");
    environment.insert("TERMINAL_JARVIS_DISTRIBUTION", "source");
    environment.insert("TERMINAL_JARVIS_WRAPPER", "npm-wrapper-secret");
    environment.insert("TERMINAL_JARVIS_CACHE", root.join("cache"));
    environment.insert("EMPTY_TOKEN", "  ");
    environment.insert("BROKEN_TOKEN", OsString::from_vec(vec![0xff, 0xfe]));
    let input = DiagnosticInput {
        version: "test".into(),
        executable: Some(bin.join("tj")),
        catalog: catalog.clone(),
        home: home.clone(),
        config: home.join("session.toml"),
        home_prefix: Some(home.clone()),
        temp_prefix: Some(root.clone()),
        active_harness: Some("fixture".into()),
        harnesses: vec![HarnessInput {
            name: "fixture".into(),
            binary: "fixture".into(),
            env_mode: EnvMode::All,
            env: vec![
                "EMPTY_TOKEN".into(),
                "MISSING_TOKEN".into(),
                "BROKEN_TOKEN".into(),
            ],
            support: vec![("headless".into(), "unknown".into(), false)],
        }],
        platform: PlatformInput {
            os: "linux".into(),
            arch: "x86_64".into(),
            libc: "musl".into(),
            wsl: "wsl1-or-unknown".into(),
        },
        environment,
        runtime: RuntimeInput::default(),
        now: SystemTime::now() + Duration::from_secs(2),
        stale_after: Duration::ZERO,
    };
    let report = collect(&input);
    assert!(!report.ok && report.exit_code() == 4);
    assert_code(&report, "tj.distribution", Code::Conflicting);
    assert_code(&report, "platform.target", Code::Unsupported);
    assert_code(&report, "state.cache", Code::Stale);
    assert_code(&report, "state.config", Code::Conflicting);
    assert_code(
        &report,
        "harness.fixture.executable",
        Code::PermissionDenied,
    );
    assert_code(&report, "harness.fixture.support", Code::Unsupported);
    assert_code(&report, "harness.fixture.readiness", Code::Unsupported);
    assert_code(&report, "harness.fixture.env.EMPTY_TOKEN", Code::Empty);
    assert_code(&report, "harness.fixture.env.MISSING_TOKEN", Code::Missing);
    assert_code(&report, "harness.fixture.env.BROKEN_TOKEN", Code::Malformed);
    let output = format!("{}{}", report.plain(), report.json());
    assert!(!output.contains("npm-wrapper-secret"));
    fs::remove_dir_all(root).unwrap();
}
