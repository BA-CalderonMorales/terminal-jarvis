use self::harness::{chain_mode, harnesses, probe, restore, write_gate};
use super::*;

const WARNING_TAIL: &str = "uses a custom installer";

#[cfg(test)]
#[path = "guard_narrate_harness.rs"]
mod harness;

#[test]
fn chain_narrate_probe() {
    let Some(narration) = chain_mode() else {
        return;
    };
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let root = std::env::temp_dir().join(format!("tj-chain-{}", std::process::id()));
    let (home, catalog) = (root.join("home"), root.join("catalog"));
    let _ = std::fs::remove_dir_all(&root);
    write_gate(&catalog, "pass", "true");
    let previous_gates = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    crate::gates::enable(&home, "pass").unwrap();
    let options = Options {
        no_input: true,
        confirm: Some("download:vibe".into()),
        narrate: narration,
        ..Default::default()
    };
    let result = capability(&harnesses(), "vibe", Capability::Download, &options, &home);
    restore(previous_gates);
    let _ = std::fs::remove_dir_all(root);
    assert!(result.is_ok(), "{result:?}");
}

#[test]
fn quiet_mode_reports_phases_in_one_line_each_and_loud_narrates() {
    // probe() spawns a child process that inherits PATH at spawn time; guard
    // against other tests concurrently mutating PATH under this same lock
    // (see chain_narrate_probe() above and ENV_LOCK's other callers) --
    // without it this races and can transiently spawn with a PATH that
    // doesn't have "true" on it.
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let exe = std::env::current_exe().unwrap();
    assert_eq!(
        probe(&exe, "quiet"),
        format!(
            "security scan (pass) ...\rsecurity scan (pass): passed \n\
             warning: vibe's installing {WARNING_TAIL} that cannot be pre-scanned; continuing\n\
             installing vibe ...\n"
        )
    );
    assert_eq!(
        probe(&exe, "loud"),
        format!(
            "running security gate 'pass' ...\nsecurity gate 'pass' passed\n\
             warning: vibe's installing {WARNING_TAIL} that cannot be pre-scanned; continuing\n\
             installing vibe: true ...\n"
        )
    );
}
