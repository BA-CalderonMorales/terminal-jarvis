use dialoguer::MultiSelect;

/// Create a themed MultiSelect - returns a MultiSelect without applying theme due to lifetime issues
pub fn apply_theme_to_multiselect<T: std::fmt::Display + Clone>(
    prompt: &str,
    items: Vec<T>,
) -> MultiSelect {
    // Note: We use the default theme here since lifetime management with custom themes is complex
    MultiSelect::new().with_prompt(prompt).items(&items)
}
