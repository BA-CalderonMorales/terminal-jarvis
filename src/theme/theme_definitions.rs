// Theme Definitions - Core theme data structures and types
//
// This module defines the fundamental theme structures including ColorCodes,
// Theme, and ThemeType enums that form the foundation of the theming system.

use std::fmt;

/// Color codes for terminal styling
#[derive(Debug, Clone)]
pub struct ColorCodes {
    pub reset: &'static str,
    pub background: &'static str,
    pub primary_text: &'static str,
    pub secondary_text: &'static str,
    pub accent_text: &'static str,
    pub border: &'static str,
    pub logo: &'static str,
}

/// Theme configuration for Terminal Jarvis interface
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub colors: ColorCodes,
}

/// Available themes for Terminal Jarvis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    TJarvis,
    Classic,
    Matrix,
}

impl fmt::Display for ThemeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeType::TJarvis => write!(f, "tjarvis"),
            ThemeType::Classic => write!(f, "classic"),
            ThemeType::Matrix => write!(f, "matrix"),
        }
    }
}

impl std::str::FromStr for ThemeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tjarvis" | "t.jarvis" | "default" => Ok(ThemeType::TJarvis),
            "classic" | "minimal" => Ok(ThemeType::Classic),
            "matrix" | "terminal" => Ok(ThemeType::Matrix),
            _ => Err(format!("Unknown theme: {s}")),
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_display() {
        let theme = Theme {
            name: "Test Theme",
            colors: ColorCodes {
                reset: "\x1b[0m",
                background: "",
                primary_text: "\x1b[97m",
                secondary_text: "\x1b[96m",
                accent_text: "\x1b[94m",
                border: "\x1b[36m",
                logo: "\x1b[96m",
            },
        };

        assert_eq!(format!("{theme}"), "Test Theme");
    }

    #[test]
    fn test_color_codes_structure() {
        let colors = ColorCodes {
            reset: "\x1b[0m",
            background: "\x1b[40m",
            primary_text: "\x1b[97m",
            secondary_text: "\x1b[96m",
            accent_text: "\x1b[94m",
            border: "\x1b[36m",
            logo: "\x1b[96m",
        };

        assert_eq!(colors.reset, "\x1b[0m");
        assert_eq!(colors.background, "\x1b[40m");
        assert!(!colors.primary_text.is_empty());
    }
}
