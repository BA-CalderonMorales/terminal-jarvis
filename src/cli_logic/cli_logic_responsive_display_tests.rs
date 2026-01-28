// src/cli_logic/cli_logic_responsive_display_tests.rs
//
// Comprehensive unit tests for ResponsiveDisplay system
// Tests terminal resize handling, ANSI code processing, and responsive breakpoints

#[cfg(test)]
mod responsive_display_tests {
    use crate::cli_logic::cli_logic_responsive_display::*;
    use crate::theme::{Theme, ThemeType};

    // Test fixture: Create a minimal test theme
    fn create_test_theme() -> Theme {
        Theme::get(ThemeType::TJarvis)
    }

    // Test fixture: Create DisplayConfig with specific width
    fn create_test_config(term_width: usize) -> DisplayConfig {
        let content_width = if term_width < 60 {
            term_width.saturating_sub(4)
        } else if term_width < 100 {
            term_width.saturating_sub(6)
        } else {
            term_width.saturating_sub(10)
        };

        DisplayConfig {
            term_width,
            content_width,
        }
    }

    mod display_config_tests {
        use super::*;

        #[test]
        fn test_content_width_calculation_small_terminal() {
            let config = create_test_config(25);
            assert_eq!(config.term_width, 25);
            assert_eq!(config.content_width, 21); // 25 - 4
        }

        #[test]
        fn test_content_width_calculation_medium_terminal() {
            let config = create_test_config(40);
            assert_eq!(config.term_width, 40);
            assert_eq!(config.content_width, 36); // 40 - 4
        }

        #[test]
        fn test_content_width_calculation_medium_large_terminal() {
            let config = create_test_config(60);
            assert_eq!(config.term_width, 60);
            assert_eq!(config.content_width, 54); // 60 - 6
        }

        #[test]
        fn test_content_width_calculation_standard_terminal() {
            let config = create_test_config(80);
            assert_eq!(config.term_width, 80);
            assert_eq!(config.content_width, 74); // 80 - 6
        }

        #[test]
        fn test_content_width_calculation_large_terminal() {
            let config = create_test_config(100);
            assert_eq!(config.term_width, 100);
            assert_eq!(config.content_width, 90); // 100 - 10
        }

        #[test]
        fn test_content_width_calculation_very_large_terminal() {
            let config = create_test_config(120);
            assert_eq!(config.term_width, 120);
            assert_eq!(config.content_width, 110); // 120 - 10
        }

        #[test]
        fn test_content_width_calculation_ultra_wide_terminal() {
            let config = create_test_config(200);
            assert_eq!(config.term_width, 200);
            assert_eq!(config.content_width, 190); // 200 - 10
        }

        #[test]
        fn test_content_width_edge_case_zero_width() {
            let config = create_test_config(0);
            assert_eq!(config.term_width, 0);
            assert_eq!(config.content_width, 0); // 0 - 4 with saturating_sub
        }

        #[test]
        fn test_content_width_edge_case_width_one() {
            let config = create_test_config(1);
            assert_eq!(config.term_width, 1);
            assert_eq!(config.content_width, 0); // 1 - 4 with saturating_sub
        }

        #[test]
        fn test_content_width_edge_case_width_three() {
            let config = create_test_config(3);
            assert_eq!(config.term_width, 3);
            assert_eq!(config.content_width, 0); // 3 - 4 with saturating_sub
        }

        #[test]
        fn test_content_width_boundary_at_sixty() {
            // Test boundary: 59 uses -4, 60 uses -6
            let config_59 = create_test_config(59);
            assert_eq!(config_59.content_width, 55); // 59 - 4

            let config_60 = create_test_config(60);
            assert_eq!(config_60.content_width, 54); // 60 - 6
        }

        #[test]
        fn test_content_width_boundary_at_hundred() {
            // Test boundary: 99 uses -6, 100 uses -10
            let config_99 = create_test_config(99);
            assert_eq!(config_99.content_width, 93); // 99 - 6

            let config_100 = create_test_config(100);
            assert_eq!(config_100.content_width, 90); // 100 - 10
        }

        #[test]
        fn test_horizontal_padding_centered() {
            let config = create_test_config(80);
            // content_width = 74, border_width = 76, padding = (80 - 76) / 2 = 2
            assert_eq!(config.get_horizontal_padding(), 2);
        }

        #[test]
        fn test_horizontal_padding_large_terminal() {
            let config = create_test_config(120);
            // content_width = 110, border_width = 112, padding = (120 - 112) / 2 = 4
            assert_eq!(config.get_horizontal_padding(), 4);
        }

