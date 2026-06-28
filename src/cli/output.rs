use crate::contracts::{Capability, Harness};
use crate::{context::Session, runtime, security};

pub fn help() -> &'static str {
    super::help::text()
}

pub fn list(harnesses: &[Harness]) -> String {
    let mut out = String::new();
    for harness in harnesses {
        out.push_str(&format!("{} - {}\n", harness.name, harness.description));
    }
    out
}

pub fn current(session: Option<Session>) -> String {
    session
        .map(|session| format!("active harness = {}\n", session.active_harness))
        .unwrap_or_else(|| "active harness = none\n".to_string())
}

pub fn show(harness: &Harness) -> String {
    let mut out = format!(
        "{} ({})\n{}\n",
        harness.display, harness.name, harness.description
    );
    out.push_str(&format!("setup: {}\n", harness.setup_hint()));
    out.push_str("agent loop:\n");
    for plan in runtime::planned_steps(harness) {
        out.push_str(&format!("  {}: {}\n", plan.capability, plan.summary));
    }
    out
}

pub fn plan(harness: &Harness, capability: Capability) -> String {
    let plan = harness
        .plan(capability)
        .expect("validated harness capability");
    format!(
        "{}:{}\n{}\ncommand: {}\nenv: {}\n",
        harness.name,
        capability,
        plan.summary,
        plan.command.render(),
        harness.setup_hint()
    )
}

pub fn checks(harnesses: &[Harness]) -> String {
    let mut out = String::new();
    for harness in harnesses {
        let binary = if security::command_on_path(&harness.binary) {
            "found"
        } else {
            "missing"
        };
        let env = security::missing_env(harness);
        let env_status = env_status(harness, &env);
        out.push_str(&format!(
            "{} binary={} env={}\n",
            harness.name, binary, env_status
        ));
    }
    out
}

pub fn is_harness_ready(h: &Harness) -> bool {
    security::command_on_path(&h.binary) && security::missing_env(h).is_empty()
}

pub fn audit(harnesses: &[Harness]) -> String {
    let mut out = checks(harnesses);
    let ready = harnesses.iter().filter(|h| is_harness_ready(h)).count();
    out.push_str(&format!(
        "\naudit summary: {}/{} harnesses ready\n",
        ready,
        harnesses.len()
    ));
    out
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::contracts::EnvMode;

    fn tmpdir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "tjharnesstest_{:x}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn mock_binary_on_path(tmpdir: &Path) -> String {
        let bin = tmpdir.join("mock-harness");
        std::fs::write(&bin, "#!/bin/sh\necho ok").unwrap();
        std::fs::set_permissions(
            &bin,
            std::os::unix::fs::PermissionsExt::from_mode(0o755),
        )
        .unwrap();
        let old = std::env::var("PATH").unwrap_or_default();
        std::env::set_var("PATH", format!("{}:{}", tmpdir.display(), &old));
        old
    }

    #[test]
    fn is_harness_ready_false_when_binary_missing() {
        let h = Harness {
            name: "x".into(),
            display: "X".into(),
            description: "".into(),
            binary: "does-not-exist-hopefully".into(),
            env_mode: EnvMode::None,
            env: vec![],
            capabilities: vec![],
        };
        assert!(!is_harness_ready(&h));
    }

    #[test]
    fn is_harness_ready_false_when_env_var_missing() {
        let dir = tmpdir();
        let _old = mock_binary_on_path(&dir);

        let h = Harness {
            name: "x".into(),
            display: "X".into(),
            description: "".into(),
            binary: "mock-harness".into(),
            env_mode: EnvMode::All,
            env: vec!["SOME_MISSING_VAR".into()],
            capabilities: vec![],
        };
        assert!(!is_harness_ready(&h));
    }

    #[test]
    fn is_harness_ready_true_when_binary_on_path_and_no_env_required() {
        let dir = tmpdir();
        let _old = mock_binary_on_path(&dir);

        let h = Harness {
            name: "x".into(),
            display: "X".into(),
            description: "".into(),
            binary: "mock-harness".into(),
            env_mode: EnvMode::None,
            env: vec![],
            capabilities: vec![],
        };
        assert!(is_harness_ready(&h));
    }

    #[test]
    fn is_harness_ready_true_when_binary_on_path_and_env_var_set() {
        let dir = tmpdir();
        let _old = mock_binary_on_path(&dir);

        std::env::set_var("TJHARNESS_TEST_VAR", "1");

        let h = Harness {
            name: "x".into(),
            display: "X".into(),
            description: "".into(),
            binary: "mock-harness".into(),
            env_mode: EnvMode::All,
            env: vec!["TJHARNESS_TEST_VAR".into()],
            capabilities: vec![],
        };
        assert!(is_harness_ready(&h));

        std::env::remove_var("TJHARNESS_TEST_VAR");
    }
}

fn env_status(harness: &Harness, missing: &[String]) -> String {
    if missing.is_empty() {
        return "ready".to_string();
    }
    match harness.env_mode {
        crate::contracts::EnvMode::Any => format!("missing one of {}", missing.join(", ")),
        crate::contracts::EnvMode::All => format!("missing {}", missing.join(", ")),
        crate::contracts::EnvMode::None => "ready".to_string(),
    }
}
