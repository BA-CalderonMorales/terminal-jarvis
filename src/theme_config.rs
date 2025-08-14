use crate::theme::{Theme, ThemeType};
use anyhow::Result;
use std::sync::{Mutex, OnceLock};

/// Global theme configuration using thread-safe OnceLock with Mutex for updates
static GLOBAL_THEME_CONFIG: OnceLock<Mutex<ThemeConfig>> = OnceLock::new();

/// Configuration for theme management
pub struct ThemeConfig {
    current_theme: Theme,
}

impl ThemeConfig {
    /// Create a new theme configuration with default T.JARVIS theme
    pub fn new() -> Self {
        Self {
            current_theme: Theme::get(ThemeType::TJarvis),
        }
    }

    /// Create theme config with specified theme
    #[allow(dead_code)]
    pub fn with_theme(theme_type: ThemeType) -> Self {
        Self {
            current_theme: Theme::get(theme_type),
        }
    }

    /// Get the current theme
    pub fn current(&self) -> &Theme {
        &self.current_theme
    }

    /// Set a new theme
    #[allow(dead_code)]
    pub fn set_theme(&mut self, theme_type: ThemeType) {
        self.current_theme = Theme::get(theme_type);
    }

    /// Load theme from configuration file (future enhancement)
    pub fn load_from_config() -> Result<Self> {
        // For now, return default theme
        // In the future, this could read from terminal-jarvis.toml
        Ok(Self::new())
    }

    /// Get available theme options
    #[allow(dead_code)]
    pub fn available_themes() -> Vec<(&'static str, ThemeType)> {
        vec![
            ("T.JARVIS (Default)", ThemeType::TJarvis),
            ("Classic", ThemeType::Classic),
            ("Matrix", ThemeType::Matrix),
        ]
    }

    /// Validate if theme is supported
    #[allow(dead_code)]
    pub fn is_valid_theme(name: &str) -> bool {
        matches!(
            name.to_lowercase().as_str(),
            "t.jarvis" | "tjarvis" | "classic" | "matrix"
        )
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the global theme configuration
pub fn initialize_theme_config() -> Result<()> {
    let config = ThemeConfig::load_from_config()?;
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