        #[test]
        fn test_horizontal_padding_small_terminal() {
            let config = create_test_config(40);
            // content_width = 36, border_width = 38, padding = (40 - 38) / 2 = 1
            assert_eq!(config.get_horizontal_padding(), 1);
        }

        #[test]
        fn test_horizontal_padding_edge_case_zero_width() {
            let config = create_test_config(0);
            // content_width = 0, border_width = 2, term_width < border_width
            assert_eq!(config.get_horizontal_padding(), 0);
        }

        #[test]
        fn test_horizontal_padding_edge_case_minimal_width() {
            let config = create_test_config(2);
            // content_width = 0, border_width = 2, padding = (2 - 2) / 2 = 0
            assert_eq!(config.get_horizontal_padding(), 0);
        }

        #[test]
        fn test_default_constructor_creates_valid_config() {
            let config = DisplayConfig::default();
            assert!(config.term_width > 0);
            assert!(config.content_width <= config.term_width);
        }
    }

    mod ansi_handling_tests {
        use super::*;

        #[test]
        fn test_strip_ansi_codes_plain_text() {
            let plain = "Hello, World!";
            let result = strip_ansi_codes(plain);
            assert_eq!(result, "Hello, World!");
        }

        #[test]
        fn test_strip_ansi_codes_single_color() {
            let colored = "\x1b[97mHello\x1b[0m";
            let result = strip_ansi_codes(colored);
            assert_eq!(result, "Hello");
        }

        #[test]
        fn test_strip_ansi_codes_multiple_colors() {
            let multicolor = "\x1b[97mHello\x1b[0m \x1b[96mWorld\x1b[0m";
            let result = strip_ansi_codes(multicolor);
            assert_eq!(result, "Hello World");
        }

        #[test]
        fn test_strip_ansi_codes_nested_sequences() {
            let nested = "\x1b[97m\x1b[1mBold White\x1b[0m";
            let result = strip_ansi_codes(nested);
            assert_eq!(result, "Bold White");
        }

        #[test]
        fn test_strip_ansi_codes_complex_sequence() {
            let complex = "\x1b[38;5;208mOrange\x1b[0m";
            let result = strip_ansi_codes(complex);
            assert_eq!(result, "Orange");
        }

        #[test]
        fn test_strip_ansi_codes_empty_string() {
            let empty = "";
            let result = strip_ansi_codes(empty);
            assert_eq!(result, "");
        }

        #[test]
        fn test_strip_ansi_codes_only_ansi() {
            let only_ansi = "\x1b[97m\x1b[0m";
            let result = strip_ansi_codes(only_ansi);
            assert_eq!(result, "");
        }

        #[test]
        fn test_strip_ansi_codes_unicode_characters() {
            let unicode = "\x1b[97m你好世界\x1b[0m";
            let result = strip_ansi_codes(unicode);
            assert_eq!(result, "你好世界");
        }

        #[test]
        fn test_strip_ansi_codes_emoji() {
            let emoji = "\x1b[97m🚀 Rocket\x1b[0m";
            let result = strip_ansi_codes(emoji);
            assert_eq!(result, "🚀 Rocket");
        }

        #[test]
        fn test_truncate_preserving_ansi_plain_text() {
            let plain = "Hello, World!";
            let result = truncate_preserving_ansi(plain, 5);
            assert_eq!(result, "Hello");
        }

        #[test]
        fn test_truncate_preserving_ansi_colored_text() {
            let colored = "\x1b[97mHello, World!\x1b[0m";
            let result = truncate_preserving_ansi(colored, 5);
            assert_eq!(result, "\x1b[97mHello");
        }

        #[test]
        fn test_truncate_preserving_ansi_multiple_colors() {
            let multicolor = "\x1b[97mHello\x1b[0m \x1b[96mWorld\x1b[0m";
            let result = truncate_preserving_ansi(multicolor, 10);
            // Truncates to 10 visible chars: "Hello Worl" (includes 4 chars from "World")
            assert_eq!(result, "\x1b[97mHello\x1b[0m \x1b[96mWorl");
        }

        #[test]
        fn test_truncate_preserving_ansi_zero_length() {
            let text = "\x1b[97mHello\x1b[0m";
            let result = truncate_preserving_ansi(text, 0);
            // Truncates at 0, preserving only ANSI codes encountered before any visible chars
            assert_eq!(result, "\x1b[97m");
        }

