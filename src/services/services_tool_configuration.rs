// Tool Configuration Management - Display name mapping and config key resolution
//
// This module handles the mapping between user-friendly tool display names
// and their corresponding configuration keys in the TOML config file.

use std::collections::HashMap;

/// Manages tool configuration mapping and resolution
pub struct ToolConfigurationManager;

impl ToolConfigurationManager {
    #[allow(dead_code)] // Framework code for future use
    pub fn new() -> Self {
        Self
    }

    /// Get the mapping between display names and configuration keys
    ///
    /// This mapping is crucial for translating user-facing tool names
    /// to the keys used in the configuration file.
    #[allow(dead_code)]
    pub fn get_display_name_to_config_mapping() -> HashMap<&'static str, &'static str> {
        let mut mapping = HashMap::new();
        mapping.insert("claude", "claude-code");
        mapping.insert("gemini", "gemini-cli");
        mapping.insert("qwen", "qwen-code");
        mapping.insert("opencode", "opencode");
        mapping.insert("llxprt", "llxprt-code");
        mapping.insert("codex", "codex");
        mapping.insert("crush", "crush");
        // New tools
        mapping.insert("ollama", "ollama");
        mapping.insert("vibe", "vibe");
        mapping.insert("droid", "droid");
        mapping.insert("forge", "forge");
        mapping.insert("cursor-agent", "cursor-agent");
        mapping.insert("jules", "jules");
        mapping.insert("kilocode", "kilocode");
        mapping.insert("letta", "letta");
        mapping.insert("nanocoder", "nanocoder");
        mapping.insert("pi", "pi");
        mapping.insert("code", "code");
        mapping.insert("eca", "eca");
        mapping
    }

    /// Get the configuration key for a given tool display name
    ///
    /// If the tool is not in our mapping, returns the display name itself
    /// as a fallback (useful for tools that have the same display and config names).
    #[allow(dead_code)]
    pub fn get_config_key_for_tool<'a>(&self, tool_display_name: &'a str) -> &'a str {
        let mapping = Self::get_display_name_to_config_mapping();
        mapping.get(tool_display_name).unwrap_or(&tool_display_name)
    }
}
