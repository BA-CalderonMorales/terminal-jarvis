use crate::gates::{enable, preflight};
use std::path::Path;

const NARRATED: &str = "running security gate 'pass' ...\nsecurity gate 'pass' passed\n";

#[test]
fn narrate_probe() {
    let narration = narrate_mode();
    if narration.is_none() {
        return;
    }
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let root = std::env::temp_dir().join(format!("tj-narrate-{}", std::process::id()));
    let home = root.join("home");
    let catalog = root.join("catalog");
    let _ = std::fs::remove_dir_all(&root);
    write_gate(&catalog, "pass", "true");
    let previous = std::env::var_os("TERMINAL_JARVIS_GATES");
    std::env::set_var("TERMINAL_JARVIS_GATES", &catalog);
    enable(&home, "pass").unwrap();
    preflight(&home, narration.unwrap()).unwrap();
    restore(previous);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn quiet_preflight_is_silent_and_loud_narrates_the_stages() {
    let exe = std::env::current_exe().unwrap();
    assert_eq!(probe(&exe, "quiet"), "");
    assert_eq!(probe(&exe, "loud"), NARRATED);
}

fn narrate_mode() -> Option<bool> {
    match std::env::var("TJ_NARRATE_PROBE") {
        Ok(value) if value == "quiet" => Some(false),
        Ok(value) if value == "loud" => Some(true),
        _ => None,
    }
}

fn probe(exe: &Path, mode: &str) -> String {
    let output = std::process::Command::new(exe)
        .arg("narrate_probe")
        .arg("--nocapture")
        .env("TJ_NARRATE_PROBE", mode)
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn write_gate(root: &Path, name: &str, binary: &str) {
    let directory = root.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    let body = "name = \"{name}\"\ndisplay = \"{name}\"\ndescription = \"test\"\nbinary = \"{binary}\"\nargs = []\ninstall_hint = \"install\"\n";
    std::fs::write(
        directory.join("index.toml"),
        body.replace("{name}", name).replace("{binary}", binary),
    )
    .unwrap();
}

fn restore(previous: Option<std::ffi::OsString>) {
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_GATES", value),
        None => std::env::remove_var("TERMINAL_JARVIS_GATES"),
    }
}
