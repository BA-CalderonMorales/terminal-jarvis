// Authentication API Key Management - API key detection and help messages
//
// REFACTORED: Now delegates to auth_preflight.rs for data-driven logic.
// Kept as a thin compatibility layer for existing callers.

/// API key management utilities (delegates to AuthPreflight)
pub struct ApiKeyManager;

impl ApiKeyManager {
    /// Check if required API keys are set for a tool
    pub fn check_api_keys_for_tool(tool: &str) -> bool {
        use super::auth_preflight::AuthPreflight;
        AuthPreflight::check(tool).is_ready
    }

    /// Provide helpful error messages for missing API keys
    pub fn get_api_key_help_message(tool: &str) -> String {
        use super::auth_preflight::AuthPreflight;
        AuthPreflight::get_help_message(tool)
    }

    /// Get all supported API key environment variables for a tool
    #[allow(dead_code)]
    pub fn get_supported_env_vars(tool: &str) -> Vec<String> {
        use super::auth_preflight::AuthPreflight;
        AuthPreflight::get_required_env_vars(tool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_detection_unknown_tool() {
        // Unknown tools are assumed to not need keys (ready by default)
        assert!(ApiKeyManager::check_api_keys_for_tool(
            "nonexistent_tool_xyz"
        ));
    }

    #[test]
    fn test_help_messages() {
        // Known tool should include env var names if TOML is loadable
        let claude_help = ApiKeyManager::get_api_key_help_message("claude");
        assert!(!claude_help.is_empty());

        // Unknown tool returns a generic message
        let unknown_help = ApiKeyManager::get_api_key_help_message("unknown_tool_xyz");
        assert!(unknown_help.contains("may require authentication"));
    }

    #[test]
    fn test_supported_env_vars() {
        // Unknown tool returns empty
        let unknown_vars = ApiKeyManager::get_supported_env_vars("unknown_tool_xyz");
        assert!(unknown_vars.is_empty());
    }
}
