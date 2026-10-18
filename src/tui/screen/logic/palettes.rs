//! Palettes: the named theme table. A theme answers every [`Role`] with one
//! SGR code; adding or dropping a theme is a one-row change. `default` is
//! what the screen shipped with, so the boot look never changes.

use super::roles::{Role, COUNT};

/// The scoped code set every chrome and body style derives from. ANSI SGR
/// codes, so the whole system stays std-only.
#[derive(Clone, Copy)]
pub struct Palette {
    codes: [&'static str; COUNT],
}

impl Palette {
    /// The SGR code for a role; empty means "leave the text unstyled".
    pub fn code(&self, role: Role) -> &'static str {
        self.codes[role as usize]
    }
}

/// Row order is [`Role`] order:
/// title, accent, accent2, label, value, dim, ok, warn, err, rule.
const THEMES: [(&str, [&str; COUNT]); 6] = [
    (
        "default",
        [
            "1;36", "1;36", "1;35", "1;37", "", "2", "1;32", "1;33", "1;31", "2",
        ],
    ),
    (
        "midnight",
        [
            "1;34", "1;34", "1;36", "1;37", "", "2", "1;32", "1;33", "1;31", "2",
        ],
    ),
    (
        "ember",
        [
            "1;31", "1;31", "1;33", "1;37", "", "2", "1;32", "1;33", "1;31", "2",
        ],
    ),
    (
        "moss",
        [
            "1;32", "1;32", "1;36", "1;37", "", "2", "1;32", "1;33", "1;31", "2",
        ],
    ),
    (
        "solarized",
        [
            "1;34", "1;34", "1;33", "1;37", "", "2", "1;32", "1;33", "1;31", "2",
        ],
    ),
    ("mono", ["1", "1", "1", "1", "", "2", "1", "1", "1", "2"]),
];

/// The boot palette; `default` is the first row by construction.
pub const DEFAULT_PALETTE: Palette = Palette { codes: THEMES[0].1 };

const DEFAULT_NAME: &str = "default";

/// Sorted theme names, the default first -- the settings-facing list.
pub fn theme_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = THEMES.iter().map(|(name, _)| *name).collect();
    names.sort_unstable();
    if let Some(position) = names.iter().position(|name| *name == DEFAULT_NAME) {
        names.swap(0, position);
    }
    names
}

/// Resolves a theme name (case-, hyphen-, underscore-, space-insensitive)
/// to its canonical name and palette.
pub fn lookup_theme(name: &str) -> Option<(&'static str, Palette)> {
    let norm = normalize(name);
    THEMES
        .iter()
        .find(|(theme, _)| normalize(theme) == norm)
        .map(|(theme, codes)| (*theme, Palette { codes: *codes }))
}

fn normalize(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .flat_map(|c| c.to_lowercase())
        .collect()
}
