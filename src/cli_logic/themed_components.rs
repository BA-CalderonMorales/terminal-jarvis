// Themed Components for inquire-based UI
//
// Provides unified, consistent theming for all inquire components:
// - Select menus
// - MultiSelect
// - Text input
// - Confirm dialogs
//
// Design principles:
// 1. Each theme has a DRAMATICALLY different visual identity
// 2. TJarvis: Modern professional with cyan accents and Unicode symbols
// 3. Classic: Ultra-minimal, monochrome, ASCII-only, understated
// 4. Matrix: Hacker aesthetic with green, bold symbols, terminal feel

use crate::theme::{theme_global_config, Theme};
use inquire::ui::{Attributes, Color as InquireColor, RenderConfig, StyleSheet, Styled};
use inquire::{Confirm, MultiSelect, Select, Text};

/// Visual style elements that differ per theme
/// Each theme has distinct symbols, prefixes, and styling approach
struct ThemeStyle {
    // Colors
    primary: InquireColor,
    accent: InquireColor,
    muted: InquireColor,
    // Symbols and prefixes - these make themes visually distinct
    prompt_prefix: &'static str,
    selected_prefix: &'static str,
    scroll_up: &'static str,
    scroll_down: &'static str,
    autocomplete_prefix: &'static str,
    // Styling attributes
    use_italic_help: bool,
}

impl ThemeStyle {
    /// Create style from theme name - each theme is dramatically different
    fn from_theme(theme: &Theme) -> Self {
        match theme.name {
            "Default" => Self {
                // TJarvis: Modern professional - cyan with Unicode symbols
                primary: InquireColor::LightCyan,
                accent: InquireColor::DarkCyan,
                muted: InquireColor::DarkGrey,
                // Modern Unicode symbols
                prompt_prefix: "?",
                selected_prefix: "▶",
                scroll_up: "↑",
                scroll_down: "↓",
                autocomplete_prefix: "›",
                use_italic_help: false,
            },
            "Minimal" => Self {
                // Classic: Ultra-minimal - grayscale, ASCII only, understated
                primary: InquireColor::Grey,
                accent: InquireColor::White,
                muted: InquireColor::DarkGrey,
                // Simple ASCII - no Unicode
                prompt_prefix: "::",
                selected_prefix: ">",
                scroll_up: "^",
                scroll_down: "v",
                autocomplete_prefix: "-",
                use_italic_help: true, // Italic help for subtle distinction
            },
            "Terminal" => Self {
                // Matrix: Hacker terminal - green, bold, dramatic
                primary: InquireColor::LightGreen,
                accent: InquireColor::DarkGreen,
                muted: InquireColor::DarkGrey,
                // Terminal/hacker symbols
                prompt_prefix: "$",
                selected_prefix: ">>",
                scroll_up: "[^]",
                scroll_down: "[v]",
                autocomplete_prefix: "$",
                use_italic_help: false,
            },
            _ => Self {
                primary: InquireColor::LightCyan,
                accent: InquireColor::DarkCyan,
                muted: InquireColor::DarkGrey,
                prompt_prefix: "?",
                selected_prefix: "▶",
                scroll_up: "↑",
                scroll_down: "↓",
                autocomplete_prefix: "›",
                use_italic_help: false,
            },
        }
    }

