// Integration tests for GitHub Copilot CLI tool
use terminal_jarvis::tools::tools_command_mapping::get_cli_command;
use terminal_jarvis::tools::tools_config::get_tool_config_loader;

#[test]
fn test_copilot_tool_loaded() {
    let loader = get_tool_config_loader();
    let tool_names = loader.get_tool_names();

    // Verify copilot is discovered from config/tools/copilot.toml
    assert!(
        tool_names.contains(&"copilot".to_string()),
        "Copilot tool should be auto-discovered from config/tools/"
    );
}

#[test]
fn test_copilot_command_mapping() {
    // Verify copilot maps to correct CLI command
    let cli_command = get_cli_command("copilot");
    assert_eq!(
        cli_command, "copilot",
        "Copilot should map to 'copilot' CLI command"
    );
}

#[test]
fn test_copilot_tool_definition() {
    let loader = get_tool_config_loader();
    let tool_def = loader
        .get_tool_definition("copilot")
        .expect("Copilot tool definition should exist");

    // Verify basic metadata
    assert_eq!(tool_def.display_name, "Copilot");
    assert_eq!(tool_def.config_key, "copilot");
    assert_eq!(tool_def.cli_command, "copilot");
    assert!(tool_def.requires_npm, "Copilot requires npm");
    assert!(!tool_def.requires_sudo, "Copilot should not require sudo");
    assert_eq!(tool_def.status, "stable");
}

#[test]
fn test_copilot_install_command() {
    let loader = get_tool_config_loader();
    let install_cmd = loader
        .get_install_command("copilot")
        .expect("Copilot install command should exist");

    assert_eq!(install_cmd.command, "npm");
    assert!(
        install_cmd.args.contains(&"install".to_string()),
        "Install command should contain 'install'"
    );
    assert!(
        install_cmd.args.contains(&"-g".to_string()),
        "Install command should be global"
    );
    assert!(
        install_cmd.args.contains(&"@github/copilot".to_string()),
        "Install command should specify @github/copilot package"
    );
}

#[test]
fn test_copilot_update_command() {
    let loader = get_tool_config_loader();
    let update_cmd = loader
        .get_update_command("copilot")
        .expect("Copilot update command should exist");

    assert_eq!(update_cmd.command, "npm");
    assert!(
        update_cmd.args.contains(&"update".to_string()),
        "Update command should contain 'update'"
    );
    assert!(
        update_cmd.args.contains(&"-g".to_string()),
        "Update command should be global"
    );
    assert!(
        update_cmd.args.contains(&"@github/copilot".to_string()),
        "Update command should specify @github/copilot package"
    );
}

#[test]
fn test_copilot_auth_configuration() {
    let loader = get_tool_config_loader();
    let auth_info = loader
        .get_auth_info("copilot")
        .expect("Copilot auth info should exist");

    // Verify auth environment variables
    assert!(
        auth_info.env_vars.contains(&"GITHUB_TOKEN".to_string())
            || auth_info.env_vars.contains(&"GH_TOKEN".to_string()),
        "Copilot should support GITHUB_TOKEN or GH_TOKEN"
    );

    assert!(auth_info.browser_auth, "Copilot supports browser auth");
    assert!(
        auth_info.setup_url.contains("github.com"),
        "Setup URL should point to GitHub"
    );
}

#[test]
fn test_copilot_features() {
    let loader = get_tool_config_loader();
    let tool_def = loader
        .get_tool_definition("copilot")
        .expect("Copilot tool definition should exist");

    let features = tool_def
        .features
        .as_ref()
        .expect("Copilot should have features defined");

    assert!(features.supports_files, "Copilot should support files");
    assert!(
        features.supports_streaming,
        "Copilot should support streaming"
    );
    assert!(
        features.supports_conversation,
        "Copilot should support conversation"
    );

    // Verify it supports common languages
    assert!(
        features.supported_languages.contains(&"python".to_string()),
        "Should support Python"
    );
    assert!(
        features.supported_languages.contains(&"javascript".to_string()),
        "Should support JavaScript"
    );
    assert!(
        features.supported_languages.contains(&"rust".to_string()),
        "Should support Rust"
    );
}

#[test]
fn test_eleven_tools_total() {
    let loader = get_tool_config_loader();
    let tool_names = loader.get_tool_names();

    // We should now have 11 tools total (10 original + copilot)
    assert!(
        tool_names.len() >= 11,
        "Should have at least 11 tools loaded, got {}",
        tool_names.len()
    );

    // Verify all expected tools are present
    let expected_tools = vec![
        "claude", "gemini", "qwen", "opencode", "llxprt",
        "codex", "crush", "goose", "amp", "aider", "copilot"
    ];

    for tool in expected_tools {
        assert!(
            tool_names.contains(&tool.to_string()),
            "Expected tool '{}' not found in loaded tools",
            tool
        );
    }
}
