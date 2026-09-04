//! Output-test harness: mock harness builders and PATH fixtures shared by
//! the readiness/status tests. Imports ride `super::*`.

use super::*;
use crate::contracts::EnvMode;
use std::path::{Path, PathBuf};

pub fn tmpdir() -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("tjharnesstest_{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn mock_binary_on_path(tmpdir: &Path) -> String {
    let bin = tmpdir.join("mock-harness");
    std::fs::write(&bin, "#!/bin/sh\necho ok").unwrap();
    make_executable(&bin);
    let old = std::env::var("PATH").unwrap_or_default();
    let joined = std::env::join_paths(
        std::iter::once(tmpdir.to_path_buf()).chain(std::env::split_paths(&old)),
    )
    .expect("PATH entries join");
    std::env::set_var("PATH", joined);
    old
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
}

/// Windows has no execute bit; `command_on_path` treats any existing file as
/// runnable there (see `security::path`), so there is nothing to mark.
#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

pub fn mock_harness(binary: &str, env_mode: EnvMode, env: Vec<String>) -> Harness {
    let mut plan = crate::cli::logic::test_support::plan(
        crate::contracts::Capability::Version,
        binary,
        vec!["--version".into()],
    );
    plan.support = crate::contracts::SupportState::Expected;
    plan.platforms = vec![crate::context::platform::id().unwrap().into()];
    Harness {
        name: "x".into(),
        display: "X".into(),
        description: "".into(),
        binary: binary.into(),
        env_mode,
        env,
        capabilities: vec![plan],
    }
}
