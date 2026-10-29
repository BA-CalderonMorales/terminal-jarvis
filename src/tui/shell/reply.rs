//! Reply: the shell's answer for a local command that changes no fleet
//! state. Kept apart from the resolver so the wording can be read, tested,
//! and changed without widening the dispatch ladder.

/// `/theme` reply: with no argument, cycle to the next theme; with one, pin
/// it. Unknown names keep the list discoverable.
pub fn theme(choice: Option<&str>) -> String {
    match choice {
        None => format!("theme '{}' applied", crate::tui::screen::cycle_theme()),
        Some(name) if crate::tui::screen::apply_theme(name) => format!("theme '{name}' applied"),
        Some(name) => format!(
            "unknown theme '{name}'; themes: {}",
            crate::tui::screen::theme_names().join(", ")
        ),
    }
}
