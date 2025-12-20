// Preference Keys
//
// Well-known preference key constants.
// Centralized to prevent typos and enable refactoring.

/// Well-known preference keys
pub struct PreferenceKeys;

impl PreferenceKeys {
    /// Last tool that was used
    pub const LAST_USED_TOOL: &'static str = "last_used_tool";

    /// Default tool to launch
    pub const DEFAULT_TOOL: &'static str = "default_tool";

    /// Current UI theme
    pub const THEME: &'static str = "theme";

    /// Whether app has been initialized
    pub const INITIALIZED: &'static str = "initialized";

    /// Whether first run wizard is complete
    pub const FIRST_RUN_COMPLETE: &'static str = "first_run_complete";

    /// Preferred installation method (npm, cargo, etc.)
    pub const PREFERRED_INSTALL_METHOD: &'static str = "preferred_install_method";

    /// Whether to auto-update tools
    pub const AUTO_UPDATE_ENABLED: &'static str = "auto_update_enabled";

    /// Terminal width preference
    pub const TERMINAL_WIDTH: &'static str = "terminal_width";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keys_are_unique() {
        let keys = vec![
            PreferenceKeys::LAST_USED_TOOL,
            PreferenceKeys::DEFAULT_TOOL,
            PreferenceKeys::THEME,
            PreferenceKeys::INITIALIZED,
            PreferenceKeys::FIRST_RUN_COMPLETE,
        ];

        // Check no duplicates
        let mut seen = std::collections::HashSet::new();
        for key in keys {
            assert!(seen.insert(key), "Duplicate key: {}", key);
        }
    }
}
