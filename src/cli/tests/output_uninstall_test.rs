use super::*;
use crate::contracts::CommandPlan;

fn harness() -> Harness {
    Harness {
        name: "probe".into(),
        display: "Probe".into(),
        description: "probe".into(),
        binary: "probe".into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
        sunset: None,
    }
}

fn removal() -> CommandPlan {
    CommandPlan::new(
        "npm".into(),
        vec!["uninstall".into(), "-g".into(), "@scope/probe".into()],
    )
}

#[test]
fn plain_preview_names_the_command_and_the_consent_token() {
    let previous = style::set(true, true);
    let text = uninstall(&harness(), &removal(), "uninstall:probe");
    style::restore(previous);
    assert!(text.contains("uninstall:probe\n"), "{text}");
    assert!(
        text.contains("command: npm uninstall -g @scope/probe"),
        "{text}"
    );
    assert!(text.contains("--confirm=uninstall:probe"), "{text}");
    assert!(!text.lines().next().unwrap().ends_with(' '), "{text}");
}

#[test]
fn rich_preview_is_a_card_a_user_can_read_before_consenting() {
    let previous = style::set(false, true);
    let text = uninstall(&harness(), &removal(), "uninstall:probe");
    style::restore(previous);
    assert!(text.starts_with("Plan: uninstall probe"), "{text}");
    assert!(text.contains("npm uninstall -g @scope/probe"), "{text}");
    assert!(text.contains("changes files on this machine"), "{text}");
    assert!(
        text.contains("--no-input --confirm=uninstall:probe"),
        "{text}"
    );
}

#[test]
fn a_package_manager_removal_says_it_works_offline() {
    let previous = style::set(true, true);
    let text = uninstall(&harness(), &removal(), "uninstall:probe");
    style::restore(previous);
    assert!(text.contains("effect: changes files on this machine; works offline;"));
}

#[test]
fn a_tools_own_uninstaller_makes_no_network_claim() {
    let own = CommandPlan::new("probe".into(), vec!["uninstall".into(), "--force".into()]);
    let previous = style::set(true, true);
    let text = uninstall(&harness(), &own, "uninstall:probe");
    style::restore(previous);
    assert!(text.contains("effect: changes files on this machine; runs unattended"));
    assert!(!text.contains("offline"), "{text}");
}
