use std::env::{remove_var, set_var, var_os};
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
    let previous = var_os("TERMINAL_JARVIS_HOME");
    let previous_home = var_os("HOME");
    remove_var("TERMINAL_JARVIS_HOME");
    set_var("HOME", temp_home());
    let home = context::default_home();
    assert!(
        home.is_absolute(),
        "active harness home must be global, not CWD-relative: {home:?}"
    );
    assert!(home.ends_with("terminal-jarvis"));
    if let Some(value) = previous {
        set_var("TERMINAL_JARVIS_HOME", value);
    }
    match previous_home {
        Some(value) => set_var("HOME", value),
        None => remove_var("HOME"),
    }
}

#[test]
fn config_home_skips_empty_xdg() {
    let _guard = ENV_LOCK.lock().unwrap();
    let prev_home = var_os("HOME");
    remove_var("TERMINAL_JARVIS_HOME");
    set_var("XDG_CONFIG_HOME", "");
    set_var("HOME", temp_home());
    let home = context::default_home();
    assert!(home.is_absolute() && home.ends_with(".config/terminal-jarvis"));
    if let Some(v) = prev_home {
        set_var("HOME", v);
    }
}

#[test]
fn catalog_root_uses_explicit_env_path() {
    let _guard = ENV_LOCK.lock().unwrap();
    let previous = var_os("TERMINAL_JARVIS_CATALOG");
    set_var("TERMINAL_JARVIS_CATALOG", "/tmp/terminal-jarvis-catalog");
    assert_eq!(
        context::catalog_root(),
        PathBuf::from("/tmp/terminal-jarvis-catalog")
    );
    match previous {
        Some(value) => set_var("TERMINAL_JARVIS_CATALOG", value),
        None => remove_var("TERMINAL_JARVIS_CATALOG"),
    }
}

#[test]
fn catalog_root_uses_bundled_data_without_a_workspace_override() {
    let _guard = ENV_LOCK.lock().unwrap();
    let previous = var_os("TERMINAL_JARVIS_CATALOG");
    remove_var("TERMINAL_JARVIS_CATALOG");
    let root = context::catalog_root();
    assert!(
        root == PathBuf::from(context::EMBEDDED_CATALOG)
            || (root.is_absolute() && root.ends_with("harnesses"))
    );
    if let Some(value) = previous {
        set_var("TERMINAL_JARVIS_CATALOG", value);
    }
}
