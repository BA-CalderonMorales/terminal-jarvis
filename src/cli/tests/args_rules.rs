use super::*;

fn flat(input: &[&str]) -> Vec<String> {
    input.iter().map(|s| s.to_string()).collect()
}

#[test]
fn boundary_requires_run_or_direct() {
    assert!(parse_cli(flat(&["tj", "run", "--", "cmd"])).is_ok());
    assert!(parse_cli(flat(&["tj", "opencode", "--", "cmd"])).is_ok());
    assert!(parse_cli(flat(&["tj", "list", "--"])).is_err());
    assert!(parse_cli(flat(&["tj", "list", "--", "x"])).is_err());
    assert!(parse_cli(flat(&["tj", "--"])).is_err());
}

#[test]
fn confirm_rules_deterministic() {
    assert!(parse_cli(flat(&["tj", "run", "--confirm=bad"])).is_err());
    assert!(parse_cli(flat(&["tj", "run", "--confirm=:target"])).is_err());
    assert!(parse_cli(flat(&["tj", "run", "--confirm=op:"])).is_err());
    assert!(parse_cli(flat(&["tj", "run", "--confirm=op:target"])).is_ok());
    assert!(parse_cli(flat(&["tj", "run", "--confirm=a:b", "--confirm=c:d"])).is_err());
}

#[test]
fn json_gate_spawns_child() {
    assert!(parse_cli(flat(&["tj", "gate", "run", "--json"])).is_err());
    assert!(parse_cli(flat(&["tj", "gate", "status", "--json"])).is_ok());
    assert!(parse_cli(flat(&["tj", "gate", "--json"])).is_ok());
    assert!(parse_cli(flat(&["tj", "--update", "--json", "--dry-run"])).is_ok());
    assert!(parse_cli(flat(&["tj", "update", "--json"])).is_ok());
}

#[test]
fn validate_options_branches_deterministic() {
    let errs: &[&[&str]] = &[
        &["tj", "list", "--verbose"],
        &["tj", "list", "--dry-run"],
        &["tj", "run", "--json"],
        &["tj", "opencode", "--json"],
        &["tj", "opencode", "a", "--json"],
        &["tj", "install", "x", "--json"],
        &["tj", "update", "x", "--json"],
        &["tj", "self-update", "--json"],
    ];
    for words in errs {
        assert!(parse_cli(flat(words)).is_err(), "red for {words:?}");
    }
    for words in [
        &["tj", "check", "--verbose"][..],
        &["tj", "run", "--dry-run"][..],
        &["tj", "install", "x", "--json", "--dry-run"][..],
        &["tj", "update", "x", "--json", "--dry-run"][..],
        &["tj", "self-update", "--json", "--dry-run"][..],
        &["tj", "list", "--json"][..],
        &["tj", "security", "--json"][..],
    ] {
        assert!(parse_cli(flat(words)).is_ok(), "green for {words:?}");
    }
}
