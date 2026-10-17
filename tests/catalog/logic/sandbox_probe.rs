use super::Sandbox;
use crate::structs::input;
use std::process::Command;
use terminal_jarvis::contracts::{Capability, Effect, SupportState};

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

    /// Every catalog descriptor gets probed. Verified/expected rows run when
    /// the current platform is claimed; a row narrowed to another platform is
    /// correctly blocked, so the guard's platform gate is witnessed too.
    pub fn verify_guards(
        &self,
        samples: &[(String, Capability, SupportState, Effect, Vec<String>)],
    ) {
        assert_eq!(samples.len(), 225, "every descriptor must be probed");
        let platform = terminal_jarvis::context::platform::id();
        std::thread::scope(|scope| {
            for chunk in samples.chunks(29) {
                scope.spawn(move || {
                    for (harness, capability, state, effect, platforms) in chunk {
                        let output = self.probe(harness, *capability);
                        let supported =
                            matches!(state, SupportState::Verified | SupportState::Expected);
                        let claimed = platforms.is_empty()
                            || platforms.iter().any(|name| Some(name.as_str()) == platform);
                        if supported && !claimed {
                            assert_eq!(
                                output.status.code(),
                                Some(4),
                                "{harness}:{capability} platform row"
                            );
                            assert!(output.stdout.is_empty());
                            let diagnostic = String::from_utf8_lossy(&output.stderr);
                            assert!(
                                diagnostic.contains("does not support platform"),
                                "{harness}:{capability}: {diagnostic}"
                            );
                        } else if supported && effect == &Effect::ReadOnly {
                            assert_eq!(
                                output.status.code(),
                                Some(2),
                                "{harness}:{capability} read-only row"
                            );
                            assert!(output.stdout.is_empty());
                        } else if supported {
                            assert_eq!(
                                output.status.code(),
                                Some(0),
                                "{harness}:{capability} executable row"
                            );
                            assert!(!output.stdout.is_empty());
                        } else {
                            assert_eq!(output.status.code(), Some(4));
                            assert!(output.stdout.is_empty());
                            let diagnostic = String::from_utf8_lossy(&output.stderr);
                            assert!(diagnostic.contains(&format!(" is {};", state.as_str())));
                        }
                    }
                });
            }
        });
    }
}
