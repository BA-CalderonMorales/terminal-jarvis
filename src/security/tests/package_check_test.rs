use super::pkgcheck_harness::*;

#[test]
fn missing_npm_or_trivy_skips_the_check() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let previous = std::env::var_os("PATH");
    std::env::set_var("PATH", "/nonexistent");
    let result = check("fixture-package");
    if let Some(value) = previous {
        std::env::set_var("PATH", value);
    } else {
        std::env::remove_var("PATH");
    }
    assert!(result.is_none());
}

#[test]
fn trivy_missing_does_not_invoke_npm() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let (dir, sentinel) = fake_bin_npm_only();
    let previous = std::env::var_os("PATH");
    std::env::set_var("PATH", &dir);
    let result = check("fixture-package");
    if let Some(value) = previous {
        std::env::set_var("PATH", value);
    } else {
        std::env::remove_var("PATH");
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert!(result.is_none());
    assert!(!sentinel.exists(), "npm ran despite trivy being missing");
    let _ = std::fs::remove_file(&sentinel);
}

#[test]
fn npm_success_without_lockfile_skips_the_check() {
    let result = run_with_path("npm-only", "#!/bin/sh\nexit 0\n");
    assert!(result.is_none());
}

#[test]
fn npm_failure_skips_the_check() {
    let result = run_with_path("both", "#!/bin/sh\nexit 1\n");
    assert!(result.is_none());
}

#[test]
fn lockfile_with_clean_trivy_reports_clean() {
    let result = run_with_path(
        "both",
        "#!/bin/sh\nprintf '{\"name\":\"fixture\"}' > package-lock.json\nexit 0\n",
    );
    assert!(result.is_some());
    assert!(result.unwrap().clean);
}

#[test]
fn lockfile_with_vulnerable_trivy_reports_detail() {
    let result = run_with_bins(
        "both",
        "#!/bin/sh\nprintf '{\"name\":\"fixture\"}' > package-lock.json\nexit 0\n",
        "#!/bin/sh\necho 'CRITICAL: minimist'\nexit 1\n",
    );
    let verdict = result.unwrap();
    assert!(!verdict.clean);
    assert!(verdict.detail.contains("CRITICAL: minimist"));
}
