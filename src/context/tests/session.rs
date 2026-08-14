use super::*;

#[test]
fn save_and_load_round_trip() {
    let home = std::env::temp_dir().join(format!("tj-session-{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);
    assert!(load(&home).unwrap().is_none());
    save(&home, "codex").unwrap();
    assert_eq!(load(&home).unwrap().unwrap().active_harness, "codex");
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn load_reports_strict_errors_and_degrades_to_defaults() {
    let home = std::env::temp_dir().join(format!("tj-session-bad-{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);
    fs::create_dir_all(&home).unwrap();
    fs::write(
        home.join("session.toml"),
        "active_harness = \"a\"\nactive_harness = \"b\"\n",
    )
    .unwrap();
    assert!(load(&home).unwrap().is_none());
    let _ = fs::remove_dir_all(&home);
}
