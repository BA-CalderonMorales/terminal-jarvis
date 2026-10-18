//! Theme: the live palette the whole frame paints with -- chrome and body
//! alike. Swapped by `/theme` in-session and pinned by
//! `TERMINAL_JARVIS_THEME` at boot; every role reads it live, so a swap
//! repaints the entire surface, not just the chrome.

use std::env;
use std::sync::Mutex;

use super::palettes::{self, Palette};
use super::roles::Role;

static ACTIVE: Mutex<(&'static str, Palette)> = Mutex::new(("default", palettes::DEFAULT_PALETTE));

/// Swaps the active palette; false when the name is unknown.
pub fn apply_theme(name: &str) -> bool {
    match palettes::lookup_theme(name) {
        Some((theme_name, palette)) => {
            *ACTIVE.lock().unwrap_or_else(|e| e.into_inner()) = (theme_name, palette);
            true
        }
        None => false,
    }
}

/// The active theme's canonical name.
pub fn active_theme() -> &'static str {
    ACTIVE.lock().unwrap_or_else(|e| e.into_inner()).0
}

/// Advances to the next theme in the sorted cycle; returns the new name.
pub fn cycle_theme() -> &'static str {
    let names = palettes::theme_names();
    let current = active_theme();
    let next = match names.iter().position(|name| *name == current) {
        Some(index) => names[(index + 1) % names.len()],
        None => names[0],
    };
    apply_theme(next);
    next
}

/// The sorted theme list, for `/theme` with no argument.
pub fn theme_names() -> Vec<&'static str> {
    palettes::theme_names()
}

/// Pins the theme from the environment at boot; wrong names are ignored.
pub fn boot_from_env() {
    if let Ok(name) = env::var("TERMINAL_JARVIS_THEME") {
        apply_theme(&name);
    }
}

/// The active palette, read live so `/theme` swaps take effect mid-session.
fn palette() -> Palette {
    ACTIVE.lock().unwrap_or_else(|e| e.into_inner()).1
}

/// The active SGR code for a role; empty when the theme leaves it unstyled.
pub fn code(role: Role) -> &'static str {
    palette().code(role)
}

fn wrap(value: &str, code: &str) -> String {
    if code.is_empty() {
        return value.to_string();
    }
    format!("\x1b[{code}m{value}\x1b[0m")
}

/// Paints text in the active theme's role -- the one styling entry point, so
/// chrome and body both follow a theme swap.
pub fn paint(role: Role, value: &str) -> String {
    wrap(value, code(role))
}

/// The greyed chrome layer: present, never the center of focus.
pub fn dim(value: &str) -> String {
    paint(Role::Dim, value)
}

/// The theme accent, for chrome that may carry the vibe.
pub fn accent(value: &str) -> String {
    paint(Role::Accent, value)
}

/// The second accent: the converse reply side.
pub fn accent2(value: &str) -> String {
    paint(Role::Accent2, value)
}

#[cfg(test)]
pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
#[path = "../../tests/screen_theme.rs"]
mod tests;
