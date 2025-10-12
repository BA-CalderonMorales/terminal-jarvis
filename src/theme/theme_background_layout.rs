// Theme Background Layout - Background color management and layout
//
// This module handles background color application, full-width lines,
// and content positioning within themed backgrounds.

use crate::theme::theme_definitions::Theme;
use crate::theme::theme_utilities::ThemeUtilities;

/// Background layout utilities for themes
pub struct BackgroundLayoutManager;

impl BackgroundLayoutManager {
    /// Create a complete background line with content
    pub fn background_line_with_content(theme: &Theme, content: &str, width: usize) -> String {
        // All themes now have proper background colors - no empty backgrounds
        // Full background color with content - ensure complete coverage
        // The width parameter is the inner content width (between borders)
        let available_space = width;

        // Calculate visual length (excluding ANSI escape codes)
        let visual_len = ThemeUtilities::visual_length(content);

        // Always ensure we fill the exact available space with background color
        // DO NOT include reset - let the caller handle it
        if visual_len >= available_space {
            // Content is too long - truncate but fill entire width with background
            let stripped_content = ThemeUtilities::strip_ansi_codes(content);
            let truncated: String = stripped_content.chars().take(available_space).collect();

            format!(
                "{}{:<width$}",
                theme.colors.background,
                truncated,
                width = available_space
            )
        } else {
            // Content fits, center it with full background coverage
            let padding_total = available_space - visual_len;
            let left_padding = padding_total / 2;
            let right_padding = padding_total - left_padding;

            format!(
                "{}{}{}{}",
                theme.colors.background,
                " ".repeat(left_padding),
                content,
                " ".repeat(right_padding)
            )
        }
    }

    /// Create a full-width line with background color
    #[allow(dead_code)]
    pub fn background_line(theme: &Theme, width: usize) -> String {
        if theme.colors.background.is_empty() {
            String::new()
        } else {
            format!(
                "{}{}{}",
                theme.colors.background,
                " ".repeat(width),
                theme.colors.reset
            )
        }
    }

    /// Create a background section with multiple lines
    #[allow(dead_code)]
    pub fn background_section(theme: &Theme, lines: &[String], width: usize) -> String {
        let mut result = String::new();

        for line in lines {
            result.push_str(&Self::background_line_with_content(theme, line, width));
            result.push_str(theme.colors.reset);
            result.push('\n');
        }

        result
    }

    /// Apply background color with proper padding for centered content
    #[allow(dead_code)]
    pub fn centered_background_content(theme: &Theme, content: &str, total_width: usize) -> String {
        let visual_len = ThemeUtilities::visual_length(content);

        if visual_len >= total_width {
            // Content too long, just apply background
            format!(
                "{}{}{}",
                theme.colors.background, content, theme.colors.reset
            )
        } else {
            let padding = (total_width - visual_len) / 2;
            format!(
                "{}{}{}{}{}",
                theme.colors.background,
                " ".repeat(padding),
                content,
                " ".repeat(total_width - visual_len - padding),
                theme.colors.reset
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
    fn test_background_line_with_content() {
        let theme = create_test_theme();
        let result = BackgroundLayoutManager::background_line_with_content(&theme, "test", 10);

        assert!(result.contains("\x1b[40m"));
        assert!(result.contains("test"));
        // Should have padding spaces
        assert!(result.len() > "test".len() + "\x1b[40m".len());
    }

    #[test]
    fn test_background_line() {
        let theme = create_test_theme();
        let result = BackgroundLayoutManager::background_line(&theme, 5);

        assert!(result.contains("\x1b[40m"));
        assert!(result.contains("     ")); // 5 spaces
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_background_line_with_empty_background() {
        let mut theme = create_test_theme();
        theme.colors.background = "";

        let result = BackgroundLayoutManager::background_line(&theme, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_content_truncation() {
        let theme = create_test_theme();
        let long_content = "this is a very long content that should be truncated";
        let result =
            BackgroundLayoutManager::background_line_with_content(&theme, long_content, 10);

        assert!(result.contains("\x1b[40m"));
        // Content should be truncated to fit width
        let visual_part = result.replace("\x1b[40m", "");
        assert!(visual_part.len() <= 10);
    }
}