    /// Create style directly from theme name - useful for testing
    #[cfg(test)]
    fn from_theme_name(name: &str) -> Self {
        match name {
            "Default" => Self::from_theme(&crate::theme::theme_global_config::current_theme()),
            "Minimal" => Self {
                primary: InquireColor::Grey,
                accent: InquireColor::White,
                muted: InquireColor::DarkGrey,
                prompt_prefix: "::",
                selected_prefix: ">",
                scroll_up: "^",
                scroll_down: "v",
                autocomplete_prefix: "-",
                use_italic_help: true,
            },
            "Terminal" => Self {
                primary: InquireColor::LightGreen,
                accent: InquireColor::DarkGreen,
                muted: InquireColor::DarkGrey,
                prompt_prefix: "$",
                selected_prefix: ">>",
                scroll_up: "[^]",
                scroll_down: "[v]",
                autocomplete_prefix: "$",
                use_italic_help: false,
            },
            _ => Self {
                primary: InquireColor::LightCyan,
                accent: InquireColor::DarkCyan,
                muted: InquireColor::DarkGrey,
                prompt_prefix: "?",
                selected_prefix: "▶",
                scroll_up: "↑",
                scroll_down: "↓",
                autocomplete_prefix: "›",
                use_italic_help: false,
            },
        }
    }
}

/// Create a unified RenderConfig for inquire components
/// This is the single source of truth for menu styling
fn create_render_config(theme: &Theme) -> RenderConfig<'static> {
    let style = ThemeStyle::from_theme(theme);

    // Use a minimal render config to avoid terminal rendering issues
    RenderConfig::default()
        .with_prompt_prefix(Styled::new(style.prompt_prefix).with_fg(style.accent))
        .with_highlighted_option_prefix(Styled::new(style.selected_prefix).with_fg(style.accent))
        .with_scroll_up_prefix(Styled::new(style.scroll_up).with_fg(style.muted))
        .with_scroll_down_prefix(Styled::new(style.scroll_down).with_fg(style.muted))
}

/// Create a RenderConfig optimized for autocomplete suggestions
fn create_autocomplete_config(theme: &Theme) -> RenderConfig<'static> {
    let style = ThemeStyle::from_theme(theme);

    // Build help style
    let help_style = if style.use_italic_help {
        StyleSheet::new()
            .with_fg(style.muted)
            .with_attr(Attributes::ITALIC)
    } else {
        StyleSheet::new().with_fg(style.muted)
    };

    RenderConfig::default()
        // Theme-specific prompt
        .with_prompt_prefix(Styled::new(style.autocomplete_prefix).with_fg(style.accent))
        .with_answered_prompt_prefix(Styled::new(style.autocomplete_prefix).with_fg(style.accent))
        // Text input is primary color
        .with_text_input(StyleSheet::new().with_fg(style.primary))
        // Suggestions are muted
        .with_option(StyleSheet::new().with_fg(style.muted))
        .with_default_value(StyleSheet::new().with_fg(style.muted))
        // Selected suggestion
        .with_highlighted_option_prefix(
            Styled::new(style.autocomplete_prefix).with_fg(style.accent),
        )
        .with_selected_option(Some(StyleSheet::new().with_fg(style.accent)))
        // Help
        .with_help_message(help_style)
        // Scroll indicators
        .with_scroll_up_prefix(Styled::new(style.scroll_up).with_fg(style.muted))
        .with_scroll_down_prefix(Styled::new(style.scroll_down).with_fg(style.muted))
}

// =============================================================================
// Public API - Themed Component Constructors
// =============================================================================

/// Create a themed Select menu
///
/// Example:
/// ```ignore
/// let selection = themed_select("Choose an option:", options)
///     .with_page_size(10)
///     .prompt()?;
/// ```
pub fn themed_select<'a>(prompt: &'a str, options: Vec<String>) -> Select<'a, String> {
    let theme = theme_global_config::current_theme();
    Select::new(prompt, options)
        .with_render_config(create_render_config(&theme))
        .with_page_size(10)
}

/// Create a themed Select menu with explicit theme
///
/// Use when you already have the theme to avoid repeated lookups
pub fn themed_select_with<'a>(
    theme: &Theme,
    prompt: &'a str,
    options: Vec<String>,
) -> Select<'a, String> {
    Select::new(prompt, options)
        .with_render_config(create_render_config(theme))
        .with_page_size(10)
}

