// Themed Components for inquire-based UI
//
// Provides unified, consistent theming for all inquire components:
// - Select menus
// - MultiSelect
// - Text input
// - Confirm dialogs
//
// Design principles:
// 1. Clean, minimalistic appearance
// 2. Consistent color usage across all components
// 3. Theme-aware: colors adapt to TJarvis/Classic/Matrix themes

use crate::theme::{theme_global_config, Theme};
use inquire::ui::{Attributes, Color as InquireColor, RenderConfig, StyleSheet, Styled};
use inquire::{Confirm, MultiSelect, Select, Text};

/// Color palette derived from theme
/// Maps semantic colors (primary, accent, muted) to inquire colors
struct ThemePalette {
    primary: InquireColor,
    accent: InquireColor,
    muted: InquireColor,
    highlight: InquireColor,
}

impl ThemePalette {
    /// Create palette from theme name
    fn from_theme(theme: &Theme) -> Self {
        match theme.name {
            "Default" => Self {
                // TJarvis: Cyan-based professional look
                primary: InquireColor::LightCyan,
                accent: InquireColor::DarkCyan,
                muted: InquireColor::DarkGrey,
                highlight: InquireColor::White,
            },
            "Minimal" => Self {
                // Classic: Clean gray/white minimal aesthetic
                primary: InquireColor::White,
                accent: InquireColor::DarkCyan,
                muted: InquireColor::DarkGrey,
                highlight: InquireColor::LightCyan,
            },
            "Terminal" => Self {
                // Matrix: Green terminal hacker aesthetic
                primary: InquireColor::LightGreen,
                accent: InquireColor::DarkGreen,
                muted: InquireColor::DarkGrey,
                highlight: InquireColor::White,
            },
            _ => Self {
                primary: InquireColor::LightCyan,
                accent: InquireColor::DarkCyan,
                muted: InquireColor::DarkGrey,
                highlight: InquireColor::White,
            },
        }
    }
}

/// Create a unified RenderConfig for inquire components
/// This is the single source of truth for menu styling
fn create_render_config(theme: &Theme) -> RenderConfig<'static> {
    let palette = ThemePalette::from_theme(theme);

    RenderConfig::default()
        // Prompt styling
        .with_prompt_prefix(Styled::new("?").with_fg(palette.accent))
        .with_answered_prompt_prefix(Styled::new("✓").with_fg(palette.accent))
        // Option styling - clean and readable
        .with_option(StyleSheet::new().with_fg(palette.primary))
        // Highlighted option - bold with marker
        .with_highlighted_option_prefix(Styled::new("▶").with_fg(palette.accent))
        .with_selected_option(Some(
            StyleSheet::new()
                .with_fg(palette.highlight)
                .with_attr(Attributes::BOLD),
        ))
        // Input styling
        .with_text_input(StyleSheet::new().with_fg(palette.primary))
        .with_default_value(StyleSheet::new().with_fg(palette.muted))
        // Help and hints
        .with_help_message(StyleSheet::new().with_fg(palette.muted))
        // Scroll indicators
        .with_scroll_up_prefix(Styled::new("↑").with_fg(palette.muted))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(palette.muted))
}

/// Create a RenderConfig optimized for autocomplete suggestions
/// Uses muted colors for the suggestion list to reduce visual noise
fn create_autocomplete_config(theme: &Theme) -> RenderConfig<'static> {
    let palette = ThemePalette::from_theme(theme);

    RenderConfig::default()
        // Minimal prompt - just a chevron
        .with_prompt_prefix(Styled::new(">").with_fg(palette.accent))
        .with_answered_prompt_prefix(Styled::new(">").with_fg(palette.accent))
        // Text input is primary color (what user types)
        .with_text_input(StyleSheet::new().with_fg(palette.primary))
        // Suggestions are muted (background hints)
        .with_option(StyleSheet::new().with_fg(palette.muted))
        .with_default_value(StyleSheet::new().with_fg(palette.muted))
        // Selected suggestion pops with accent
        .with_highlighted_option_prefix(Styled::new(">").with_fg(palette.accent))
        .with_selected_option(Some(StyleSheet::new().with_fg(palette.accent)))
        // Minimal help
        .with_help_message(StyleSheet::new().with_fg(palette.muted))
        // Scroll indicators
        .with_scroll_up_prefix(Styled::new("↑").with_fg(palette.muted))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(palette.muted))
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
        .with_vim_mode(true)
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
        .with_vim_mode(true)
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
        .with_vim_mode(true)
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
    fn test_theme_palette_default() {
        let theme = theme_global_config::current_theme();
        let palette = ThemePalette::from_theme(&theme);
        // Default theme should use cyan-based colors
        assert!(matches!(palette.primary, InquireColor::LightCyan));
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
