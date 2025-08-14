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
#[derive(Debug, Clone)]
pub enum ThemeType {
    TJarvis,
    #[allow(dead_code)]
    Classic,
    #[allow(dead_code)]
    Matrix,
}

impl Theme {
    /// Get the appropriate theme based on type
    pub fn get(theme_type: ThemeType) -> Self {
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
    fn classic_theme() -> Self {
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
    fn matrix_theme() -> Self {
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

    /// Apply background color for the entire line
    #[allow(dead_code)]
    pub fn format_line(&self, content: &str) -> String {
        if self.colors.background.is_empty() {
            content.to_string()
        } else {
            format!("{}{}{}", self.colors.background, content, self.colors.reset)
        }
    }

    /// Format text with logo colors without reset (for use with backgrounds)
    pub fn logo_no_reset(&self, text: &str) -> String {
        format!("{}{}", self.colors.logo, text)
    }

    /// Format text with secondary colors without reset (for use with backgrounds)  
    pub fn secondary_no_reset(&self, text: &str) -> String {
        format!("{}{}", self.colors.secondary_text, text)
    }

    /// Create a complete background line with content
    pub fn background_line_with_content(&self, content: &str, width: usize) -> String {
        // All themes now have proper background colors - no empty backgrounds
        // Full background color with content - ensure complete coverage
        // The width parameter is the inner content width (between borders)
        let available_space = width;

        // Calculate visual length (excluding ANSI escape codes)
        let visual_len = Self::visual_length(content);

        // Always ensure we fill the exact available space with background color
        // DO NOT include reset - let the caller handle it
        if visual_len >= available_space {
            // Content is too long - truncate but fill entire width with background
            let stripped_content = Self::strip_ansi_codes(content);
            let truncated: String = stripped_content.chars().take(available_space).collect();

            format!(
                "{}{:<width$}",
                self.colors.background,
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
                self.colors.background,
                " ".repeat(left_padding),
                content,
                " ".repeat(right_padding)
            )
        }
    }

    /// Calculate visual length of string (excluding ANSI escape codes)
    fn visual_length(text: &str) -> usize {
        let mut visual_len = 0;
        let mut in_escape = false;

        for ch in text.chars() {
            if ch == '\x1b' {
                in_escape = true;
            } else if in_escape && (ch == 'm' || ch.is_ascii_alphabetic()) {
                in_escape = false;
            } else if !in_escape {
                visual_len += 1;
            }
        }

        visual_len
    }

    /// Strip ANSI escape codes from string for display
    fn strip_ansi_codes(text: &str) -> String {
        let mut result = String::new();
        let mut in_escape = false;

        for ch in text.chars() {
            if ch == '\x1b' {
                in_escape = true;
            } else if in_escape && (ch == 'm' || ch.is_ascii_alphabetic()) {
                in_escape = false;
            } else if !in_escape {
                result.push(ch);
            }
        }

        result
    }

    /// Get the reset code
    pub fn reset(&self) -> &str {
        self.colors.reset
    }

    /// Format text with primary color
    pub fn primary(&self, text: &str) -> String {
        format!("{}{}{}", self.colors.primary_text, text, self.colors.reset)
    }

    /// Format text with secondary color
    pub fn secondary(&self, text: &str) -> String {
        format!(
            "{}{}{}",
            self.colors.secondary_text, text, self.colors.reset
        )
    }

    /// Format text with accent color
    pub fn accent(&self, text: &str) -> String {
        format!("{}{}{}", self.colors.accent_text, text, self.colors.reset)
    }

    /// Format border elements
    #[allow(dead_code)]
    pub fn border(&self, text: &str) -> String {
        format!("{}{}{}", self.colors.border, text, self.colors.reset)
    }

    /// Create a full-width line with background color
    #[allow(dead_code)]
    pub fn background_line(&self, width: usize) -> String {
        if self.colors.background.is_empty() {
            String::new()
        } else {
            format!(
                "{}{}{}",
                self.colors.background,
                " ".repeat(width),
                self.colors.reset
            )
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
