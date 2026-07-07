use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use terminal_jarvis::context;

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn temp_home() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "terminal-jarvis-context-{}-{}",
        std::process::id(),
        TEMP_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn missing_session_loads_as_none() {
    let home = temp_home();
    assert_eq!(context::load(&home).unwrap(), None);
}

#[test]
fn active_harness_round_trips() {
    let home = temp_home();
    context::save(&home, "codex").unwrap();
    let session = context::load(&home).unwrap().unwrap();
    assert_eq!(session.active_harness, "codex");
}

#[test]
fn default_home_resolves_to_global_config_dir_without_env() {
    let _guard = ENV_LOCK.lock().unwrap();
    let previous = std::env::var_os("TERMINAL_JARVIS_HOME");
    std::env::remove_var("TERMINAL_JARVIS_HOME");
    let home = context::default_home();
    assert!(
        home.is_absolute(),
        "active harness home must be global, not CWD-relative: {home:?}"
    );
    assert!(home.ends_with("terminal-jarvis"));
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_HOME", value);
    }
}

#[test]
fn catalog_root_uses_explicit_env_path() {
    let _guard = ENV_LOCK.lock().unwrap();
    let previous = std::env::var_os("TERMINAL_JARVIS_CATALOG");
    std::env::set_var("TERMINAL_JARVIS_CATALOG", "/tmp/terminal-jarvis-catalog");
    assert_eq!(
        context::catalog_root(),
        PathBuf::from("/tmp/terminal-jarvis-catalog")
    );
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_CATALOG", value);
    } else {
        std::env::remove_var("TERMINAL_JARVIS_CATALOG");
    }
}

#[test]
fn catalog_root_finds_repo_catalog() {
    let _guard = ENV_LOCK.lock().unwrap();
    let previous = std::env::var_os("TERMINAL_JARVIS_CATALOG");
    std::env::remove_var("TERMINAL_JARVIS_CATALOG");
    let root = context::catalog_root();
    assert!(root.is_absolute());
    assert!(root.is_dir());
    assert!(root.ends_with("harnesses"));
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_CATALOG", value);
    }
}
