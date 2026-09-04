//! Palettes: the named theme table. Adding or dropping a theme is a
//! one-entry change in [`THEMES`]; `default` is what the screen shipped
//! with, so the boot look never changes.

/// The scoped code set every chrome style derives from. ANSI SGR codes,
/// so the whole system stays std-only.
#[derive(Clone, Copy)]
pub struct Palette {
    pub dim: &'static str,
    pub accent: &'static str,
}

struct Theme {
    name: &'static str,
    palette: Palette,
}

const DEFAULT: Theme = Theme {
    name: "default",
    palette: Palette {
        dim: "2",
        accent: "1;36",
    },
};
const MIDNIGHT: Theme = Theme {
    name: "midnight",
    palette: Palette {
        dim: "2",
        accent: "1;34",
    },
};
const EMBER: Theme = Theme {
    name: "ember",
    palette: Palette {
        dim: "2",
        accent: "1;31",
    },
};
const MOSS: Theme = Theme {
    name: "moss",
    palette: Palette {
        dim: "2",
        accent: "1;32",
    },
};
const SOLARIZED: Theme = Theme {
    name: "solarized",
    palette: Palette {
        dim: "2",
        accent: "1;34",
    },
};
const MONO: Theme = Theme {
    name: "mono",
    palette: Palette {
        dim: "2",
        accent: "1",
    },
};

const THEMES: [Theme; 6] = [DEFAULT, MIDNIGHT, EMBER, MOSS, SOLARIZED, MONO];

/// Sorted theme names, the default first -- the settings-facing list.
pub fn theme_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = THEMES.iter().map(|theme| theme.name).collect();
    names.sort_unstable();
    if let Some(position) = names.iter().position(|name| *name == DEFAULT.name) {
        names.swap(0, position);
    }
    names
}

/// Resolves a theme name (case-, hyphen-, underscore-, space-insensitive).
pub fn lookup_theme(name: &str) -> Option<Palette> {
    let norm = normalize(name);
    THEMES
        .iter()
        .find(|theme| normalize(theme.name) == norm)
        .map(|theme| theme.palette)
}

fn normalize(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .flat_map(|c| c.to_lowercase())
        .collect()
}
