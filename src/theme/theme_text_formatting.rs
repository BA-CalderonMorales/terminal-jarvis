// Theme Text Formatting - Text styling and color application
//
// This module provides text formatting functionality including primary, secondary,
// accent colors, and specialized formatting methods for logos and borders.

use crate::theme::theme_definitions::Theme;

/// Text formatting utilities for themes
pub struct TextFormatter;

impl TextFormatter {
    /// Get the reset code
    pub fn reset(theme: &Theme) -> &str {
        theme.colors.reset
    }

    /// Format text with primary color
    pub fn primary(theme: &Theme, text: &str) -> String {
        format!(
            "{}{}{}",
            theme.colors.primary_text, text, theme.colors.reset
        )
    }

    /// Format text with secondary color
    pub fn secondary(theme: &Theme, text: &str) -> String {
        format!(
            "{}{}{}",
            theme.colors.secondary_text, text, theme.colors.reset
        )
    }

    /// Format text with accent color
    pub fn accent(theme: &Theme, text: &str) -> String {
        format!("{}{}{}", theme.colors.accent_text, text, theme.colors.reset)
    }

    /// Format border elements
    #[allow(dead_code)]
    pub fn border(theme: &Theme, text: &str) -> String {
        format!("{}{}{}", theme.colors.border, text, theme.colors.reset)
    }

    /// Format text with logo colors without reset (for use with backgrounds)
    pub fn logo_no_reset(theme: &Theme, text: &str) -> String {
        format!("{}{}", theme.colors.logo, text)
    }

    /// Format text with secondary colors without reset (for use with backgrounds)  
    pub fn secondary_no_reset(theme: &Theme, text: &str) -> String {
        format!("{}{}", theme.colors.secondary_text, text)
    }

    /// Apply background color for the entire line
    #[allow(dead_code)]
    pub fn format_line(theme: &Theme, content: &str) -> String {
        if theme.colors.background.is_empty() {
            content.to_string()
        } else {
            format!(
                "{}{}{}",
                theme.colors.background, content, theme.colors.reset
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::theme_definitions::{ColorCodes, Theme};

    fn create_test_theme() -> Theme {
        Theme {
            name: "Test",
            colors: ColorCodes {
                reset: "\x1b[0m",
                background: "\x1b[40m",
                primary_text: "\x1b[97m",
                secondary_text: "\x1b[96m",
                accent_text: "\x1b[94m",
                border: "\x1b[36m",
                logo: "\x1b[92m",
            },
        }
    }

    #[test]
    fn test_text_formatting() {
        let theme = create_test_theme();

        let primary_text = TextFormatter::primary(&theme, "test");
        assert!(primary_text.contains("\x1b[97m"));
        assert!(primary_text.contains("test"));
        assert!(primary_text.ends_with("\x1b[0m"));

        let secondary_text = TextFormatter::secondary(&theme, "test");
        assert!(secondary_text.contains("\x1b[96m"));
        assert!(secondary_text.contains("test"));

        let accent_text = TextFormatter::accent(&theme, "test");
        assert!(accent_text.contains("\x1b[94m"));
        assert!(accent_text.contains("test"));
    }

    #[test]
    fn test_no_reset_formatting() {
        let theme = create_test_theme();

        let logo_text = TextFormatter::logo_no_reset(&theme, "test");
        assert!(logo_text.contains("\x1b[92m"));
        assert!(logo_text.contains("test"));
        assert!(!logo_text.contains("\x1b[0m"));

        let secondary_text = TextFormatter::secondary_no_reset(&theme, "test");
        assert!(secondary_text.contains("\x1b[96m"));
        assert!(!secondary_text.contains("\x1b[0m"));
    }

    #[test]
    fn test_reset_code() {
        let theme = create_test_theme();
        assert_eq!(TextFormatter::reset(&theme), "\x1b[0m");
    }
}
