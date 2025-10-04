// Clean Responsive Display System
//
// Minimal, professional terminal UI without excessive decoration

use crate::theme::Theme;
use unicode_width::UnicodeWidthStr;

/// Display configuration
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub term_width: usize,
    pub content_width: usize,
}

impl DisplayConfig {
    pub fn new() -> Self {
        let (term_width, _term_height) = terminal_size::terminal_size()
            .map(|(w, h)| (w.0 as usize, h.0 as usize))
            .unwrap_or((80, 24));

        // Simple content width: full width minus small margins
        let content_width = if term_width < 60 {
            term_width.saturating_sub(4)
        } else if term_width < 100 {
            term_width.saturating_sub(6)
        } else {
            term_width.saturating_sub(10)
        };

        Self {
            term_width,
            content_width,
        }
    }

    pub fn get_horizontal_padding(&self) -> usize {
        let border_width = self.content_width + 2;
        if self.term_width > border_width {
            (self.term_width - border_width) / 2
        } else {
            0
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Minimal line rendering without borders
pub struct LineRenderer;

impl LineRenderer {
    /// Horizontal separator line
    pub fn separator(theme: &Theme, width: usize, padding: usize) -> String {
        format!(
            "{}{}{}{}",
            " ".repeat(padding),
            theme.colors.border,
            "─".repeat(width),
            theme.colors.reset
        )
    }

    /// Simple centered line without borders
    pub fn simple_line(content: &str, width: usize, padding: usize) -> String {
        // Strip ANSI codes to measure actual display width (not byte count)
        let plain_content = strip_ansi_codes(content);
        let display_width = plain_content.width(); // Unicode-aware width calculation

        // Truncate with ellipsis if content exceeds width
        if display_width > width {
            let truncate_at = width.saturating_sub(3);
            let truncated = truncate_preserving_ansi(content, truncate_at);
            return format!("{}{}...", " ".repeat(padding), truncated);
        }

        let total_padding = width.saturating_sub(display_width);
        let left_pad = total_padding / 2;
        let right_pad = total_padding - left_pad; // Ensure symmetric padding

        format!(
            "{}{}{}",
            " ".repeat(padding + left_pad),
            content,
            " ".repeat(padding + right_pad)
        )
    }
}

/// Main responsive display
pub struct ResponsiveDisplay {
    pub config: DisplayConfig,
}

impl ResponsiveDisplay {
    pub fn new() -> Self {
        Self {
            config: DisplayConfig::new(),
        }
    }

    /// Print horizontal separator
    pub fn print_separator(&self, theme: &Theme) {
        let padding = self.config.get_horizontal_padding();
        println!(
            "{}",
            LineRenderer::separator(theme, self.config.content_width, padding)
        );
    }

    /// Print empty line for spacing
    #[allow(dead_code)]
    pub fn print_empty_line(&self, _theme: &Theme) {
        println!();
    }

    /// Print centered text without borders
    pub fn print_centered_text(&self, _theme: &Theme, text: &str) {
        let padding = self.config.get_horizontal_padding();
        println!(
            "{}",
            LineRenderer::simple_line(text, self.config.content_width, padding)
        );
    }

    /// Legacy compatibility - top border now prints separator
    #[allow(dead_code)]
    pub fn print_top_border(&self, theme: &Theme) {
        self.print_separator(theme);
    }

    /// Legacy compatibility - bottom border now prints separator
    #[allow(dead_code)]
    pub fn print_bottom_border(&self, theme: &Theme) {
        self.print_separator(theme);
    }

    /// Print logo - clean text-based design with responsive sizing
    #[allow(dead_code)]
    pub fn print_logo(&self, theme: &Theme) {
        // Wide terminal (80+ chars): Full branding with spacing
        if self.config.content_width >= 80 {
            let title = format!(
                "{}T E R M I N A L   {}J A R V I S{}",
                theme.colors.logo, theme.colors.accent_text, theme.colors.reset
            );
            self.print_centered_text(theme, &title);
            self.print_centered_text(theme, "");
            let tagline = format!(
                "{}AI Coding Assistant Command Center{}",
                theme.colors.secondary_text, theme.colors.reset
            );
            self.print_centered_text(theme, &tagline);
        } else if self.config.content_width >= 50 {
            // Medium terminal (50-79 chars): Compact branding
            let title = format!(
                "{}TERMINAL {}JARVIS{}",
                theme.colors.logo, theme.colors.accent_text, theme.colors.reset
            );
            self.print_centered_text(theme, &title);
            let tagline = format!(
                "{}AI Coding Assistant{}",
                theme.colors.secondary_text, theme.colors.reset
            );
            self.print_centered_text(theme, &tagline);
        } else if self.config.content_width >= 30 {
            // Narrow terminal (30-49 chars): Minimal branding
            let title = format!("{}TERMINAL JARVIS{}", theme.colors.logo, theme.colors.reset);
            self.print_centered_text(theme, &title);
            let tagline = format!(
                "{}AI Coding Tools{}",
                theme.colors.secondary_text, theme.colors.reset
            );
            self.print_centered_text(theme, &tagline);
        } else {
            // Very narrow terminal (<30 chars): Essential only
            let title = format!("{}T.JARVIS{}", theme.colors.logo, theme.colors.reset);
            self.print_centered_text(theme, &title);
        }
    }
}

impl Default for ResponsiveDisplay {
    fn default() -> Self {
        Self::new()
    }
}

/// Strip ANSI escape codes for accurate length measurement
pub(crate) fn strip_ansi_codes(text: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;

    for ch in text.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Truncate text while preserving ANSI escape codes
pub(crate) fn truncate_preserving_ansi(text: &str, max_visible_len: usize) -> String {
    let mut result = String::new();
    let mut visible_count = 0;
    let mut in_escape = false;

    for ch in text.chars() {
        if ch == '\x1b' {
            in_escape = true;
            result.push(ch);
        } else if in_escape {
            result.push(ch);
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            if visible_count >= max_visible_len {
                break;
            }
            result.push(ch);
            visible_count += 1;
        }
    }

    result
}
