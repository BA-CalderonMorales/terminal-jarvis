use std::path::Path;
use terminal_jarvis::catalog;
use terminal_jarvis::contracts::{Capability, CommandPlan};
use terminal_jarvis::runtime;

#[test]
fn command_plan_renders_command_and_args() {
    let command = CommandPlan::new(
        "npm".to_string(),
        vec![
            "install".to_string(),
            "-g".to_string(),
            "@openai/codex".to_string(),
        ],
    );
    assert_eq!(command.render(), "npm install -g @openai/codex");
}

#[test]
fn command_plan_quotes_shell_fragments() {
    let command = CommandPlan::new(
        "sh".to_string(),
        vec![
            "-c".to_string(),
            "curl -fsSL https://example.invalid/install.sh | bash".to_string(),
        ],
    );
    assert_eq!(
        command.render(),
        "sh -c 'curl -fsSL https://example.invalid/install.sh | bash'"
    );
}

#[test]
fn agent_loop_uses_contract_order() {
    let harnesses = catalog::load(Path::new("harnesses")).unwrap();
    let codex = harnesses
        .iter()
        .find(|harness| harness.name == "codex")
        .unwrap();
    let steps = runtime::planned_steps(codex);
    assert_eq!(steps.len(), Capability::ALL.len());
    assert_eq!(steps[0].capability, Capability::Download);
    assert_eq!(steps.last().unwrap().capability, Capability::Ui);
}

#[test]
fn next_step_skips_completed_capabilities() {
    let harnesses = catalog::load(Path::new("harnesses")).unwrap();
    let codex = harnesses
        .iter()
        .find(|harness| harness.name == "codex")
        .unwrap();
    let next = runtime::next_step(codex, &[Capability::Download]).unwrap();
    assert_eq!(next.capability, Capability::Update);
}
