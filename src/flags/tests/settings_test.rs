use super::support::{registry, temp, Env};
use crate::flags::{
    enabled, load_choices, parse_choices, resolve_all, save, settings_path, Source,
};
use std::fs;

#[test]
fn the_parser_reads_the_flags_table_and_skips_others() {
    let data = "# note\n[other]\nx = true\n[flags]\na = true\nb = false\n";
    let choices = parse_choices(data).unwrap();
    assert_eq!(choices.len(), 2);
    assert!(choices["a"]);
    assert!(!choices["b"]);
    assert!(parse_choices("").unwrap().is_empty());
}

#[test]
fn malformed_files_are_errors() {
    for bad in [
        "[flags]\na = maybe\n",
        "[flags]\njust words\n",
        "a = true\n",
    ] {
        assert!(parse_choices(bad).is_err(), "{bad}");
    }
}

#[test]
fn a_malformed_file_falls_back_to_no_choices() {
    let home = temp("malformed");
    fs::write(settings_path(&home), "[flags]\na = maybe\n").unwrap();
    assert!(load_choices(&home).is_empty());
}

#[test]
fn save_round_trips_resets_and_keeps_other_choices() {
    let home = temp("save");
    save(&home, "a", Some(true)).unwrap();
    save(&home, "b", Some(false)).unwrap();
    save(&home, "a", None).unwrap();
    let choices = load_choices(&home);
    assert_eq!(choices.keys().collect::<Vec<_>>(), ["b"]);
    let staged = fs::read_dir(&home)
        .unwrap()
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"));
    assert_eq!(staged.count(), 0);
}

#[test]
fn enabled_reads_env_then_file_then_default_over_a_fixture_registry() {
    let root = registry(&[("alpha", false, "labs"), ("beta", true, "stable")]);
    let home = temp("enabled");
    let mut env = Env::new();
    env.remove("TJ_FLAG_ALPHA");
    env.remove("TJ_FLAG_BETA");
    env.set("TERMINAL_JARVIS_FLAGS", &root);
    env.set("TERMINAL_JARVIS_HOME", &home);
    assert!(!enabled("alpha"));
    assert!(enabled("beta"));
    save(&home, "alpha", Some(true)).unwrap();
    assert!(enabled("alpha"));
    env.set("TJ_FLAG_ALPHA", "off");
    assert!(!enabled("alpha"));
    let all = resolve_all(&home).unwrap();
    assert_eq!(all[0].source, Source::Env);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "not declared")]
fn an_undeclared_name_trips_the_debug_assert() {
    let mut env = Env::new();
    env.set("TERMINAL_JARVIS_FLAGS", registry(&[]));
    let _ = enabled("nope");
}
