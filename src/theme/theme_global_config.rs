use crate::theme::{Theme, ThemeType};
use anyhow::Result;
use std::sync::{Mutex, OnceLock};

/// Global theme configuration using thread-safe OnceLock with Mutex for updates
static GLOBAL_THEME_CONFIG: OnceLock<Mutex<ThemeConfig>> = OnceLock::new();

/// Configuration for theme management
pub struct ThemeConfig {
    current_theme: Theme,
    current_theme_type: ThemeType,
}

impl ThemeConfig {
    /// Create a new theme configuration with default T.JARVIS theme
    pub fn new() -> Self {
        Self {
            current_theme: Theme::get(ThemeType::TJarvis),
            current_theme_type: ThemeType::TJarvis,
        }
    }

    /// Create theme config with specified theme
    pub fn with_theme(theme_type: ThemeType) -> Self {
        Self {
            current_theme: Theme::get(theme_type),
            current_theme_type: theme_type,
        }
    }

    /// Get the current theme
    pub fn current(&self) -> &Theme {
        &self.current_theme
    }

    /// Get the current theme type
    pub fn current_type(&self) -> ThemeType {
        self.current_theme_type
    }

    /// Set a new theme
    pub fn set_theme(&mut self, theme_type: ThemeType) {
        self.current_theme = Theme::get(theme_type);
        self.current_theme_type = theme_type;
    }

    /// Get available theme options
    pub fn available_themes() -> Vec<(&'static str, ThemeType)> {
        vec![
            ("Default", ThemeType::TJarvis),
            ("Minimal", ThemeType::Classic),
            ("Terminal", ThemeType::Matrix),
        ]
    }

    /// Validate if theme is supported
    #[allow(dead_code)]
    pub fn is_valid_theme(name: &str) -> bool {
        name.parse::<ThemeType>().is_ok()
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the global theme configuration with default theme
pub fn initialize_theme_config() -> Result<()> {
    initialize_theme_config_with(ThemeType::TJarvis)
}

/// Initialize the global theme configuration with a specific theme
pub fn initialize_theme_config_with(theme_type: ThemeType) -> Result<()> {
    let config = ThemeConfig::with_theme(theme_type);
    let _ = GLOBAL_THEME_CONFIG.set(Mutex::new(config));
    Ok(())
}

/// Set the global theme
pub fn set_theme(theme_type: ThemeType) {
    if let Some(config_mutex) = GLOBAL_THEME_CONFIG.get() {
        if let Ok(mut config) = config_mutex.lock() {
            config.set_theme(theme_type);
        }
    } else {
        // Initialize if not already done
        let _ = GLOBAL_THEME_CONFIG.set(Mutex::new(ThemeConfig::with_theme(theme_type)));
    }
}

/// Get the current theme
pub fn current_theme() -> Theme {
    GLOBAL_THEME_CONFIG
        .get_or_init(|| Mutex::new(ThemeConfig::new()))
        .lock()
        .map(|config| config.current().clone())
        .unwrap_or_else(|_| Theme::get(ThemeType::TJarvis))
}

/// Get the current theme type
pub fn current_theme_type() -> ThemeType {
    GLOBAL_THEME_CONFIG
        .get_or_init(|| Mutex::new(ThemeConfig::new()))
        .lock()
        .map(|config| config.current_type())
        .unwrap_or(ThemeType::TJarvis)
}