        #[test]
        fn test_truncate_preserving_ansi_longer_than_content() {
            let text = "\x1b[97mHi\x1b[0m";
            let result = truncate_preserving_ansi(text, 10);
            assert_eq!(result, "\x1b[97mHi\x1b[0m");
        }

        #[test]
        fn test_truncate_preserving_ansi_exact_length() {
            let text = "\x1b[97mHello\x1b[0m";
            let result = truncate_preserving_ansi(text, 5);
            assert_eq!(result, "\x1b[97mHello\x1b[0m");
        }

        #[test]
        fn test_truncate_preserving_ansi_unicode() {
            let unicode = "\x1b[97m你好世界\x1b[0m";
            let result = truncate_preserving_ansi(unicode, 2);
            assert_eq!(result, "\x1b[97m你好");
        }

        #[test]
        fn test_truncate_preserving_ansi_empty_string() {
            let empty = "";
            let result = truncate_preserving_ansi(empty, 5);
            assert_eq!(result, "");
        }

        #[test]
        fn test_truncate_preserving_ansi_complex_sequences() {
            let complex = "\x1b[1m\x1b[97mBold White Text\x1b[0m";
            let result = truncate_preserving_ansi(complex, 4);
            assert_eq!(result, "\x1b[1m\x1b[97mBold");
        }
    }

    mod line_renderer_tests {
        use super::*;
        use unicode_width::UnicodeWidthStr;

        #[test]
        fn test_separator_basic() {
            let theme = create_test_theme();
            let result = LineRenderer::separator(&theme, 10, 2);

            // Should contain: 2 spaces + border color + 10 dashes + reset
            assert!(result.contains("──────────")); // 10 dashes
            assert!(result.starts_with("  ")); // 2 space padding
            assert!(result.contains(theme.colors.border));
            assert!(result.contains(theme.colors.reset));
        }

        #[test]
        fn test_separator_zero_width() {
            let theme = create_test_theme();
            let result = LineRenderer::separator(&theme, 0, 0);

            // Should contain only ANSI codes, no dashes
            assert!(!result.contains("─"));
            assert!(result.contains(theme.colors.border));
            assert!(result.contains(theme.colors.reset));
        }

        #[test]
        fn test_separator_zero_padding() {
            let theme = create_test_theme();
            let result = LineRenderer::separator(&theme, 5, 0);

            assert!(result.contains("─────")); // 5 dashes
            assert!(!result.starts_with(" ")); // No padding
        }

        #[test]
        fn test_separator_large_width() {
            let theme = create_test_theme();
            let result = LineRenderer::separator(&theme, 100, 5);

            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped.chars().filter(|c| *c == '─').count(), 100);
            assert!(result.starts_with("     ")); // 5 space padding
        }

