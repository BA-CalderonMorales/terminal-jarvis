use super::*;

fn action(args: &[&str]) -> Action {
    parse(args.iter().map(|value| value.to_string())).unwrap()
}

#[test]
fn gate_actions_parse() {
    assert_eq!(action(&["tj", "gate"]), Action::Gate(Vec::new()));
    assert_eq!(
        action(&["tj", "gate", "enable", "trivy"]),
        Action::Gate(vec!["enable".to_string(), "trivy".to_string()])
    );
}

#[test]
fn gate_help_routes_to_top_level_help() {
    assert_eq!(action(&["tj", "gate", "--help"]), Action::Help);
}
