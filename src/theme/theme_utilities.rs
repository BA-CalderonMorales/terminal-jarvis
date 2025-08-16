// Theme Utilities - Helper functions for ANSI code handling
//
// This module provides utility functions for working with ANSI escape codes,
// calculating visual string lengths, and text manipulation for themed display.

/// Theme utility functions
pub struct ThemeUtilities;

impl ThemeUtilities {
    /// Calculate visual length of string (excluding ANSI escape codes)
    pub fn visual_length(text: &str) -> usize {
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
    pub fn strip_ansi_codes(text: &str) -> String {
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

    /// Check if a string contains ANSI escape codes
    #[allow(dead_code)]
    pub fn has_ansi_codes(text: &str) -> bool {
        text.contains('\x1b')
    }

    /// Truncate text to specified visual length while preserving ANSI codes
    #[allow(dead_code)]
    pub fn truncate_with_ansi(text: &str, max_visual_length: usize) -> String {
        let mut result = String::new();
        let mut visual_len = 0;
        let mut in_escape = false;

        for ch in text.chars() {
            if ch == '\x1b' {
                in_escape = true;
                result.push(ch);
            } else if in_escape {
                result.push(ch);
                if ch == 'm' || ch.is_ascii_alphabetic() {
                    in_escape = false;
                }
            } else if visual_len < max_visual_length {
                result.push(ch);
                visual_len += 1;
            } else {
                break;
            }
        }

        result
    }

    /// Pad text to specified visual width while preserving ANSI codes
    #[allow(dead_code)]
    pub fn pad_to_width(text: &str, width: usize, align: TextAlign) -> String {
        let visual_len = Self::visual_length(text);

        if visual_len >= width {
            return text.to_string();
        }

        let padding_needed = width - visual_len;

        match align {
            TextAlign::Left => format!("{}{}", text, " ".repeat(padding_needed)),
            TextAlign::Right => format!("{}{}", " ".repeat(padding_needed), text),
            TextAlign::Center => {
                let left_padding = padding_needed / 2;
                let right_padding = padding_needed - left_padding;
                format!(
                    "{}{}{}",
                    " ".repeat(left_padding),
                    text,
                    " ".repeat(right_padding)
                )
            }
        }
    }

    /// Clean and normalize text for consistent display
    #[allow(dead_code)]
    pub fn normalize_text(text: &str) -> String {
        text.trim()
            .replace('\t', "    ") // Convert tabs to spaces
            .replace('\r', "") // Remove carriage returns
            .lines()
            .collect::<Vec<_>>()
            .join(" ") // Join multiple lines with spaces
    }
}

/// Text alignment options for padding
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_length() {
        // Plain text
        assert_eq!(ThemeUtilities::visual_length("hello"), 5);

        // Text with ANSI codes
        assert_eq!(ThemeUtilities::visual_length("\x1b[97mhello\x1b[0m"), 5);

        // Multiple ANSI codes
        assert_eq!(
            ThemeUtilities::visual_length("\x1b[1;97mhello\x1b[0m world"),
            11
        );

        // Empty string
        assert_eq!(ThemeUtilities::visual_length(""), 0);
    }

    #[test]
    fn test_strip_ansi_codes() {
        assert_eq!(ThemeUtilities::strip_ansi_codes("hello"), "hello");
        assert_eq!(
            ThemeUtilities::strip_ansi_codes("\x1b[97mhello\x1b[0m"),
            "hello"
        );
        assert_eq!(
            ThemeUtilities::strip_ansi_codes("\x1b[1;97mhello\x1b[0m world"),
            "hello world"
        );
        assert_eq!(ThemeUtilities::strip_ansi_codes(""), "");
    }

    #[test]
    fn test_has_ansi_codes() {
        assert!(!ThemeUtilities::has_ansi_codes("hello"));
        assert!(ThemeUtilities::has_ansi_codes("\x1b[97mhello"));
        assert!(ThemeUtilities::has_ansi_codes("hello\x1b[0m"));
    }

    #[test]
    fn test_truncate_with_ansi() {
        let text = "\x1b[97mhello world\x1b[0m";
        let truncated = ThemeUtilities::truncate_with_ansi(text, 5);
        assert_eq!(ThemeUtilities::visual_length(&truncated), 5);
        assert!(truncated.contains("\x1b[97m"));
    }

    #[test]
    fn test_pad_to_width() {
        let text = "hello";

        let left_padded = ThemeUtilities::pad_to_width(text, 10, TextAlign::Left);
        assert_eq!(left_padded, "hello     ");

        let right_padded = ThemeUtilities::pad_to_width(text, 10, TextAlign::Right);
        assert_eq!(right_padded, "     hello");

        let center_padded = ThemeUtilities::pad_to_width(text, 10, TextAlign::Center);
        assert_eq!(center_padded, "  hello   ");
    }

    #[test]
    fn test_normalize_text() {
        assert_eq!(
            ThemeUtilities::normalize_text("  hello\tworld  "),
            "hello    world"
        );
        assert_eq!(
            ThemeUtilities::normalize_text("line1\nline2"),
            "line1 line2"
        );
        assert_eq!(
            ThemeUtilities::normalize_text("text\r\nwith\r\nreturns"),
            "text with returns"
        );
    }
}
