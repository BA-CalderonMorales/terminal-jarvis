//! PackageCheck: pre-install vulnerability verdict for registry packages.
//!
//! A package's risk cannot be read from its tarball (no lockfile inside),
//! so the honest mechanism is: resolve the real dependency tree into a
//! lockfile with `npm install --package-lock-only`, then let trivy scan
//! that lockfile with its native npm analysis. Both tools must be on PATH;
//! otherwise the check is skipped and the caller decides how to warn.

use super::resolve::scoped_dir;
use std::process::{Command, Stdio};

pub struct Verdict {
    pub clean: bool,
    pub detail: String,
}

pub fn check(package: &str) -> Option<Verdict> {
    if !super::path::command_on_path("npm") || !super::path::command_on_path("trivy") {
        return None;
    }
    let dir = scoped_dir()?;
    let spec = format!("{package}@latest");
    let resolve = Command::new(super::resolve::resolved("npm").as_ref())
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
    let scan = Command::new(super::resolve::resolved("trivy").as_ref())
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

#[cfg(test)]
#[path = "../tests/package_check_harness.rs"]
mod pkgcheck_harness;

#[cfg(test)]
#[path = "../tests/package_check_scripts.rs"]
mod pkgcheck_scripts;

#[cfg(test)]
#[path = "../tests/package_check_test.rs"]
mod tests;
