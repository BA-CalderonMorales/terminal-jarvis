use super::*;
use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EnvMode, EvidenceMode, Interaction,
    SupportState,
};

fn harness(binary: &str) -> Harness {
    Harness {
        name: "probe".into(),
        display: "Probe".into(),
        description: "probe".into(),
        binary: binary.into(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![],
        sunset: None,
    }
}

fn plan(command: &str, args: &[&str], package: Option<&str>) -> CapabilityPlan {
    CapabilityPlan {
        capability: Capability::Download,
        summary: "install".into(),
        command: CommandPlan::new(command.into(), args.iter().map(|a| a.to_string()).collect()),
        support: SupportState::Expected,
        evidence: EvidenceMode::Deterministic,
        effect: Effect::StateChanging,
        network: true,
        interaction: Interaction::Noninteractive,
        platforms: vec![],
        executable: command.into(),
        source: "internal:test".into(),
        verified_at: "2026-10-29T00:00:00Z".into(),
        package: package.map(String::from),
        uninstall: None,
    }
}

#[test]
fn a_declared_uninstaller_wins_over_any_guess() {
    let mut download = plan("npm", &["install", "-g", "pkg"], Some("pkg"));
    download.uninstall = Some(CommandPlan::new(
        "pkg".into(),
        vec!["uninstall".into(), "--force".into()],
    ));
    let derived = super::uninstall_plan::derive(&download, &harness("pkg")).expect("declared");
    assert_eq!(derived.command, "pkg");
    assert_eq!(derived.args, ["uninstall", "--force"]);
}

#[test]
fn npm_install_inverts_to_a_global_uninstall() {
    let download = plan("npm", &["install", "-g", "latte"], Some("latte"));
    let derived = super::uninstall_plan::derive(&download, &harness("latte")).expect("npm");
    assert_eq!(derived.command, "npm");
    assert_eq!(derived.args, ["uninstall", "-g", "latte"]);
}

#[test]
fn uv_tool_install_inverts_without_a_package_field() {
    let download = plan(
        "uv",
        &[
            "tool",
            "install",
            "--python",
            "python3.12",
            "aider-chat@latest",
        ],
        None,
    );
    let derived = super::uninstall_plan::derive(&download, &harness("aider")).expect("uv");
    assert_eq!(derived.command, "uv");
    assert_eq!(derived.args, ["tool", "uninstall", "aider-chat"]);
}

#[test]
fn cargo_install_inverts_by_binary() {
    let download = plan(
        "cargo",
        &["install", "ripgrep", "--locked"],
        Some("ripgrep"),
    );
    let derived = super::uninstall_plan::derive(&download, &harness("rg")).expect("cargo");
    assert_eq!(derived.command, "cargo");
    assert_eq!(derived.args, ["uninstall", "rg"]);
}

#[test]
fn a_script_install_has_no_honest_inverse() {
    let download = plan(
        "sh",
        &["-c", "curl https://example.com/install | bash"],
        None,
    );
    assert!(super::uninstall_plan::derive(&download, &harness("probe")).is_none());
}
