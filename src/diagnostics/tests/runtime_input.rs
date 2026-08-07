use super::*;

fn envs(values: &[(&str, Option<&str>)]) -> RuntimeInput {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let old: Vec<_> = values
        .iter()
        .map(|(k, _)| (*k, std::env::var_os(k)))
        .collect();
    for (k, v) in values {
        match v {
            Some(v) => std::env::set_var(k, v),
            None => std::env::remove_var(k),
        }
    }
    let input = RuntimeInput::local(false, false, false, 100, "cargo");
    for (k, v) in old {
        match v {
            Some(v) => std::env::set_var(k, v),
            None => std::env::remove_var(k),
        }
    }
    input
}

#[test]
fn checksum_reads_the_variable_when_set() {
    assert_eq!(
        envs(&[("TERMINAL_JARVIS_CHECKSUM", Some("abc"))]).checksum,
        "abc"
    );
}

#[test]
fn blank_checksum_marks_npm_unknown_and_plain_not_applicable() {
    assert_eq!(
        envs(&[
            ("TERMINAL_JARVIS_CHECKSUM", Some("")),
            ("TERMINAL_JARVIS_WRAPPER", Some("cli")),
            ("TERMINAL_JARVIS_DISTRIBUTION", None),
        ])
        .checksum,
        "unknown"
    );
    assert_eq!(
        envs(&[
            ("TERMINAL_JARVIS_CHECKSUM", Some("")),
            ("TERMINAL_JARVIS_WRAPPER", None),
            ("TERMINAL_JARVIS_DISTRIBUTION", None),
        ])
        .checksum,
        "not-applicable"
    );
}
