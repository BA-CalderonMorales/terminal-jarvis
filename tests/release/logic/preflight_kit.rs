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
    Command::new("sh")
        .arg("scripts/bash/release/index.sh")
        .arg("preflight")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap()
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
