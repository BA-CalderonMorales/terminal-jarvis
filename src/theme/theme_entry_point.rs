// Theme Entry Point - Main Theme interface coordination
//
// This module provides the main Theme interface that coordinates between different
// theme domains for a unified API, maintaining backward compatibility.

// Re-export core structures
pub use crate::theme::theme_definitions::{Theme, ThemeType};

impl Theme {
    /// Get the appropriate theme based on type
    pub fn get(theme_type: ThemeType) -> Self {
        use crate::theme::theme_config::ThemeConfigurator;
        ThemeConfigurator::get_theme(theme_type)
    }

    /// T.JARVIS professional theme with proper contrast (for backward compatibility)
    #[allow(dead_code)]
    pub fn t_jarvis_theme() -> Theme {
        use crate::theme::theme_config::ThemeConfigurator;
        ThemeConfigurator::t_jarvis_theme()
    }

    /// Get the reset code
    pub fn reset(&self) -> &str {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::reset(self)
    }

    /// Format text with primary color
    pub fn primary(&self, text: &str) -> String {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::primary(self, text)
    }

    /// Format text with secondary color
    pub fn secondary(&self, text: &str) -> String {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::secondary(self, text)
    }

    /// Format text with accent color
    pub fn accent(&self, text: &str) -> String {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::accent(self, text)
    }

    /// Format border elements
    #[allow(dead_code)]
    pub fn border(&self, text: &str) -> String {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::border(self, text)
    }

    /// Format text with logo colors without reset (for use with backgrounds)
    #[allow(dead_code)]
    pub fn logo_no_reset(&self, text: &str) -> String {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::logo_no_reset(self, text)
    }

    /// Format text with secondary colors without reset (for use with backgrounds)  
    #[allow(dead_code)]
    pub fn secondary_no_reset(&self, text: &str) -> String {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::secondary_no_reset(self, text)
    }

    /// Apply background color for the entire line
    #[allow(dead_code)]
    pub fn format_line(&self, content: &str) -> String {
        use crate::theme::theme_text_formatting::TextFormatter;
        TextFormatter::format_line(self, content)
    }

    /// Create a complete background line with content
    #[allow(dead_code)]
    pub fn background_line_with_content(&self, content: &str, width: usize) -> String {
        use crate::theme::theme_background_layout::BackgroundLayoutManager;
        BackgroundLayoutManager::background_line_with_content(self, content, width)
    }

    /// Create a full-width line with background color
    #[allow(dead_code)]
    pub fn background_line(&self, width: usize) -> String {
        use crate::theme::theme_background_layout::BackgroundLayoutManager;
        BackgroundLayoutManager::background_line(self, width)
    }

    /// Calculate visual length of string (excluding ANSI escape codes)
    #[allow(dead_code)]
    pub fn visual_length(text: &str) -> usize {
        use crate::theme::theme_utilities::ThemeUtilities;
        ThemeUtilities::visual_length(text)
    }

    /// Strip ANSI escape codes from string for display
    #[allow(dead_code)]
    pub fn strip_ansi_codes(text: &str) -> String {
        use crate::theme::theme_utilities::ThemeUtilities;
        ThemeUtilities::strip_ansi_codes(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_get_method() {
        let theme = Theme::get(ThemeType::TJarvis);
        assert_eq!(theme.name, "T.JARVIS");

        let classic = Theme::get(ThemeType::Classic);
        assert_eq!(classic.name, "Classic");

        let matrix = Theme::get(ThemeType::Matrix);
        assert_eq!(matrix.name, "Matrix");
    }

    #[test]
    fn test_theme_text_formatting() {
        let theme = Theme::get(ThemeType::TJarvis);

        let primary = theme.primary("test");
        assert!(primary.contains("test"));
        assert!(primary.contains("\x1b["));

        let secondary = theme.secondary("test");
        assert!(secondary.contains("test"));

        let accent = theme.accent("test");
        assert!(accent.contains("test"));
    }

    #[test]
    fn test_theme_background_operations() {
        let theme = Theme::get(ThemeType::TJarvis);

        let bg_line = theme.background_line_with_content("test", 10);
        assert!(bg_line.contains("test"));

        let full_bg = theme.background_line(5);
        assert!(full_bg.len() > 5); // Should contain ANSI codes + spaces
    }

    #[test]
    fn test_theme_utilities() {
        assert_eq!(Theme::visual_length("hello"), 5);
        assert_eq!(Theme::visual_length("\x1b[97mhello\x1b[0m"), 5);

        assert_eq!(Theme::strip_ansi_codes("\x1b[97mhello\x1b[0m"), "hello");
        assert_eq!(Theme::strip_ansi_codes("plain text"), "plain text");
    }

    #[test]
    fn test_backward_compatibility() {
        let theme = Theme::t_jarvis_theme();
        assert_eq!(theme.name, "T.JARVIS");

        let reset = theme.reset();
        assert_eq!(reset, "\x1b[0m");
    }
}
