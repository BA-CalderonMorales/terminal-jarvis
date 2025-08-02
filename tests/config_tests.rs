use terminal_jarvis::config::Config;

#[test]
fn test_default_config() {
    let config = Config::default();
    
    // Test that default tools are configured
    assert!(config.get_tool_config("claude-code").is_some());
    assert!(config.get_tool_config("gemini-cli").is_some());
    assert!(config.get_tool_config("qwen-code").is_some());
    assert!(config.get_tool_config("opencode").is_some());
    
    // Test that claude-code is enabled by default
    assert!(config.is_tool_enabled("claude-code"));
    
    // Test that opencode is disabled by default
    assert!(!config.is_tool_enabled("opencode"));
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    let toml_string = toml::to_string(&config).expect("Failed to serialize config");
    
    // Should be able to deserialize back
    let deserialized: Config = toml::from_str(&toml_string).expect("Failed to deserialize config");
    
    // Check that a few key values are preserved
    assert_eq!(config.is_tool_enabled("claude-code"), deserialized.is_tool_enabled("claude-code"));
    assert_eq!(config.api.base_url, deserialized.api.base_url);
}
