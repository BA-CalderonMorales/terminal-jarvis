use super::*;

const WARNING_TAIL: &str = "uses a custom installer";

#[test]
fn chain_narrate_probe() {
    let Some(narration) = chain_mode() else {
        return;
    };
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let root = std::env::temp_dir().join(format!("tj-chain-{}", std::process::id()));
    let (home, catalog) = (root.join("home"), root.join("catalog"));
    let _ = std::fs::remove_dir_all(&root);
    write_gate(&catalog, "pass", "true");
    let previous_gates = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    crate::gates::enable(&home, "pass").unwrap();
    let options = Options {
        no_input: true,
        confirm: Some("download:vibe".into()),
        narrate: narration,
        ..Default::default()
    };
    let result = capability(&harnesses(), "vibe", Capability::Download, &options, &home);
    restore(previous_gates);
    let _ = std::fs::remove_dir_all(root);
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn quiet_mode_reports_phases_in_one_line_each_and_loud_narrates() {
    let exe = std::env::current_exe().unwrap();
    assert_eq!(
        probe(&exe, "quiet"),
        format!(
            "security scan (pass) ...\rsecurity scan (pass): passed \n\
             warning: vibe's installing {WARNING_TAIL} that cannot be pre-scanned; continuing\n\
             installing vibe ...\n"
        )
    );
    assert_eq!(
        probe(&exe, "loud"),
        format!(
            "running security gate 'pass' ...\nsecurity gate 'pass' passed\n\
             warning: vibe's installing {WARNING_TAIL} that cannot be pre-scanned; continuing\n\
             installing vibe: true ...\n"
        )
    );
}

fn chain_mode() -> Option<bool> {
    let narration = std::env::var("TJ_CHAIN_NARRATE").ok()?;
    Some(match narration.as_str() {
        "quiet" => false,
        "loud" => true,
        _ => return None,
    })
}

fn probe(exe: &std::path::Path, mode: &str) -> String {
    let output = std::process::Command::new(exe)
        .args(["chain_narrate_probe", "--nocapture"])
        .env("TJ_CHAIN_NARRATE", mode)
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn harnesses() -> Vec<Harness> {
    let mut plan = crate::cli::logic::test_support::plan(Capability::Download, "true", vec![]);
    plan.support = crate::contracts::SupportState::Verified;
    plan.verified_at = "2026-08-06T00:00:00Z".to_string();
    vec![Harness {
        name: "vibe".into(),
        display: "Vibe".into(),
        description: "test fixture".into(),
        binary: "true".into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![plan],
    }]
}

fn write_gate(root: &Path, name: &str, binary: &str) {
    let directory = root.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    let body = "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{binary}\"\nargs = []\ninstall_hint = \"install\"\n";
    std::fs::write(
        directory.join("index.toml"),
        body.replace("{name}", name).replace("{binary}", binary),
    )
    .unwrap();
}

fn restore(previous: Option<std::ffi::OsString>) {
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_GATES", value),
        None => std::env::remove_var("TERMINAL_JARVIS_GATES"),
    }
}
