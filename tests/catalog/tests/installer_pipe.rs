//! A remote installer must fail closed: `curl URL | bash` reports bash's
//! status, so a failed download looked like a successful install. The rows
//! fetch first and pipe only a script that arrived. These runs swap `curl`
//! and `bash` for fakes on PATH, so nothing reaches the network.
#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::Capability;

const ROWS: [(&str, Capability); 3] = [
    ("amp", Capability::Download),
    ("amp", Capability::Update),
    ("claude", Capability::Download),
];
const SCRIPT: &str = "echo two  spaces *\necho done";

fn fakes(tag: &str, curl: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tj-pipe-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, body) in [("curl", curl), ("bash", "#!/bin/sh\ncat > \"$TJ_SEEN\"\n")] {
        let path = dir.join(name);
        std::fs::write(&path, body).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    dir
}

fn run_row(harness: &str, capability: Capability, bin: &Path) -> (Option<i32>, PathBuf) {
    let harnesses = catalog::load(Path::new("harnesses")).expect("catalog loads");
    let row = harnesses.iter().find(|h| h.name == harness).unwrap();
    let plan = row.plan(capability).expect("row exists");
    assert_eq!(plan.command.command, "sh", "{harness}:{capability}");
    let seen = bin.join("seen");
    let status = Command::new("/bin/sh")
        .args(&plan.command.args)
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .env("TJ_SEEN", &seen)
        .status()
        .expect("sh runs");
    (status.code(), seen)
}

#[test]
fn a_failed_download_fails_the_install_and_never_runs_bash() {
    for (harness, capability) in ROWS {
        let bin = fakes(
            &format!("fail-{harness}-{capability}"),
            "#!/bin/sh\nexit 22\n",
        );
        let (code, seen) = run_row(harness, capability, &bin);
        assert_eq!(code, Some(22), "{harness}:{capability} must fail closed");
        assert!(!seen.exists(), "{harness}:{capability} piped into bash");
    }
}

#[test]
fn a_downloaded_script_reaches_bash_byte_for_byte() {
    for (harness, capability) in ROWS {
        let curl = format!("#!/bin/sh\nprintf '%s\\n' '{SCRIPT}'\n");
        let bin = fakes(&format!("ok-{harness}-{capability}"), &curl);
        let (code, seen) = run_row(harness, capability, &bin);
        assert_eq!(code, Some(0), "{harness}:{capability}");
        let received = std::fs::read_to_string(&seen).expect("bash ran");
        assert_eq!(
            received, SCRIPT,
            "{harness}:{capability} mangled the script"
        );
    }
}
