//! PackageCheck: pre-install vulnerability verdict for registry packages.
//!
//! A package's risk cannot be read from its tarball (no lockfile inside),
//! so the honest mechanism is: resolve the real dependency tree into a
//! lockfile with `npm install --package-lock-only`, then let trivy scan
//! that lockfile with its native npm analysis. Both tools must be on PATH;
//! otherwise the check is skipped and the caller decides how to warn.

use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Verdict {
    pub clean: bool,
    pub detail: String,
}

pub fn check(package: &str) -> Option<Verdict> {
    if !super::checks::command_on_path("npm") || !super::checks::command_on_path("trivy") {
        return None;
    }
    let dir = scoped_dir()?;
    let spec = format!("{package}@latest");
    let resolve = Command::new("npm")
        .args([
            "install",
            "--package-lock-only",
            "--ignore-scripts",
            "--no-audit",
            "--no-fund",
            &spec,
        ])
        .current_dir(&dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !resolve.status.success() || !dir.join("package-lock.json").is_file() {
        let _ = std::fs::remove_dir_all(&dir);
        return None;
    }
    let scan = Command::new("trivy")
        .args([
            "fs",
            "--scanners",
            "vuln",
            "--severity",
            "HIGH,CRITICAL",
            "--exit-code",
            "1",
            ".",
        ])
        .current_dir(&dir)
        .output()
        .ok()?;
    let detail = [
        String::from_utf8_lossy(&scan.stdout).trim().to_string(),
        String::from_utf8_lossy(&scan.stderr).trim().to_string(),
    ]
    .into_iter()
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>()
    .join("\n");
    let _ = std::fs::remove_dir_all(&dir);
    Some(Verdict {
        clean: scan.status.success(),
        detail,
    })
}

fn scoped_dir() -> Option<PathBuf> {
    let leaf = format!(
        "terminal-jarvis-package-check-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_nanos()
    );
    let dir = std::env::temp_dir().join(leaf);
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

#[cfg(test)]
#[path = "../tests/package_check_harness.rs"]
mod pkgcheck_harness;

#[cfg(test)]
#[path = "../tests/package_check_test.rs"]
mod tests;
