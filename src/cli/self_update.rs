use std::process::Command;

pub fn run() -> Result<(i32, String), String> {
    if wrapper_path().is_some() {
        return npm_update();
    }
    let raw = std::env::var("TERMINAL_JARVIS_DISTRIBUTION").unwrap_or_default();
    match raw.as_str() {
        "github-release" | "github-release-cache" => return npm_update(),
        "source" | "env" => return cargo_update(),
        _ => {}
    }
    let binary = std::env::current_exe().map_err(|e| e.to_string())?;
    let path = binary.to_string_lossy();
    if path.contains("homebrew") || path.contains("Cellar") {
        return brew_update();
    }
    cargo_update()
}

fn wrapper_path() -> Option<std::path::PathBuf> {
    let wrapper = std::env::var("TERMINAL_JARVIS_WRAPPER").ok()?;
    let pkg = std::path::Path::new(&wrapper)
        .parent()
        .and_then(std::path::Path::parent)?
        .join("package.json");
    pkg.exists().then_some(pkg)
}

fn npm_update() -> Result<(i32, String), String> {
    run_cmd("npm", &["install", "-g", "terminal-jarvis@latest"])
}

fn brew_update() -> Result<(i32, String), String> {
    run_cmd("brew", &["upgrade", "terminal-jarvis"])
}

fn cargo_update() -> Result<(i32, String), String> {
    run_cmd("cargo", &["install", "terminal-jarvis"])
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<(i32, String), String> {
    let mut command = Command::new(cmd);
    command.args(args);
    let status = command.status().map_err(|e| {
        format!(
            "failed to run '{}': {}; install {} or update manually",
            cmd, e, cmd
        )
    })?;
    let code = status.code().unwrap_or(1);
    if code == 0 {
        Ok((0, format!("terminal-jarvis updated via {cmd}\n")))
    } else {
        Err(format!("'{} {}' exited with {code}", cmd, args.join(" ")))
    }
}