/// Create a themed MultiSelect menu
///
/// Example:
/// ```ignore
/// let selections = themed_multiselect("Select items:", options)
///     .with_page_size(10)
///     .prompt()?;
/// ```
pub fn themed_multiselect<'a, T: std::fmt::Display>(
    prompt: &'a str,
    options: Vec<T>,
) -> MultiSelect<'a, T> {
    let theme = theme_global_config::current_theme();
    MultiSelect::new(prompt, options)
        .with_render_config(create_render_config(&theme))
        .with_page_size(10)
}

/// Create a themed Confirm dialog
///
/// Example:
/// ```ignore
/// let confirmed = themed_confirm("Are you sure?")
///     .with_default(false)
///     .prompt()?;
/// ```
pub fn themed_confirm<'a>(prompt: &'a str) -> Confirm<'a> {
    let theme = theme_global_config::current_theme();
    Confirm::new(prompt).with_render_config(create_render_config(&theme))
}

/// Create a themed Text input
///
/// Example:
/// ```ignore
/// let name = themed_text("Enter your name:")
///     .with_placeholder("John Doe")
///     .prompt()?;
/// ```
pub fn themed_text<'a>(prompt: &'a str) -> Text<'a, 'a> {
    let theme = theme_global_config::current_theme();
    Text::new(prompt).with_render_config(create_render_config(&theme))
}

/// Get the autocomplete-specific RenderConfig
///
/// For use with Text::with_autocomplete() to style suggestions dimly
pub fn get_autocomplete_config() -> RenderConfig<'static> {
    let theme = theme_global_config::current_theme();
    create_autocomplete_config(&theme)
}

/// Get the standard RenderConfig for custom component styling
///
/// Use when you need to apply theme to an existing inquire component
pub fn get_render_config() -> RenderConfig<'static> {
    let theme = theme_global_config::current_theme();
    create_render_config(&theme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::theme_global_config;

    #[test]
    fn test_theme_style_default() {
        let theme = theme_global_config::current_theme();
        let style = ThemeStyle::from_theme(&theme);
        // Default theme should use cyan-based colors and Unicode symbols
        assert!(matches!(style.primary, InquireColor::LightCyan));
        assert_eq!(style.prompt_prefix, "?");
        assert_eq!(style.selected_prefix, "▶");
    }

    #[test]
    fn test_theme_styles_are_distinct() {
        // Verify that each theme has distinct visual elements
        // Create styles directly from theme names for testing
        let default_style = ThemeStyle::from_theme_name("Default");
        let minimal_style = ThemeStyle::from_theme_name("Minimal");
        let terminal_style = ThemeStyle::from_theme_name("Terminal");

        // Each theme should have different selection prefixes
        assert_ne!(default_style.selected_prefix, minimal_style.selected_prefix);
        assert_ne!(
            minimal_style.selected_prefix,
            terminal_style.selected_prefix
        );
        assert_ne!(
            default_style.selected_prefix,
            terminal_style.selected_prefix
        );

        // Each theme should have different prompt prefixes
        assert_ne!(default_style.prompt_prefix, minimal_style.prompt_prefix);
        assert_ne!(minimal_style.prompt_prefix, terminal_style.prompt_prefix);
    }

    #[test]
    fn test_themed_select_creates_valid_menu() {
        let options = vec!["Option 1".to_string(), "Option 2".to_string()];
        let _select = themed_select("Choose:", options);
        // If it compiles and doesn't panic, it works
    }

    #[test]
    fn test_themed_confirm_creates_valid_dialog() {
        let _confirm = themed_confirm("Are you sure?");
        // If it compiles and doesn't panic, it works
    }

    #[test]
    fn test_render_config_creation() {
        let config = get_render_config();
        // Verify we get a valid RenderConfig (no panic)
        let _ = config;
    }

    #[test]
    fn test_autocomplete_config_creation() {
        let config = get_autocomplete_config();
        // Verify we get a valid RenderConfig (no panic)
        let _ = config;
    }
}
