use super::*;
use crate::diagnostics::Environment;
use crate::diagnostics::{DiagnosticInput, PlatformInput, RuntimeInput};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn input(platform: PlatformInput, environment: Environment) -> DiagnosticInput {
    let base = PathBuf::from("/probe");
    DiagnosticInput {
        version: "v0.1.13".into(),
        executable: None,
        catalog: base.clone(),
        home: base.clone(),
        config: base.join("config.toml"),
        home_prefix: None,
        temp_prefix: None,
        active_harness: None,
        harnesses: vec![],
        platform,
        environment,
        runtime: RuntimeInput::default(),
        now: SystemTime::now(),
        stale_after: Duration::from_secs(0),
    }
}

fn platform() -> PlatformInput {
    PlatformInput {
        os: "linux".into(),
        arch: "x86_64".into(),
        libc: "gnu".into(),
        wsl: "no".into(),
    }
}

#[test]
fn wsl_states_map_to_ready_error_and_malformed() {
    let mut environment = Environment::process();
    for (wsl, expected) in [
        ("no", Code::Ready),
        ("wsl2", Code::Ready),
        ("wsl1-or-unknown", Code::Unsupported),
        ("plan9", Code::Malformed),
    ] {
        let mut input = input(platform(), environment.clone());
        input.platform.wsl = wsl.into();
        let (records, _) = collect(&mut input);
        let record = records.iter().find(|r| r.key == "platform.wsl").unwrap();
        assert_eq!(record.code, expected, "{wsl}");
    }
}

#[test]
fn shell_record_prefers_basename_of_the_present_variable() {
    let mut environment = Environment::process();
    environment.insert("SHELL", "/bin/zsh");
    let (records, _) = collect(&mut input(platform(), environment));
    let record = records.iter().find(|r| r.key == "platform.shell").unwrap();
    assert_eq!(record.code, Code::Ready);
    assert_eq!(record.value, "zsh");
}

#[test]
fn unsupported_targets_carry_a_recovery_action() {
    let mut input = input(
        PlatformInput {
            os: "plan9".into(),
            ..platform()
        },
        Environment::process(),
    );
    let (records, supported) = collect(&mut input);
    assert!(!supported);
    let record = records.iter().find(|r| r.key == "platform.target").unwrap();
    assert!(record.action.is_some());
}
