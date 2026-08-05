use super::*;

fn action(args: &[&str]) -> Action {
    parse(args.iter().map(|value| value.to_string())).unwrap()
}

#[test]
fn gate_and_experimental_actions_parse() {
    assert_eq!(action(&["tj", "gate"]), Action::Gate(Vec::new()));
    assert_eq!(
        action(&["tj", "gate", "enable", "trivy"]),
        Action::Gate(vec!["enable".to_string(), "trivy".to_string()])
    );
    assert_eq!(
        action(&["tj", "experimental", "dashboard"]),
        Action::Experimental(vec!["dashboard".to_string()])
    );
}

#[test]
fn gate_and_experimental_help_route_to_top_level_help() {
    assert_eq!(action(&["tj", "gate", "--help"]), Action::Help);
    assert_eq!(action(&["tj", "experimental", "-h"]), Action::Help);
}
