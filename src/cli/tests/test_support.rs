//! Shared fixtures: plan builders plus cross-platform command helpers.
//! POSIX utilities (`sh`, `true`, `false`, `echo`) are absent from a clean
//! Windows PATH, so process fixtures use `cmd` there instead.

use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EvidenceMode, Interaction, SupportState,
};

fn owned(program: &str, args: &[&str]) -> (String, Vec<String>) {
    let args = args.iter().map(|arg| arg.to_string()).collect();
    (program.to_string(), args)
}

/// "succeeds, prints nothing".
pub fn succeed() -> (String, Vec<String>) {
    if cfg!(windows) {
        owned("cmd", &["/c", "exit 0"])
    } else {
        owned("true", &[])
    }
}

/// "fails with code N".
pub fn fail(code: i32) -> (String, Vec<String>) {
    let exit = format!("exit {code}");
    if cfg!(windows) {
        owned("cmd", &["/c", exit.as_str()])
    } else {
        owned("sh", &["-c", exit.as_str()])
    }
}

/// Runs a shell snippet; the caller supplies one per platform. On Unix `sh`
/// gets a `$0`, so the caller's extra args land positionally in `$1`; on
/// Windows the extra args are appended to the `cmd /c` command string.
pub fn shell(posix: &str, windows: &str) -> (String, Vec<String>) {
    if cfg!(windows) {
        owned("cmd", &["/c", windows])
    } else {
        owned("sh", &["-c", posix, "sh"])
    }
}

/// A probe fixture that prints exactly one version line, `v1.2.3`.
pub fn version_probe() -> (String, Vec<String>) {
    shell("printf 'v1.2.3\\n'", "echo v1.2.3")
}

pub fn plan(capability: Capability, command: &str, args: Vec<String>) -> CapabilityPlan {
    CapabilityPlan {
        capability,
        summary: capability.as_str().to_string(),
        command: CommandPlan::new(command.to_string(), args),
        support: SupportState::Unknown,
        evidence: EvidenceMode::Deterministic,
        effect: match capability {
            Capability::Yolo => Effect::Dangerous,
            Capability::Download | Capability::Update | Capability::Ui => Effect::StateChanging,
            _ => Effect::ReadOnly,
        },
        network: matches!(
            capability,
            Capability::Download | Capability::Update | Capability::Ui | Capability::Yolo
        ),
        interaction: if capability == Capability::Ui {
            Interaction::Interactive
        } else {
            Interaction::Noninteractive
        },
        platforms: Vec::new(),
        executable: command.to_string(),
        source: "internal:test-fixture".to_string(),
        verified_at: "2026-07-17T04:59:27Z".to_string(),
        package: None,
    }
}
