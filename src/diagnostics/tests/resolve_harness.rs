//! Resolve harness: shared fixture builders for the resolve test tree.

use super::{Code, DiagnosticInput, Resolution};
use crate::diagnostics::{Environment, PlatformInput, RuntimeInput};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn dummy_input() -> DiagnosticInput {
    DiagnosticInput {
        version: "0.1.14".into(),
        executable: None,
        catalog: PathBuf::from("/tmp/catalog"),
        home: PathBuf::from("/tmp/home"),
        config: PathBuf::from("/tmp/config"),
        home_prefix: None,
        temp_prefix: None,
        active_harness: None,
        harnesses: vec![],
        platform: PlatformInput {
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            libc: crate::context::platform::libc().into(),
            wsl: crate::context::platform::wsl().into(),
        },
        environment: Environment::process(),
        runtime: RuntimeInput {
            gate: PathBuf::from("/tmp/gate"),
            stdout_tty: true,
            stderr_tty: true,
            color: false,
            width: 80,
            update_route: "source".into(),
            checksum: "".into(),
            probes: true,
        },
        now: SystemTime::now(),
        stale_after: Duration::from_secs(300),
    }
}

pub fn temp_executable() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let test_file = std::env::temp_dir().join(format!("resolve_test_binary_{n}"));
    std::fs::write(&test_file, "").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&test_file, perms).unwrap();
    }
    test_file
}

pub fn assert_ready(result: &Resolution) {
    assert_eq!(result.code, Code::Ready);
    assert_eq!(result.matches, 1);
    assert!(result.path.is_some());
}
