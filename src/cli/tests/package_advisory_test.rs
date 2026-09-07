use super::*;
use crate::cli::logic::test_support;

fn options() -> Options {
    Options::default()
}

fn harness() -> Harness {
    let mut plan = test_support::plan(
        Capability::Download,
        "sh",
        vec!["-c".into(), "exit 0".into()],
    );
    plan.package = Some("fixture-package".into());
    let mut h = Harness {
        name: "fixture".into(),
        display: "Fixture".into(),
        description: String::new(),
        binary: "fixture-child".into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![plan],
    };
    h.capabilities
        .push(test_support::plan(Capability::Headless, "true", vec![]));
    h
}

fn home() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("tj-advisory-{}", std::process::id()))
}

#[test]
fn gate_off_and_package_warns_and_continues() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let previous = std::env::var_os("TERMINAL_JARVIS_GATE");
    std::env::remove_var("TERMINAL_JARVIS_GATE");
    let plan = harness().capabilities[0].clone();
    assert!(check_quiet(&harness(), &plan, &options(), &home(), false, &mut |_| {}).is_ok());
    if let Some(value) = previous {
        std::env::set_var("TERMINAL_JARVIS_GATE", value);
    }
}

#[test]
fn gate_off_and_no_package_warns_and_continues() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("TERMINAL_JARVIS_GATE");
    let mut plan = harness().capabilities[0].clone();
    plan.package = None;
    assert!(check_quiet(&harness(), &plan, &options(), &home(), false, &mut |_| {}).is_ok());
}

#[test]
fn read_only_capabilities_skip_the_advisory() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var("TERMINAL_JARVIS_GATE");
    let plan = harness().capabilities[1].clone();
    assert!(check_quiet(&harness(), &plan, &options(), &home(), false, &mut |_| {}).is_ok());
}

#[test]
fn dry_run_skips_the_advisory() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var("TERMINAL_JARVIS_GATE", "trivy");
    let plan = harness().capabilities[0].clone();
    let mut dry = options();
    dry.dry_run = true;
    assert!(check_quiet(&harness(), &plan, &dry, &home(), false, &mut |_| {}).is_ok());
    std::env::remove_var("TERMINAL_JARVIS_GATE");
}

#[test]
fn gate_on_without_package_warns_custom_installer_and_continues() {
    let _guard = crate::ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var("TERMINAL_JARVIS_GATE", "trivy");
    let mut plan = harness().capabilities[0].clone();
    plan.package = None;
    assert!(check_quiet(&harness(), &plan, &options(), &home(), false, &mut |_| {}).is_ok());
    std::env::remove_var("TERMINAL_JARVIS_GATE");
}
