use super::support::{registry, Env};
use crate::flags::{enabled, is_declared, resolve_all};

#[test]
fn the_embedded_registry_declares_scanner_choice_and_no_unknown_name() {
    let mut env = Env::new();
    env.remove("TERMINAL_JARVIS_FLAGS");
    assert!(is_declared("scanner-choice"));
    assert!(!is_declared("no-such-flag"));
    assert!(!is_declared(""));
}

#[test]
fn is_declared_agrees_with_the_known_set_enabled_reads() {
    let root = registry(&[("alpha", false, "labs"), ("beta", true, "stable")]);
    let mut env = Env::new();
    env.set("TERMINAL_JARVIS_FLAGS", &root);
    let known: Vec<String> = resolve_all(&root)
        .unwrap()
        .into_iter()
        .map(|entry| entry.flag.name)
        .collect();
    assert_eq!(known, ["alpha", "beta"]);
    for name in &known {
        assert!(is_declared(name), "{name} is known but not declared");
        let _ = enabled(name);
    }
    assert!(!is_declared("gamma"));
}
