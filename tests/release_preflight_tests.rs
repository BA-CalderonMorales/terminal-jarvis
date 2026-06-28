use std::{fs, path::Path, process::Command};

fn make_root(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("terminal-jarvis-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("scripts")).unwrap();
    fs::create_dir_all(root.join("npm/terminal-jarvis")).unwrap();
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/release-preflight.sh"),
        root.join("scripts/release-preflight.sh"),
    )
    .unwrap();
    root
}

fn write_metadata(root: &Path, cargo: &str, npm: &str, lock: &str) {
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

fn run_preflight(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new("sh")
        .arg("scripts/release-preflight.sh")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap()
}

fn git(root: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(root)
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?} failed");
}

fn commit(root: &Path, message: &str) {
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

#[test]
fn metadata_version_mismatch_fails_clearly() {
    let root = make_root("preflight-version-mismatch");
    write_metadata(&root, "0.1.5", "0.1.4", "0.1.5");
    let output = run_preflight(&root, &[]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("npm package version 0.1.4 does not match Cargo 0.1.5"));
}

#[test]
fn matching_metadata_passes_without_tag_context() {
    let root = make_root("preflight-metadata-ok");
    write_metadata(&root, "0.1.5", "0.1.5", "0.1.5");
    let output = run_preflight(&root, &[]);
    assert!(output.status.success());
}

#[test]
fn tag_must_match_expected_main_tip() {
    let root = make_root("preflight-main-mismatch");
    write_metadata(&root, "0.1.5", "0.1.5", "0.1.5");
    git(&root, &["init", "-b", "main"]);
    commit(&root, "release metadata");
    git(&root, &["tag", "v0.1.5"]);
    fs::write(root.join("after-tag.txt"), "new main tip\n").unwrap();
    commit(&root, "advance main");
    git(&root, &["checkout", "--detach", "v0.1.5"]);
    let output = run_preflight(&root, &["--tag", "v0.1.5", "--expected-main-ref", "main"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("v0.1.5 points to") && stderr.contains("but main is"));
}
