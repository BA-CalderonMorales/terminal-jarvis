//! CliDriver: the acceptance-test fixture — a scratch catalog, gate config,
//! and marker-instrumented child scripts driven through the real binary.

use super::platform::{make_executable, script_filename};
use crate::structs::catalog;
use crate::structs::gate;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

pub use super::platform::{prepend_to_path, GATE_MARKER_SCRIPT};

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
