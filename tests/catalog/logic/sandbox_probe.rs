use super::Sandbox;
use crate::structs::input;
use std::process::Command;
use terminal_jarvis::contracts::Capability;

impl Sandbox {
    pub(super) fn probe(&self, harness: &str, capability: Capability) -> std::process::Output {
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
            .env("TJ_CATALOG_SPAWN_LOG", &self.spawn_log);
        if let Some(value) = std::env::var_os("SystemRoot") {
            command.env("SystemRoot", value);
        }
        command
            .current_dir(&self.root)
            .output()
            .expect("terminal-jarvis probe runs")
    }
}
