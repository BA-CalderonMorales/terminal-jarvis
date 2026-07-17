mod catalog;
mod gate;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Fixture {
    pub root: PathBuf,
    marker: PathBuf,
    gate_marker: PathBuf,
}

impl Fixture {
    pub fn new(download: &str, yolo: &str, script: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "terminal-jarvis-phase02-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&root);
        let bin = root.join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        let child = bin.join("fixture-child");
        std::fs::write(&child, script).unwrap();
        make_executable(&child);
        let gate_child = bin.join("fixture-gate");
        std::fs::write(&gate_child, "#!/bin/sh\n: > \"$TJ_PHASE02_GATE_MARKER\"\n").unwrap();
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
        let path = format!(
            "{}:{}",
            self.root.join("bin").display(),
            std::env::var("PATH").unwrap_or_default()
        );
        Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
            .args(args)
            .env("PATH", path)
            .env("TERMINAL_JARVIS_CATALOG", self.root.join("catalog"))
            .env("TERMINAL_JARVIS_GATE", "acceptance")
            .env("TERMINAL_JARVIS_GATES", self.root.join("gates"))
            .env("TERMINAL_JARVIS_HOME", self.root.join("home"))
            .env("TJ_PHASE02_MARKER", &self.marker)
            .env("TJ_PHASE02_GATE_MARKER", &self.gate_marker)
            .output()
            .expect("terminal-jarvis runs")
    }

    pub fn spawned(&self) -> bool {
        self.marker.exists()
    }

    pub fn gate_spawned(&self) -> bool {
        self.gate_marker.exists()
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
