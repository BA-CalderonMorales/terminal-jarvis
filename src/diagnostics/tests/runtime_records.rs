use super::*;
use crate::diagnostics::Environment;
use crate::diagnostics::{DiagnosticInput, PlatformInput, RuntimeInput};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, SystemTime};

static GATE_SEQ: AtomicUsize = AtomicUsize::new(0);

fn gate_dir() -> PathBuf {
    let seq = GATE_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("tj-runtime-{}-gates-{seq}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let runtime_gates = dir.join("gates");
    fs::create_dir_all(&runtime_gates).unwrap();
    fs::write(
        runtime_gates.join("trivy.toml"),
        "[gate]\nname = \"trivy\"\n",
    )
    .unwrap();
    dir
}

fn input(mut gate: PathBuf, mut runtime: RuntimeInput) -> DiagnosticInput {
    runtime.gate = gate.join("gates");
    gate.push("catalog");
    DiagnosticInput {
        version: "v0.1.14".into(),
        executable: None,
        catalog: gate.clone(),
        home: gate.clone(),
        config: gate.join("config.toml"),
        home_prefix: None,
        temp_prefix: None,
        active_harness: None,
        harnesses: vec![],
        platform: PlatformInput {
            os: "linux".into(),
            arch: "x86_64".into(),
            libc: "gnu".into(),
            wsl: "no".into(),
        },
        environment: Environment::process(),
        runtime,
        now: SystemTime::now(),
        stale_after: Duration::from_secs(0),
    }
}

#[test]
fn status_code_pins_ready_and_error_records() {
    let ready = status_code("k", Code::Ready, "v", "a");
    assert_eq!(ready.code, Code::Ready);
    assert!(ready.action.is_none());
    let broken = status_code("k", Code::Missing, "v", "a");
    assert_eq!(broken.code, Code::Missing);
    assert_eq!(broken.action.as_deref(), Some("a"));
}

#[test]
fn unknown_checksum_is_marked_not_clear() {
    let gates = gate_dir();
    let input = input(
        gates.clone(),
        RuntimeInput {
            checksum: "unknown".into(),
            ..RuntimeInput::default()
        },
    );
    let (records, ok) = collect(&input, &Redactor::new(None, None));
    assert!(ok);
    let record = records
        .iter()
        .find(|r| r.key == "distribution.checksum")
        .unwrap();
    assert_eq!(record.code, Code::Unknown);
}

#[test]
fn every_health_flag_must_be_true_for_a_clean_report() {
    let gates = gate_dir();
    let healthy = RuntimeInput::default();
    let (_, ok) = collect(&input(gates.clone(), healthy), &Redactor::new(None, None));
    assert!(ok);
    let narrow = RuntimeInput {
        width: 10,
        ..RuntimeInput::default()
    };
    let (_, ok) = collect(&input(gates, narrow), &Redactor::new(None, None));
    assert!(!ok);
}
