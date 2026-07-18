use super::{fake_path, input};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use terminal_jarvis::contracts::{Capability, SupportState};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Sandbox {
    root: PathBuf,
    bin: PathBuf,
    spawn_log: PathBuf,
    commands: BTreeSet<String>,
}

impl Sandbox {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "terminal-jarvis-phase03-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&root);
        let bin = root.join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        std::fs::create_dir_all(root.join("tmp")).unwrap();
        Self {
            spawn_log: root.join("spawned"),
            root,
            bin,
            commands: BTreeSet::new(),
        }
    }

    pub fn add_fake(&mut self, command: &str) {
        if self.commands.insert(command.to_string()) {
            fake_path::write(&self.bin, command);
        }
    }

    pub fn verify_guards(&self, samples: &[(String, Capability, SupportState)]) {
        assert_eq!(samples.len(), 225, "every descriptor must be probed");
        std::thread::scope(|scope| {
            for chunk in samples.chunks(29) {
                let sandbox = self;
                scope.spawn(move || {
                    for (harness, capability, state) in chunk {
                        let output = sandbox.probe(harness, *capability);
                        assert_eq!(output.status.code(), Some(4));
                        assert!(output.stdout.is_empty());
                        let diagnostic = String::from_utf8_lossy(&output.stderr);
                        assert!(diagnostic.contains(&format!(" is {};", state.as_str())));
                    }
                });
            }
        });
    }

    pub fn assert_zero_effects(&self) {
        assert!(!self.spawn_log.exists(), "a catalog command spawned");
        assert!(
            !self.root.join("tj-home").exists(),
            "Terminal Jarvis wrote state"
        );
    }

    fn probe(&self, harness: &str, capability: Capability) -> std::process::Output {
        let mut command = Command::new(input::binary());
        command
            .args(["--plain", "run", harness, capability.as_str(), "--dry-run"])
            .env_clear()
            .env("PATH", &self.bin)
            .env("HOME", self.root.join("home"))
            .env("XDG_CONFIG_HOME", self.root.join("config"))
            .env("TMPDIR", self.root.join("tmp"))
            .env("TMP", self.root.join("tmp"))
            .env("TEMP", self.root.join("tmp"))
            .env("TERM", "dumb")
            .env("NO_COLOR", "1")
            .env("TERMINAL_JARVIS_CATALOG", input::catalog_root())
            .env("TERMINAL_JARVIS_HOME", self.root.join("tj-home"))
            .env("TJ_PHASE03_SPAWN_LOG", &self.spawn_log)
            .current_dir(&self.root);
        if let Some(value) = std::env::var_os("SystemRoot") {
            command.env("SystemRoot", value);
        }
        command.output().expect("terminal-jarvis probe runs")
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
