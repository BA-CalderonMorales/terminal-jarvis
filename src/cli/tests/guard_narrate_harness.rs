//! Guard-narrate harness: fixture builders and child-process probe shared
//! by the narrate tests. Imports ride `super::*` from the narrate module.

use super::*;

/// The fixture command that succeeds silently: `true` on Unix, `cmd /c` on
/// Windows (a clean Windows PATH has no POSIX utilities).
pub fn command() -> (String, Vec<String>) {
    crate::cli::logic::test_support::succeed()
}

/// The rendered invocation the loud narration prints, kept in step with
/// [`harnesses`] so the expectation is platform-agnostic.
pub fn rendered() -> String {
    let (command, args) = command();
    crate::cli::logic::test_support::plan(Capability::Download, &command, args)
        .command
        .render()
}

pub fn chain_mode() -> Option<bool> {
    let narration = std::env::var("TJ_CHAIN_NARRATE").ok()?;
    Some(match narration.as_str() {
        "quiet" => false,
        "loud" => true,
        _ => return None,
    })
}

pub fn probe(exe: &std::path::Path, mode: &str) -> String {
    let output = std::process::Command::new(exe)
        .args(["chain_narrate_probe", "--nocapture"])
        .env("TJ_CHAIN_NARRATE", mode)
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stderr).to_string()
}

pub fn harnesses() -> Vec<Harness> {
    let (command, args) = command();
    let mut plan = crate::cli::logic::test_support::plan(Capability::Download, &command, args);
    plan.support = crate::contracts::SupportState::Verified;
    plan.verified_at = "2026-08-06T00:00:00Z".to_string();
    vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "test fixture".into(),
        binary: command,
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![plan],
    }]
}

pub fn write_gate(root: &Path, name: &str, binary: &str, args: &[String]) {
    let directory = root.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    let args = args
        .iter()
        .map(|arg| format!("\"{arg}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let body = "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{binary}\"\nargs = [{args}]\ninstall_hint = \"install\"\n";
    std::fs::write(
        directory.join("index.toml"),
        body.replace("{name}", name)
            .replace("{binary}", binary)
            .replace("{args}", &args),
    )
    .unwrap();
}

pub fn restore(previous: Option<std::ffi::OsString>) {
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_GATES", value),
        None => std::env::remove_var("TERMINAL_JARVIS_GATES"),
    }
}
