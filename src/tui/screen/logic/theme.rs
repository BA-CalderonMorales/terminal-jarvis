//! Theme: the live palette the viewport chrome paints with -- the
//! greyed-out dim layer and the accent. Swapped by `/theme` in-session
//! and pinned by `TERMINAL_JARVIS_THEME` at boot.

use std::env;
use std::sync::Mutex;

use super::palettes::{self, Palette};

static ACTIVE: Mutex<Palette> = Mutex::new(Palette {
    dim: "2",
    accent: "1;36",
});

/// Swaps the active palette; false when the name is unknown. The default
/// theme restores the shipped look exactly.
pub fn apply_theme(name: &str) -> bool {
    match palettes::lookup_theme(name) {
        Some(palette) => {
            *ACTIVE.lock().unwrap_or_else(|e| e.into_inner()) = palette;
            true
        }
        None => false,
    }
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
    *ACTIVE.lock().unwrap_or_else(|e| e.into_inner())
}

fn paint(value: &str, code: &str) -> String {
    format!("\x1b[{code}m{value}\x1b[0m")
}

/// The greyed-out chrome layer: present, never the center of focus.
pub fn dim(value: &str) -> String {
    paint(value, palette().dim)
}

/// The theme accent, for chrome that may carry the vibe.
pub fn accent(value: &str) -> String {
    paint(value, palette().accent)
}

#[cfg(test)]
pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
#[path = "../../tests/screen_theme.rs"]
mod tests;
