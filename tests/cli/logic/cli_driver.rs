use crate::structs::catalog;
use crate::structs::gate;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Fixture {
    pub root: PathBuf,
    marker: PathBuf,
    gate_marker: PathBuf,
}

#[allow(dead_code)]
impl Fixture {
    pub fn new(download: &str, yolo: &str, script: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "terminal-jarvis-fixture-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&root);
        let bin = root.join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        let child = bin.join(script_filename("fixture-child"));
        std::fs::write(&child, script).unwrap();
        make_executable(&child);
        let gate_child = bin.join(script_filename("fixture-gate"));
        std::fs::write(&gate_child, GATE_MARKER_SCRIPT).unwrap();
        make_executable(&gate_child);
        catalog::write(&root.join("catalog"), download, yolo);
        gate::write(&root.join("gates"));
        Self {
            marker: root.join("spawned"),
            gate_marker: root.join("gate-spawned"),
            root,
        }
    }

    pub fn run(&self, args: &[&str]) -> Output {
        let path = prepend_to_path(&self.root.join("bin"));
        Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
            .args(args)
            .env("PATH", path)
            .env("TERMINAL_JARVIS_CATALOG", self.root.join("catalog"))
            .env("TERMINAL_JARVIS_GATE", "acceptance")
            .env("TERMINAL_JARVIS_GATES", self.root.join("gates"))
            .env("TERMINAL_JARVIS_HOME", self.root.join("home"))
            .env("TJ_FIXTURE_MARKER", &self.marker)
            .env("TJ_FIXTURE_GATE_MARKER", &self.gate_marker)
            .output()
            .expect("terminal-jarvis runs")
    }

    pub fn spawned(&self) -> bool {
        self.marker.exists()
    }

    pub fn gate_spawned(&self) -> bool {
        self.gate_marker.exists()
    }

    pub fn marker_path(&self) -> &PathBuf {
        &self.marker
    }

    pub fn gate_marker_path(&self) -> &PathBuf {
        &self.gate_marker
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

/// Windows has no execute-bit; a `.cmd` file is runnable purely by extension
/// (see `resolve_on_path` in `security::checks`), so there is nothing to
/// mark executable.
#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

/// A `#!/bin/sh` fixture script on Unix is written under its bare name; on
/// Windows the same logical fixture is a `.cmd` file, since that is what
/// `resolve_on_path`'s `PATHEXT` search actually finds.
#[cfg(unix)]
fn script_filename(name: &str) -> String {
    name.to_string()
}

#[cfg(not(unix))]
fn script_filename(name: &str) -> String {
    format!("{name}.cmd")
}

#[cfg(unix)]
pub const GATE_MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_GATE_MARKER\"\n";
#[cfg(not(unix))]
pub const GATE_MARKER_SCRIPT: &str = "@echo off\r\ntype nul > \"%TJ_FIXTURE_GATE_MARKER%\"\r\n";

/// Prepends `dir` onto the current `PATH` using the platform's own list
/// separator (`:` on Unix, `;` on Windows) via `env::join_paths`, rather
/// than a hardcoded `:` that silently breaks PATH parsing on Windows.
pub fn prepend_to_path(dir: &Path) -> std::ffi::OsString {
    let existing = std::env::var_os("PATH").unwrap_or_default();
    let mut dirs = vec![dir.to_path_buf()];
    dirs.extend(std::env::split_paths(&existing));
    std::env::join_paths(dirs).expect("PATH entries join")
}
