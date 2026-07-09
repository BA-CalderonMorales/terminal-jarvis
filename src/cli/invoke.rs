use super::resolve;
use crate::contracts::{Capability, CommandPlan, Harness};
use crate::runtime;

pub fn invocation(
    invocation: resolve::Invocation,
    harnesses: &[Harness],
) -> Result<(i32, String), String> {
    capability(
        harnesses,
        &invocation.harness,
        invocation.capability,
        &invocation.extra,
    )
}

pub fn capability(
    harnesses: &[Harness],
    harness: &str,
    capability: Capability,
    extra: &[String],
) -> Result<(i32, String), String> {
    let plan = find(harnesses, harness)?
        .plan(capability)
        .ok_or_else(|| format!("{harness} lacks {capability}"))?;
    runtime::run_command(plan, extra)
        .map(|(code, output)| {
            if code == 0 {
                (0, output)
            } else {
                (
                    code,
                    diagnostic(harness, capability, &plan.command, code, &output),
                )
            }
        })
        .map_err(|error| command_error(harness, plan.command.command.as_str(), error))
}

fn diagnostic(
    harness: &str,
    capability: Capability,
    command: &CommandPlan,
    code: i32,
    output: &str,
) -> String {
    let mut body = format!("harness '{harness}' capability '{capability}' failed with exit {code}\n  command: {}\n  stderr: {output}", command.render());
    if output.contains("pipefail") || output.contains("Illegal option") {
        body.push_str("\n  hint: the script uses `set -o pipefail`, which `sh` (dash) does not support; set the harness command to `bash -c ...` in the registry.");
    }
    body
}

fn find<'a>(harnesses: &'a [Harness], name: &str) -> Result<&'a Harness, String> {
    harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))
}

fn command_error(harness: &str, binary: &str, error: std::io::Error) -> String {
    if error.kind() == std::io::ErrorKind::NotFound {
        return format!("{harness} binary '{binary}' was not found on PATH; run `terminal-jarvis install {harness}` or `terminal-jarvis plan {harness} download`");
    }
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{CapabilityPlan, EnvMode, Harness};

    fn fake_harness() -> Vec<Harness> {
        vec![Harness {
            name: "vibe".into(),
            display: "Vibe".into(),
            description: "t".into(),
            binary: "sh".into(),
            env_mode: EnvMode::None,
            env: vec![],
            capabilities: vec![CapabilityPlan {
                capability: Capability::Download,
                summary: "d".into(),
                command: CommandPlan::new("sh".into(), vec!["-c".into(), "exit 3".into()]),
            }],
        }]
    }

    #[test]
    fn failing_command_diagnoses_harness_capability_and_exit() {
        let (code, body) = capability(&fake_harness(), "vibe", Capability::Download, &[]).unwrap();
        assert_eq!(code, 3);
        assert!(body.contains("vibe"), "harness: {body}");
        assert!(body.contains("download"), "capability: {body}");
        assert!(body.contains("exit 3"), "command: {body}");
        assert!(body.contains('3'), "exit code: {body}");
    }
}
