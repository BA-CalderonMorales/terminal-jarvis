use super::*;

#[test]
fn install_success_adopts_and_counts_the_seconds() {
    let ok = Ok((0, String::new()));
    let (adopted, card, persisted) = text(
        "crush",
        "installed",
        true,
        &ok,
        std::time::Duration::from_millis(3_200),
    );
    assert!(adopted && persisted);
    assert_eq!(card, "installed crush · 3.2s · now active");
}

#[test]
fn install_success_without_a_path_is_honest_about_it() {
    let ok = Ok((0, String::new()));
    let (adopted, card, persisted) = text(
        "crush",
        "installed",
        false,
        &ok,
        std::time::Duration::from_millis(3_200),
    );
    assert!(adopted && !persisted);
    assert_eq!(card, "installed crush · 3.2s · binary not on PATH");
}

#[test]
fn update_success_never_adopts() {
    let ok = Ok((0, String::new()));
    let (adopted, card, _) = text(
        "codex",
        "updated",
        true,
        &ok,
        std::time::Duration::from_secs(64),
    );
    assert!(!adopted);
    assert_eq!(card, "updated codex · 1m04s");
}

#[test]
fn human_formats_seconds_and_minutes() {
    assert_eq!(human(std::time::Duration::from_millis(0)), "0.0s");
    assert_eq!(human(std::time::Duration::from_millis(59_900)), "59.9s");
    assert_eq!(human(std::time::Duration::from_secs(64)), "1m04s");
    assert_eq!(human(std::time::Duration::from_secs(754)), "12m34s");
}

#[test]
fn failures_and_cancels_report_cleanly() {
    let broken = Ok((3, String::new()));
    let (adopted, card, _) = text(
        "crush",
        "installed",
        true,
        &broken,
        std::time::Duration::from_millis(500),
    );
    assert!(!adopted);
    assert_eq!(card, "installed crush failed (exit 3) · 0.5s");
    let (_, card, _) = text(
        "crush",
        "installed",
        true,
        &Err("nope".into()),
        std::time::Duration::from_millis(100),
    );
    assert_eq!(card, "installed crush blocked · 0.1s");
}
