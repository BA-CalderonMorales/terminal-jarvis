use super::RuntimeInput;
use std::path::PathBuf;

impl RuntimeInput {
    pub fn local(
        stdout_tty: bool,
        stderr_tty: bool,
        color: bool,
        width: usize,
        update_route: impl Into<String>,
    ) -> Self {
        let checksum = std::env::var("TERMINAL_JARVIS_CHECKSUM")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| {
                if crate::distribution::channel() == Some("npm") {
                    "unknown".into()
                } else {
                    "not-applicable".into()
                }
            });
        Self {
            gate: crate::context::gates_root(),
            stdout_tty,
            stderr_tty,
            color,
            width,
            update_route: update_route.into(),
            checksum,
        }
    }
}

impl Default for RuntimeInput {
    fn default() -> Self {
        Self {
            gate: PathBuf::from("gates"),
            stdout_tty: false,
            stderr_tty: false,
            color: false,
            width: 100,
            update_route: "cargo".into(),
            checksum: "not-applicable".into(),
        }
    }
}
