use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use terminal_jarvis::contracts::EnvMode;
use terminal_jarvis::diagnostics::{
    DiagnosticInput, Environment, HarnessInput, PlatformInput, RuntimeInput,
};

pub fn input(
    root: &Path,
    private_home: &Path,
    home: &Path,
    catalog: &Path,
    bin: &Path,
) -> DiagnosticInput {
    let mut environment = Environment::default();
    environment.insert("PATH", std::env::join_paths([bin]).unwrap());
    environment.insert("SHELL", "/bin/bash");
    environment.insert("TERMINAL_JARVIS_DISTRIBUTION", "source");
    environment.insert("TERMINAL_JARVIS_CACHE", root.join("cache"));
    environment.insert("SECRET_API_TOKEN", "hunter2-super-secret");
    environment.insert("EMPTY_TOKEN", "   ");
    DiagnosticInput {
        version: "0.1.13-test".into(),
        executable: Some(bin.join("tj")),
        catalog: catalog.into(),
        home: home.into(),
        config: home.join("session.toml"),
        home_prefix: Some(private_home.into()),
        temp_prefix: Some(root.into()),
        active_harness: Some("fixture".into()),
        harnesses: vec![
            harness("fixture", "fixture"),
            harness("outside", "/opt/acme-sensitive-directory/missing-tool"),
        ],
        platform: PlatformInput {
            os: "linux".into(),
            arch: "x86_64".into(),
            libc: "gnu".into(),
            wsl: "no".into(),
        },
        environment,
        runtime: RuntimeInput::default(),
        now: SystemTime::now(),
        stale_after: Duration::from_secs(3600),
    }
}

fn harness(name: &str, binary: &str) -> HarnessInput {
    HarnessInput {
        name: name.into(),
        binary: binary.into(),
        env_mode: EnvMode::Any,
        env: vec![
            "SECRET_API_TOKEN".into(),
            "EMPTY_TOKEN".into(),
            "MISSING_TOKEN".into(),
        ],
        support: vec![("headless".into(), "expected".into(), true)],
    }
}

pub fn fixture(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("tj-phase02-{label}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }
    path
}

pub fn executable(path: &Path) {
    fs::write(path, "fixture").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

pub fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files = vec![
        root.join("catalog/index.toml"),
        root.join("cache"),
        root.join("home/alice-sensitive/.config/terminal-jarvis/session.toml"),
    ];
    files.sort();
    files
        .into_iter()
        .map(|path| {
            let data = fs::read(&path).unwrap();
            (path, data)
        })
        .collect()
}
