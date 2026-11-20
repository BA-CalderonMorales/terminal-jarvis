use crate::contracts::EnvMode;
use crate::diagnostics::{DiagnosticInput, Environment, HarnessInput, PlatformInput, RuntimeInput};
use std::time::{Duration, SystemTime};

pub(super) fn input(active: Option<&str>, names: &[&str]) -> DiagnosticInput {
    DiagnosticInput {
        version: String::new(),
        executable: None,
        catalog: Default::default(),
        home: Default::default(),
        config: Default::default(),
        home_prefix: None,
        temp_prefix: None,
        active_harness: active.map(str::to_string),
        harnesses: names
            .iter()
            .map(|name| HarnessInput {
                name: (*name).into(),
                binary: String::new(),
                env_mode: EnvMode::None,
                env: Vec::new(),
                support: Vec::new(),
                version: None,
                retirement: None,
            })
            .collect(),
        platform: PlatformInput {
            os: String::new(),
            arch: String::new(),
            libc: String::new(),
            wsl: String::new(),
        },
        environment: Environment::default(),
        runtime: RuntimeInput {
            gate: Default::default(),
            stdout_tty: false,
            stderr_tty: false,
            color: false,
            width: 0,
            update_route: String::new(),
            checksum: String::new(),
            probes: false,
        },
        now: SystemTime::UNIX_EPOCH,
        stale_after: Duration::ZERO,
    }
}
