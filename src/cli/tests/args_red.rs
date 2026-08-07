use super::*;

fn flat(input: &[&str]) -> Vec<String> {
    input.iter().map(|s| s.to_string()).collect()
}

fn red(words: &[&str]) {
    assert!(parse(flat(words)).is_err(), "red for {words:?}");
}

#[test]
fn every_action_variant_rejects_malformed_red() {
    red(&["tj", "list", "x"]);
    red(&["tj", "check", "x"]);
    red(&["tj", "current", "x"]);
    red(&["tj", "version", "x"]);
    red(&["tj", "--info", "x"]);
    red(&["tj", "use"]);
    red(&["tj", "show"]);
    red(&["tj", "plan"]);
    red(&["tj", "plan", "bogus"]);
    red(&["tj", "plan", "x", "bogus"]);
    red(&["tj", "install"]);
    red(&["tj", "self-update", "x"]);
    red(&["tj", "update", "a", "b"]);
    red(&["tj", "auth", "a", "b"]);
    red(&["tj", "auth", "help", "a", "b"]);
    red(&["tj", "config", "bogus"]);
    red(&["tj", "cache", "bogus"]);
    red(&["tj", "security", "a", "b"]);
    red(&["tj", "gate", "bogus"]);
    red(&["tj", "templates", "x"]);
    red(&["tj", "--bogus"]);
    red(&["tj", "run", "--bogus"]);
}
