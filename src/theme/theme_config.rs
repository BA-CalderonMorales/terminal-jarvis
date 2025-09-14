// Theme Configuration - Predefined theme implementations
//
// This module provides the actual color scheme implementations for different
// themes including T.JARVIS, Classic, and Matrix themes.

use crate::theme::theme_definitions::{ColorCodes, Theme, ThemeType};

/// Theme configuration provider
pub struct ThemeConfigurator;

impl ThemeConfigurator {
    /// Get the appropriate theme based on type
    pub fn get_theme(theme_type: ThemeType) -> Theme {
        match theme_type {
            ThemeType::TJarvis => Self::t_jarvis_theme(),
            ThemeType::Classic => Self::classic_theme(),
            ThemeType::Matrix => Self::matrix_theme(),
        }
    }

    /// T.JARVIS professional theme with proper contrast  
    pub fn t_jarvis_theme() -> Theme {
        Theme {
            name: "T.JARVIS",
            colors: ColorCodes {
                // T.JARVIS blue theme - enhanced professional corporate look with perfect contrast
                background: "\x1b[48;2;0;51;102m", // Deep blue background
                primary_text: "\x1b[1;38;2;255;255;255m", // Bold pure white text for maximum readability
                secondary_text: "\x1b[38;2;200;230;255m", // Enhanced light blue text with more warmth
                accent_text: "\x1b[1;38;2;0;255;255m",    // Bold cyan accent for emphasis
                logo: "\x1b[1;38;2;102;255;255m", // Enhanced bright cyan logo with subtle brightness boost
                border: "\x1b[38;2;0;255;255m",   // Cyan border maintaining consistency
                reset: "\x1b[0m",
            },
        }
    }

    /// Classic minimal theme
    pub fn classic_theme() -> Theme {
        Theme {
            name: "Classic",
            colors: ColorCodes {
                reset: "\x1b[0m",
                background: "\x1b[48;2;32;32;32m", // Dark gray background for proper contrast
                primary_text: "\x1b[96m",          // Bright cyan
                secondary_text: "\x1b[97m",        // Bright white
                accent_text: "\x1b[94m",           // Bright blue
                border: "\x1b[36m",                // Cyan
                logo: "\x1b[96m",                  // Bright cyan
            },
        }
    }

    /// Matrix-style green theme
    pub fn matrix_theme() -> Theme {
        Theme {
            name: "Matrix",
            colors: ColorCodes {
                reset: "\x1b[0m",
                background: "\x1b[40m",                // Black background
                primary_text: "\x1b[38;2;0;255;65m",   // Matrix green
                secondary_text: "\x1b[38;2;0;200;0m",  // Darker green
                accent_text: "\x1b[38;2;255;255;255m", // White
                border: "\x1b[38;2;0;255;65m",         // Matrix green
                logo: "\x1b[38;2;0;255;65m",           // Matrix green
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_jarvis_theme() {
        let theme = ThemeConfigurator::t_jarvis_theme();
        assert_eq!(theme.name, "T.JARVIS");
        assert_eq!(theme.colors.reset, "\x1b[0m");
        assert!(!theme.colors.background.is_empty());
        assert!(!theme.colors.primary_text.is_empty());
    }

    #[test]
    fn test_classic_theme() {
        let theme = ThemeConfigurator::classic_theme();
        assert_eq!(theme.name, "Classic");
        assert_eq!(theme.colors.reset, "\x1b[0m");
    }

    #[test]
    fn test_matrix_theme() {
        let theme = ThemeConfigurator::matrix_theme();
        assert_eq!(theme.name, "Matrix");
        assert!(theme.colors.primary_text.contains("255;65"));
    }

    #[test]
    fn test_get_theme_by_type() {
        let t_jarvis = ThemeConfigurator::get_theme(ThemeType::TJarvis);
        assert_eq!(t_jarvis.name, "T.JARVIS");

        let classic = ThemeConfigurator::get_theme(ThemeType::Classic);
        assert_eq!(classic.name, "Classic");

        let matrix = ThemeConfigurator::get_theme(ThemeType::Matrix);
        assert_eq!(matrix.name, "Matrix");
    }
}
