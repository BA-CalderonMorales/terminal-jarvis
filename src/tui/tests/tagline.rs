use super::*;

#[test]
fn status_rule_names_the_active_fleet_and_gate() {
    let line = tagline("codex", 1, 3, None);
    assert!(line.contains("codex"));
    assert!(line.contains("1/3 ready"));
    assert!(line.contains("gate"));
    assert!(line.contains("off"));
}

#[test]
fn a_ready_fleet_and_an_enabled_gate_read_as_settled_state() {
    let line = tagline("codex", 3, 3, Some("trivy"));
    assert!(line.contains("all 3 ready"));
    assert!(line.contains("trivy"));
}
