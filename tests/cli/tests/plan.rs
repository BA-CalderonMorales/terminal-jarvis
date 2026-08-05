use std::process::Command;

static BINARY: &str = env!("CARGO_BIN_EXE_terminal-jarvis");

fn run(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(BINARY)
        .args(["--plain"])
        .args(args)
        .env("TERMINAL_JARVIS_HOME", temp_home())
        .output()
        .expect("terminal-jarvis runs");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(1);
    (code, stdout, stderr)
}

fn temp_home() -> String {
    std::env::temp_dir()
        .join(format!("tj-redgreen-{}", std::process::id()))
        .to_string_lossy()
        .to_string()
}

#[test]
fn plan_unknown_capability_is_red() {
    for harness in ["aider", "opencode", "codex", "claude", "code"] {
        let (code, _, stderr) = run(&["plan", harness, "nonexistent-capability"]);
        assert_eq!(code, 2, "plan {harness} bogus should exit 2");
        assert!(
            stderr.contains("unknown capability"),
            "plan {harness} bogus should say capability is unknown"
        );
    }
}

#[test]
fn plan_without_active_harness_is_red() {
    let (code, _, stderr) = run(&["plan", "headless"]);
    assert_eq!(code, 3, "plan without active harness should exit 3");
    assert!(
        stderr.contains("no active harness"),
        "plan without active harness should say no active harness"
    );
    assert!(
        stderr.contains("terminal-jarvis use"),
        "plan without active harness should suggest use"
    );
}

#[test]
fn red_green_matrix_covers_all_action_variants() {
    let green_cases: Vec<(&[&str], i32)> = vec![
        (&["help"], 0),
        (&["list"], 0),
        (&["version"], 0),
        (&["current"], 0),
        (&["show", "aider"], 0),
        (&["plan", "aider", "download"], 0),
    ];
    for (args, expected_code) in green_cases {
        let (code, _, _) = run(args);
        assert_eq!(code, expected_code, "green for {args:?}");
    }

    let red_cases: Vec<(&[&str], i32)> = vec![
        (&["--bogus"], 2),
        (&["use"], 2),
        (&["install", "nonexistent"], 4),
        (&["install", "claude"], 4),
        (&["plan", "aider", "bogus"], 2),
        (&["plan", "headless"], 3),
    ];
    for (args, expected_code) in red_cases {
        let (code, _, _) = run(args);
        assert_eq!(code, expected_code, "red for {args:?}");
    }
}
