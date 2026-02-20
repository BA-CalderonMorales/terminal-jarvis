use terminal_jarvis::installation_arguments::InstallationManager;
use terminal_jarvis::tools::tools_command_mapping::get_cli_command;
use terminal_jarvis::tools::tools_config::get_tool_config_loader;

#[test]
fn test_kimi_tool_loaded() {
    let loader = get_tool_config_loader();
    let tool_names = loader.get_tool_names();

    assert!(
        tool_names.contains(&"kimi".to_string()),
        "Kimi tool should be auto-discovered from config/tools/"
    );
}

#[test]
fn test_kimi_command_mapping() {
    let cli_command = get_cli_command("kimi");
    assert_eq!(cli_command, "kimi");
}

#[test]
fn test_kimi_tool_definition() {
    let loader = get_tool_config_loader();
    let tool_def = loader
        .get_tool_definition("kimi")
        .expect("Kimi tool definition should exist");

    assert_eq!(tool_def.display_name, "Kimi");
    assert_eq!(tool_def.config_key, "kimi");
    assert_eq!(tool_def.cli_command, "kimi");
    assert!(!tool_def.requires_npm, "Kimi should not require npm");
    assert!(!tool_def.requires_sudo, "Kimi should not require sudo");
}

#[test]
fn test_kimi_install_command() {
    let loader = get_tool_config_loader();
    let install_cmd = loader
        .get_install_command("kimi")
        .expect("Kimi install command should exist");

    assert_eq!(install_cmd.command, "curl");
    assert!(
        install_cmd.args.iter().any(|a| a.contains("code.kimi.com/install.sh")),
        "Install command should use official Kimi install script"
    );
}

#[test]
fn test_kimi_auth_configuration() {
    let loader = get_tool_config_loader();
    let auth_info = loader
        .get_auth_info("kimi")
        .expect("Kimi auth info should exist");

    assert!(auth_info.browser_auth, "Kimi should support browser auth");
    assert!(
        auth_info.env_vars.contains(&"KIMI_API_KEY".to_string()),
        "Kimi should support KIMI_API_KEY"
    );
}

#[test]
fn test_kimi_installation_manager_entry() {
    let cmd = InstallationManager::get_install_command("kimi");
    assert!(cmd.is_some(), "Kimi must be retrievable via InstallationManager");
}
