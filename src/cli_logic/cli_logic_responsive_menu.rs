use crate::theme::Theme;
use inquire::ui::{Attributes, Color as InquireColor, RenderConfig, StyleSheet, Styled};
use inquire::Select;

/// Create an inquire Select menu styled with Terminal Jarvis theme
/// This renders inline without clearing the screen
pub fn create_themed_select<'a>(
    theme: &Theme,
    prompt: &'a str,
    options: Vec<String>,
) -> Select<'a, String> {
    let render_config = create_render_config(theme);

    Select::new(prompt, options)
        .with_render_config(render_config)
        .with_page_size(10)
        .with_vim_mode(true)
}

/// Map theme to appropriate InquireColor
/// Inquire supports: Black, DarkGrey, Red, DarkRed, Green, DarkGreen, Yellow, DarkYellow,
/// Blue, DarkBlue, Magenta, DarkMagenta, Cyan, DarkCyan, White, Grey, LightRed, LightGreen,
/// LightYellow, LightBlue, LightMagenta, LightCyan
fn get_theme_color(theme: &Theme) -> InquireColor {
    match theme.name {
        "Default" => InquireColor::LightCyan,   // Cyan theme
        "Minimal" => InquireColor::DarkCyan,    // Minimal cyan
        "Terminal" => InquireColor::LightGreen, // Green theme
        _ => InquireColor::LightCyan,           // Default fallback
    }
}

/// Create inquire RenderConfig using the active theme
fn create_render_config(theme: &Theme) -> RenderConfig<'static> {
    let theme_color = get_theme_color(theme);

    RenderConfig::default()
        .with_highlighted_option_prefix(Styled::new("▶").with_fg(theme_color))
        .with_selected_option(Some(
            StyleSheet::new()
                .with_attr(Attributes::BOLD)
                .with_fg(theme_color),
        ))
        .with_prompt_prefix(Styled::new("?").with_fg(theme_color))
        .with_scroll_up_prefix(Styled::new("↑").with_fg(theme_color))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(theme_color))
        .with_option(StyleSheet::new().with_fg(theme_color))
}
