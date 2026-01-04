use crate::theme::theme_global_config;
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use inquire::MultiSelect;

/// Create inquire RenderConfig based on current theme
pub fn get_themed_render_config() -> RenderConfig<'static> {
    let theme = theme_global_config::current_theme();

    // Map our theme to inquire colors based on theme name
    let (primary_color, accent_color, secondary_color) = match theme.name {
        "Default" => (Color::DarkCyan, Color::LightCyan, Color::DarkBlue),
        "Minimal" => (Color::White, Color::DarkCyan, Color::DarkGrey),
        "Terminal" => (Color::DarkGreen, Color::LightGreen, Color::Black),
        _ => (Color::DarkCyan, Color::LightCyan, Color::DarkGrey),
    };

    RenderConfig::default()
        .with_prompt_prefix(Styled::new("?").with_fg(accent_color))
        .with_answered_prompt_prefix(Styled::new("✓").with_fg(accent_color))
        .with_default_value(StyleSheet::new().with_fg(secondary_color))
        .with_help_message(StyleSheet::new().with_fg(secondary_color))
        .with_text_input(StyleSheet::new().with_fg(primary_color))
        .with_highlighted_option_prefix(Styled::new(">").with_fg(accent_color))
        .with_option(StyleSheet::new().with_fg(primary_color))
        .with_selected_option(Some(
            StyleSheet::new()
                .with_fg(accent_color)
                .with_attr(inquire::ui::Attributes::BOLD),
        ))
        .with_scroll_up_prefix(Styled::new("↑").with_fg(accent_color))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(accent_color))
}

/// Create render config for autocomplete with dimmer suggestions
/// Uses darker/lighter variants for suggestion list to distinguish from primary text
pub fn get_autocomplete_render_config() -> RenderConfig<'static> {
    let theme = theme_global_config::current_theme();

    // Map theme to colors - suggestions should be dimmer than primary text
    // Primary text stays bright, suggestions are muted/secondary
    let (primary_color, accent_color, suggestion_color, help_color) = match theme.name {
        "Default" => (
            Color::LightCyan,
            Color::DarkCyan,
            Color::DarkGrey,
            Color::DarkBlue,
        ),
        "Minimal" => (
            Color::White,
            Color::DarkCyan,
            Color::DarkGrey,
            Color::DarkGrey,
        ),
        "Terminal" => (
            Color::LightGreen,
            Color::DarkGreen,
            Color::DarkGrey,
            Color::Black,
        ),
        _ => (
            Color::LightCyan,
            Color::DarkCyan,
            Color::DarkGrey,
            Color::DarkGrey,
        ),
    };

    RenderConfig::default()
        .with_prompt_prefix(Styled::new(">").with_fg(accent_color))
        .with_answered_prompt_prefix(Styled::new(">").with_fg(accent_color))
        .with_default_value(StyleSheet::new().with_fg(suggestion_color))
        .with_help_message(StyleSheet::new().with_fg(help_color))
        .with_text_input(StyleSheet::new().with_fg(primary_color))
        // Suggestions list - dimmer color
        .with_option(StyleSheet::new().with_fg(suggestion_color))
        // Highlighted/selected suggestion - accent color (brighter when selected)
        .with_highlighted_option_prefix(Styled::new(">").with_fg(accent_color))
        .with_selected_option(Some(StyleSheet::new().with_fg(accent_color)))
        .with_scroll_up_prefix(Styled::new("↑").with_fg(suggestion_color))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(suggestion_color))
}

/// Apply themed render config to MultiSelect as well
pub fn apply_theme_to_multiselect<T: std::fmt::Display>(
    multiselect: MultiSelect<T>,
) -> MultiSelect<T> {
    multiselect.with_render_config(get_themed_render_config())
}
