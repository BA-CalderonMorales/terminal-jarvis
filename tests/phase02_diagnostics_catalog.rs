use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};
use terminal_jarvis::diagnostics::{
    collect, Code, DiagnosticInput, Environment, HarnessInput, PlatformInput, RuntimeInput,
};

#[test]
fn real_catalog_has_zero_usable_harnesses_before_promotion() {
    let catalog = terminal_jarvis::catalog::load(Path::new("harnesses")).unwrap();
    assert_eq!(catalog.len(), 25);
    let root = std::env::temp_dir().join(format!("tj-phase02-catalog-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    let bin = root.join("bin");
    let home = root.join("home");
    fs::create_dir_all(&bin).unwrap();
    fs::create_dir_all(&home).unwrap();
    fs::write(home.join("marker"), "unchanged").unwrap();
    executable(&bin.join("tj"));
    let mut environment = Environment::default();
    environment.insert("PATH", std::env::join_paths([&bin]).unwrap());
    environment.insert("SHELL", "/bin/sh");
    environment.insert("TERMINAL_JARVIS_DISTRIBUTION", "source");
    for harness in &catalog {
        executable(&bin.join(&harness.binary));
        for name in &harness.env {
            environment.insert(name, "seeded-secret");
        }
    }
    let input = DiagnosticInput {
        version: "test".into(),
        executable: Some(bin.join("tj")),
        catalog: "harnesses".into(),
        home: home.clone(),
        config: home.join("session.toml"),
        home_prefix: Some(home.clone()),
        temp_prefix: Some(root.clone()),
        active_harness: None,
        harnesses: catalog.iter().map(HarnessInput::from).collect(),
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
    };
    let report = collect(&input);
    assert_eq!(report.ready_harnesses, 0);
    assert!(!report.ok && report.exit_code() == 4);
    let unavailable = report
        .records
        .iter()
        .filter(|record| record.key.ends_with(".readiness") && record.code == Code::Unsupported)
        .count();
    assert_eq!(unavailable, 25);
    assert!(!report.json().contains("seeded-secret"));
    fs::remove_dir_all(root).unwrap();
}

fn executable(path: &Path) {
    fs::write(path, "fixture").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}
