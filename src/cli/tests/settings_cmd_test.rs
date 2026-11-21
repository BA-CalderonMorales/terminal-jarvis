use super::*;
use crate::cli::logic::style;

fn words(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| part.to_string()).collect()
}

fn fixture(tag: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let root = std::env::temp_dir().join(format!("tj-settings-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let flag = root.join("flags/wild-idea");
    std::fs::create_dir_all(&flag).unwrap();
    let data = "name = \"wild-idea\"\ndefault = false\nstage = \"labs\"\n\
                description = \"A fixture.\"\nsince = \"0.1.26\"\n";
    std::fs::write(flag.join("index.toml"), data).unwrap();
    (root.join("flags"), root.join("home"))
}

#[test]
fn list_set_dry_run_reset_and_unknown_through_the_shared_dispatch() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let (flags, home) = fixture("dispatch");
    let previous = std::env::var_os("TERMINAL_JARVIS_FLAGS");
    std::env::set_var("TERMINAL_JARVIS_FLAGS", &flags);
    let before = style::set(true, true);
    let options = Options::default();
    let preview = Options {
        dry_run: true,
        ..Options::default()
    };

    let listed = run(&[], &options, &home).unwrap().1;
    assert!(listed.contains("flag=wild-idea state=off stage=labs source=default"));
    run(&words(&["set", "wild-idea", "on"]), &preview, &home).unwrap();
    assert!(!flags::settings_path(&home).exists());
    run(&words(&["set", "wild-idea", "on"]), &options, &home).unwrap();
    assert!(run(&[], &options, &home).unwrap().1.contains("state=on"));
    run(&words(&["reset", "wild-idea"]), &options, &home).unwrap();
    assert!(run(&[], &options, &home).unwrap().1.contains("state=off"));
    let unknown = run(&words(&["reset", "nope"]), &options, &home).unwrap_err();
    assert_eq!((unknown.exit_code, unknown.code), (2, "unknown_flag"));
    assert!(unknown.message.contains("valid flags: wild-idea"));

    style::restore(before);
    match previous {
        Some(value) => std::env::set_var("TERMINAL_JARVIS_FLAGS", value),
        None => std::env::remove_var("TERMINAL_JARVIS_FLAGS"),
    }
}

#[test]
fn help_entry_names_the_grammar() {
    let text = crate::cli::logic::help_command::text("settings");
    assert!(text.contains("terminal-jarvis settings [list|set <flag> on|off"));
    assert!(text.contains("optional features"));
}
