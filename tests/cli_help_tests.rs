use std::process::Command;

fn help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain"])
        .args(args)
        .output()
        .expect("terminal-jarvis runs");
    assert!(output.status.success());
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn help_lists_every_public_command() {
    let body = help(&["--help"]);
    for command in [
        "terminal-jarvis [harness] [args...]",
        "terminal-jarvis run [harness] [capability] [args...]",
        "terminal-jarvis version [--verbose|--info|-v]",
        "terminal-jarvis list",
        "terminal-jarvis check",
        "terminal-jarvis use <harness>",
        "terminal-jarvis current",
        "terminal-jarvis show <harness>",
        "terminal-jarvis plan [harness] <capability>",
        "terminal-jarvis install <harness>",
        "terminal-jarvis update [harness]",
        "terminal-jarvis auth help <harness>",
        "terminal-jarvis config show",
        "terminal-jarvis cache status",
        "terminal-jarvis security [status|audit|harness]",
    ] {
        assert!(body.contains(command), "help missing {command}");
    }
    assert!(body.contains("download update headless version stats models security yolo ui"));
    assert!(body.contains("install <harness> -> run <harness> download"));
}

#[test]
fn empty_invocation_prints_same_help() {
    assert_eq!(help(&[]), help(&["help"]));
}
