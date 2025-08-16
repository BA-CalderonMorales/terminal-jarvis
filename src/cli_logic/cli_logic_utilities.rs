use crate::theme_config;
use anyhow::Result;
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use inquire::MultiSelect;

/// Create inquire RenderConfig based on current theme
pub fn get_themed_render_config() -> RenderConfig<'static> {
    let theme = theme_config::current_theme();

    // Map our theme to inquire colors based on theme name
    let (primary_color, accent_color, secondary_color) = match theme.name {
        "T.JARVIS" => (Color::DarkCyan, Color::LightCyan, Color::DarkBlue),
        "Classic" => (Color::White, Color::DarkCyan, Color::DarkGrey),
        "Matrix" => (Color::DarkGreen, Color::LightGreen, Color::Black),
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

/// Apply themed render config to MultiSelect as well
pub fn apply_theme_to_multiselect<T: std::fmt::Display>(multiselect: MultiSelect<T>) -> MultiSelect<T> {
    multiselect.with_render_config(get_themed_render_config())
}

/// Display a formatted welcome message with version info
pub async fn display_welcome_message() -> Result<()> {
    let theme = theme_config::current_theme();
    
    // Display welcome message with proper theming
    println!("{}", theme.primary("┌────────────────────────────────────────────────────────────┐"));
    println!("{}", theme.primary(&format!("│                     T.JARVIS v{}                      │", env!("CARGO_PKG_VERSION"))));
    println!("{}", theme.primary("│              AI Coding Tools Command Center               │"));
    println!("{}", theme.primary("└────────────────────────────────────────────────────────────┘"));
    println!();
    
    Ok(())
}

/// Show a formatted error message using current theme
pub fn show_error(message: &str) {
    let theme = theme_config::current_theme();
    eprintln!("{}", theme.accent(&format!("Error: {}", message)));
}

/// Show a formatted success message using current theme
pub fn show_success(message: &str) {
    let theme = theme_config::current_theme();
    println!("{}", theme.primary(&format!("✓ {}", message)));
}

/// Show a formatted info message using current theme
pub fn show_info(message: &str) {
    let theme = theme_config::current_theme();
    println!("{}", theme.secondary(&format!("ℹ {}", message)));
}
