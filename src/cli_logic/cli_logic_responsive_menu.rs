use crate::theme::Theme;
use inquire::ui::{Attributes, Color as InquireColor, RenderConfig, StyleSheet, Styled};
use inquire::Select;

/// Create an inquire Select menu styled with Terminal Jarvis theme
/// This renders inline without clearing the screen
pub fn create_themed_select<'a>(
    _theme: &Theme,
    prompt: &'a str,
    options: Vec<String>,
) -> Select<'a, String> {
    let render_config = create_render_config();

    Select::new(prompt, options)
        .with_render_config(render_config)
        .with_page_size(10)
        .with_vim_mode(true)
}

/// Create inquire RenderConfig for Terminal Jarvis theme
fn create_render_config() -> RenderConfig<'static> {
    RenderConfig {
        highlighted_option_prefix: Styled::new("▶").with_fg(InquireColor::LightCyan),
        selected_option: Some(StyleSheet::new().with_attr(Attributes::BOLD)),
        prompt_prefix: Styled::new("?").with_fg(InquireColor::LightCyan),
        scroll_up_prefix: Styled::new("↑"),
        scroll_down_prefix: Styled::new("↓"),
        ..Default::default()
    }
}