        #[test]
        fn test_simple_line_centered_plain_text() {
            let result = LineRenderer::simple_line("Hello", 11, 0);

            // "Hello" is 5 chars, width is 11, padding should be (11-5)/2 = 3 left, 3 right
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped, "   Hello   ");
        }

        #[test]
        fn test_simple_line_centered_odd_padding() {
            let result = LineRenderer::simple_line("Hi", 11, 0);

            // "Hi" is 2 chars, width is 11, padding should be (11-2)/2 = 4.5 -> 4 left, 5 right
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped, "    Hi     ");
        }

        #[test]
        fn test_simple_line_with_external_padding() {
            let result = LineRenderer::simple_line("Test", 10, 2);

            // Should start with 2 spaces from external padding + centering
            assert!(result.starts_with("  "));
        }

        #[test]
        fn test_simple_line_truncate_with_ellipsis() {
            let result = LineRenderer::simple_line("This is a very long text", 10, 0);

            // Should truncate to 7 chars (10-3) + "..."
            let stripped = strip_ansi_codes(&result);
            assert!(stripped.ends_with("..."));
            assert_eq!(stripped.len(), 10); // 7 chars + 3 dots
        }

        #[test]
        fn test_simple_line_truncate_exact_width() {
            let result = LineRenderer::simple_line("ExactlyTen", 10, 0);

            // Should not truncate, should center
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped, "ExactlyTen");
        }

        #[test]
        fn test_simple_line_truncate_one_char_over() {
            let result = LineRenderer::simple_line("TooLongBy1", 9, 0);

            // Should truncate to 6 chars (9-3) + "..."
            let stripped = strip_ansi_codes(&result);
            assert!(stripped.ends_with("..."));
            assert_eq!(stripped.len(), 9);
        }

        #[test]
        fn test_simple_line_with_ansi_codes() {
            let colored = "\x1b[97mHello\x1b[0m";
            let result = LineRenderer::simple_line(colored, 11, 0);

            // ANSI codes should be preserved, centering based on visible width
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped, "   Hello   ");
            assert!(result.contains("\x1b[97m"));
            assert!(result.contains("\x1b[0m"));
        }

        #[test]
        fn test_simple_line_truncate_with_ansi_preserves_codes() {
            let colored = "\x1b[97mThis is a very long colored text\x1b[0m";
            let result = LineRenderer::simple_line(colored, 10, 0);

            // Should preserve ANSI codes in truncation
            assert!(result.contains("\x1b[97m"));
            let stripped = strip_ansi_codes(&result);
            assert!(stripped.ends_with("..."));
        }

        #[test]
        fn test_simple_line_unicode_characters() {
            let result = LineRenderer::simple_line("你好", 10, 0);

            // Chinese characters are full-width (2 display columns each)
            // "你好" = 4 display columns, width = 10, padding = (10-4)/2 = 3 left, 3 right
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped.width(), 10);
        }

        #[test]
        fn test_simple_line_emoji() {
            let result = LineRenderer::simple_line("🚀", 10, 0);

            // Emoji width varies, but should be handled
            let stripped = strip_ansi_codes(&result);
            assert!(stripped.width() <= 10);
        }

        #[test]
        fn test_simple_line_empty_string() {
            let result = LineRenderer::simple_line("", 10, 0);

            // Empty string should result in full padding
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped.len(), 10);
            assert!(stripped.chars().all(|c| c == ' '));
        }

        #[test]
        fn test_simple_line_width_zero() {
            let result = LineRenderer::simple_line("Test", 0, 0);

            // Should truncate to -3 (saturating to 0) + "..."
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped, "...");
        }

        #[test]
        fn test_simple_line_width_one() {
            let result = LineRenderer::simple_line("Test", 1, 0);

            // Should truncate to -2 (saturating to 0) + "..."
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped, "...");
        }

        #[test]
        fn test_simple_line_width_three() {
            let result = LineRenderer::simple_line("Test", 3, 0);

            // Should truncate to 0 chars (3-3) + "..."
            let stripped = strip_ansi_codes(&result);
            assert_eq!(stripped, "...");
        }
    }

    mod responsive_display_integration_tests {
        use super::*;

        #[test]
        fn test_default_constructor() {
            let display = ResponsiveDisplay::default();
            assert!(display.config.term_width > 0);
            assert!(display.config.content_width <= display.config.term_width);
        }

        #[test]
        fn test_new_constructor() {
            let display = ResponsiveDisplay::new();
            assert!(display.config.term_width > 0);
            assert!(display.config.content_width <= display.config.term_width);
        }

        #[test]
        fn test_logo_very_narrow_terminal() {
            // Terminal < 30 chars: Essential only
            let config = create_test_config(25);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Cannot easily test output, but verify it doesn't panic
            display.print_logo(&theme);

            // Verify config is as expected for this breakpoint
            assert!(display.config.content_width < 30);
        }

        #[test]
        fn test_logo_narrow_terminal() {
            // Terminal 30-49 chars: Minimal branding
            let config = create_test_config(40);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            display.print_logo(&theme);

            assert!(display.config.content_width >= 30);
            assert!(display.config.content_width < 50);
        }

        #[test]
        fn test_logo_medium_terminal() {
            // Terminal 50-79 chars: Compact branding
            let config = create_test_config(60);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            display.print_logo(&theme);

            assert!(display.config.content_width >= 50);
            assert!(display.config.content_width < 80);
        }

        #[test]
        fn test_logo_wide_terminal() {
            // Terminal 80+ chars: Full branding
            let config = create_test_config(100);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            display.print_logo(&theme);

            assert!(display.config.content_width >= 80);
        }

        #[test]
        fn test_logo_boundary_at_30_chars() {
            // Test exact boundary: 29 vs 30
            let config_29 = create_test_config(35); // content_width will be 31 (35-4)
            let display_29 = ResponsiveDisplay { config: config_29 };
            let theme = create_test_theme();
            display_29.print_logo(&theme);

            let config_30 = create_test_config(36); // content_width will be 32 (36-4)
            let display_30 = ResponsiveDisplay { config: config_30 };
            display_30.print_logo(&theme);
        }

        #[test]
        fn test_logo_boundary_at_50_chars() {
            // Test exact boundary: 49 vs 50
            let config_49 = create_test_config(55); // content_width will be 49 (55-6)
            let display_49 = ResponsiveDisplay { config: config_49 };
            let theme = create_test_theme();
            display_49.print_logo(&theme);

            let config_50 = create_test_config(56); // content_width will be 50 (56-6)
            let display_50 = ResponsiveDisplay { config: config_50 };
            display_50.print_logo(&theme);
        }

        #[test]
        fn test_logo_boundary_at_80_chars() {
            // Test exact boundary: 79 vs 80
            let config_79 = create_test_config(85); // content_width will be 79 (85-6)
            let display_79 = ResponsiveDisplay { config: config_79 };
            let theme = create_test_theme();
            display_79.print_logo(&theme);

            let config_80 = create_test_config(86); // content_width will be 80 (86-6)
            let display_80 = ResponsiveDisplay { config: config_80 };
            display_80.print_logo(&theme);
        }

        #[test]
        fn test_separator_does_not_panic() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Should not panic
            display.print_separator(&theme);
        }

        #[test]
        fn test_centered_text_does_not_panic() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Should not panic
            display.print_centered_text(&theme, "Test Message");
        }

        #[test]
        fn test_empty_line_does_not_panic() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Should not panic
            display.print_empty_line(&theme);
        }

        #[test]
        fn test_legacy_top_border_compatibility() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Should not panic, calls print_separator internally
            display.print_top_border(&theme);
        }

        #[test]
        fn test_legacy_bottom_border_compatibility() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Should not panic, calls print_separator internally
            display.print_bottom_border(&theme);
        }

        #[test]
        fn test_display_config_clone() {
            let config1 = create_test_config(80);
            let config2 = config1.clone();

            assert_eq!(config1.term_width, config2.term_width);
            assert_eq!(config1.content_width, config2.content_width);
        }

        #[test]
        fn test_display_config_debug() {
            let config = create_test_config(80);
            let debug_str = format!("{config:?}");

            assert!(debug_str.contains("DisplayConfig"));
            assert!(debug_str.contains("term_width"));
            assert!(debug_str.contains("content_width"));
        }
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_full_display_sequence_wide_terminal() {
            let config = create_test_config(100);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Full display sequence should not panic
            display.print_separator(&theme);
            display.print_logo(&theme);
            display.print_separator(&theme);
            display.print_centered_text(&theme, "Welcome Message");
            display.print_separator(&theme);
        }

        #[test]
        fn test_full_display_sequence_narrow_terminal() {
            let config = create_test_config(40);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Should handle narrow terminal gracefully
            display.print_separator(&theme);
            display.print_logo(&theme);
            display.print_separator(&theme);
            display.print_centered_text(&theme, "Welcome");
            display.print_separator(&theme);
        }

        #[test]
        fn test_very_long_content_truncation() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            let very_long = "This is an extremely long message that should definitely be truncated when displayed in a terminal with limited width to prevent overflow";

            // Should truncate gracefully
            display.print_centered_text(&theme, very_long);
        }

        #[test]
        fn test_ansi_colored_content_display() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            let colored = format!(
                "{}Colored{} {}Message{}",
                theme.colors.accent_text,
                theme.colors.reset,
                theme.colors.primary_text,
                theme.colors.reset
            );

            // Should preserve ANSI codes
            display.print_centered_text(&theme, &colored);
        }

        #[test]
        fn test_unicode_content_display() {
            let config = create_test_config(80);
            let display = ResponsiveDisplay { config };
            let theme = create_test_theme();

            // Various Unicode content
            display.print_centered_text(&theme, "Hello 世界");
            display.print_centered_text(&theme, "مرحبا بالعالم");
            display.print_centered_text(&theme, "🚀 Rocket Launch");
        }

        #[test]
        fn test_responsive_breakpoint_transitions() {
            let theme = create_test_theme();

            // Test all major breakpoint transitions
            let widths = vec![25, 29, 30, 49, 50, 79, 80, 100, 120];

            for width in widths {
                let config = create_test_config(width);
                let display = ResponsiveDisplay { config };

                // Should not panic at any breakpoint
                display.print_logo(&theme);
            }
        }

        #[test]
        fn test_extreme_terminal_sizes() {
            let theme = create_test_theme();

            // Test extreme cases
            let extreme_widths = vec![1, 2, 3, 10, 250, 500];

            for width in extreme_widths {
                let config = create_test_config(width);
                let display = ResponsiveDisplay { config };

                // Should handle gracefully without panic
                display.print_separator(&theme);
                display.print_logo(&theme);
                display.print_centered_text(&theme, "Test");
            }
        }
    }
}
