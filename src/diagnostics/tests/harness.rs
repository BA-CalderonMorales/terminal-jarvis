use super::*;
use crate::contracts::EnvMode;
use crate::diagnostics::{Environment, RuntimeInput};
use std::fs;
use std::path::{Path, PathBuf};

fn temp_bin(name: &str) -> PathBuf {
    let directory =
        std::env::temp_dir().join(format!("tj_harness_{}_{}", std::process::id(), name));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();
    let binary = directory.join(name);
    fs::write(&binary, "").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&binary, perms).unwrap();
    }
    directory
}
fn probing_harness() -> HarnessInput {
    HarnessInput {
        name: "xh".into(),
        binary: "xh".into(),
        env_mode: EnvMode::None,
        env: Vec::new(),
        support: vec![("run".into(), "ok".into(), true)],
        version: Some(("sh".into(), vec!["-c".into(), "printf 'v1.2.3\\n'".into()])),
    }
}

fn input(bin: &Path) -> DiagnosticInput {
    let mut local = DiagnosticInput::local(
        Path::new("/tmp/tj-catalog"),
        Path::new("/tmp/tj-home"),
        None,
        &[],
        RuntimeInput {
            gate: "/tmp/tj-gate".into(),
            stdout_tty: true,
            stderr_tty: true,
            color: false,
            width: 80,
            update_route: "source".into(),
            checksum: "".into(),
            probes: false,
        },
    );
    let mut environment = Environment::default();
    environment.insert("PATH", std::env::join_paths([bin]).unwrap());
    local.environment = environment;
    local.harnesses = vec![probing_harness()];
    local
}

fn resolution(code: Code) -> super::super::resolve::Resolution {
    super::super::resolve::Resolution {
        code,
        path: None,
        paths: Vec::new(),
        matches: 0,
    }
}

#[test]
fn collect_reports_ready_harness() {
    let bin = temp_bin("xh");
    let result = collect(&input(&bin), &Redactor::new(None, None));
    let exec = result
        .records
        .iter()
        .find(|r| r.key == "harness.xh.executable")
        .unwrap();
    assert_eq!(exec.severity, Severity::Info);
    assert_eq!(exec.action, None);
    assert!(result.ready.contains("xh"));
    let _ = fs::remove_dir_all(&bin);
}

fn probe(code: Code, probes: bool) -> (Code, String) {
    let record = version_record("harness.xh", &probing_harness(), &resolution(code), probes);
    (record.code, record.value)
}

#[test]
fn version_record_gates_probing() {
    assert_eq!(probe(Code::Ready, true), (Code::Ready, "v1.2.3".into()));
    assert_eq!(
        probe(Code::Missing, true),
        (Code::Unknown, "unknown:not-probed".into())
    );
    assert_eq!(
        probe(Code::Ready, false),
        (Code::Unknown, "unknown:not-probed".into())
    );
}
