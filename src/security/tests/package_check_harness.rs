use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);

pub fn fake_bin_pair(npm: &str, trivy: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "tj-pkgcheck-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("npm"), npm).unwrap();
    std::fs::write(dir.join("trivy"), trivy).unwrap();
    for name in ["npm", "trivy"] {
        make_executable(&dir.join(name));
    }
    dir
}

pub fn fake_bin_npm_only() -> (std::path::PathBuf, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!(
        "tj-pkgcheck-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let sentinel = std::env::temp_dir().join(format!(
        "tj-sentinel-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let script = format!("#!/bin/sh\n: > {}\nexit 0\n", sentinel.display());
    std::fs::write(dir.join("npm"), script).unwrap();
    make_executable(&dir.join("npm"));
    (dir, sentinel)
}

pub fn run_with_path(name: &str, script: &str) -> Option<Verdict> {
    run_with_bins(name, script, "#!/bin/sh\nexit 0\n")
}

pub fn run_with_bins(name: &str, npm: &str, trivy: &str) -> Option<Verdict> {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let dir = fake_bin_pair(npm, trivy);
    let previous = std::env::var_os("PATH");
    let path = if name == "npm-only" {
        dir.to_string_lossy().into_owned()
    } else {
        let mut dirs = vec![dir.clone()];
        if let Some(value) = previous.clone() {
            dirs.extend(std::env::split_paths(&value));
        }
        std::env::join_paths(dirs)
            .unwrap()
            .to_string_lossy()
            .into_owned()
    };
    std::env::set_var("PATH", &path);
    let result = check("fixture-package");
    if let Some(value) = previous {
        std::env::set_var("PATH", value);
    } else {
        std::env::remove_var("PATH");
    }
    let _ = std::fs::remove_dir_all(&dir);
    result
}

// Re-exported for the tests module that pulls this file in.
pub use super::{check, Verdict};

fn make_executable(path: &std::path::Path) {
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}
