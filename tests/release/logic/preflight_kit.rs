use std::{fs, path::Path, process::Command};

pub fn make_root(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("terminal-jarvis-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("scripts/bash/release/logic")).unwrap();
    fs::create_dir_all(root.join("npm/terminal-jarvis")).unwrap();
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/bash/release/index.sh"),
        root.join("scripts/bash/release/index.sh"),
    )
    .unwrap();
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("scripts/bash/release/logic/release-preflight.sh"),
        root.join("scripts/bash/release/logic/release-preflight.sh"),
    )
    .unwrap();
    root
}

pub fn write_metadata(root: &Path, cargo: &str, npm: &str, lock: &str) {
    fs::write(root.join("Cargo.toml"), format!("version = \"{cargo}\"\n")).unwrap();
    fs::write(
        root.join("npm/terminal-jarvis/package.json"),
        format!("{{\"version\": \"{npm}\"}}\n"),
    )
    .unwrap();
    fs::write(
        root.join("npm/terminal-jarvis/package-lock.json"),
        format!("{{\"version\": \"{lock}\"}}\n"),
    )
    .unwrap();
    fs::write(root.join("CHANGELOG.md"), format!("## [{cargo}]\n")).unwrap();
}

pub fn run_preflight(root: &Path, args: &[&str]) -> std::process::Output {
    shell()
        .arg("scripts/bash/release/index.sh")
        .arg("preflight")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap()
}

/// The POSIX shell that runs the bash release scripts. Unix has `sh` on PATH;
/// a clean Windows box does not, but Git for Windows -- already required by
/// the tag test -- ships one beside `git`, so derive it from git's exec-path
/// instead of requiring Git-Bash on PATH.
#[cfg(unix)]
fn shell() -> Command {
    Command::new("sh")
}

#[cfg(not(unix))]
fn shell() -> Command {
    Command::new(windows_shell().unwrap_or_else(|| "sh".into()))
}

#[cfg(not(unix))]
fn windows_shell() -> Option<std::path::PathBuf> {
    let output = Command::new("git").arg("--exec-path").output().ok()?;
    let exec = String::from_utf8(output.stdout).ok()?;
    let root = Path::new(exec.trim()).ancestors().nth(3)?;
    ["bin/sh.exe", "usr/bin/sh.exe"]
        .iter()
        .map(|relative| root.join(relative))
        .find(|candidate| candidate.is_file())
}

pub fn git(root: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(root)
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?} failed");
}

pub fn commit(root: &Path, message: &str) {
    git(root, &["add", "."]);
    git(
        root,
        &[
            "-c",
            "user.name=Terminal Jarvis",
            "-c",
            "user.email=tj@example.invalid",
            "commit",
            "-m",
            message,
        ],
    );
}
