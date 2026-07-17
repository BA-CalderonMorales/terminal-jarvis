use super::Environment;
use crate::contracts::EnvMode;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformInput {
    pub os: String,
    pub arch: String,
    pub libc: String,
    pub wsl: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessInput {
    pub name: String,
    pub binary: String,
    pub env_mode: EnvMode,
    pub env: Vec<String>,
    pub support: Vec<(String, String, bool)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeInput {
    pub gate: PathBuf,
    pub stdout_tty: bool,
    pub stderr_tty: bool,
    pub color: bool,
    pub width: usize,
    pub update_route: String,
    pub checksum: String,
}

#[derive(Clone, Debug)]
pub struct DiagnosticInput {
    pub version: String,
    pub executable: Option<PathBuf>,
    pub catalog: PathBuf,
    pub home: PathBuf,
    pub config: PathBuf,
    pub home_prefix: Option<PathBuf>,
    pub temp_prefix: Option<PathBuf>,
    pub active_harness: Option<String>,
    pub harnesses: Vec<HarnessInput>,
    pub platform: PlatformInput,
    pub environment: Environment,
    pub runtime: RuntimeInput,
    pub now: SystemTime,
    pub stale_after: Duration,
}
