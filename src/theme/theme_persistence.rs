// Theme Persistence
//
// Bridges the theme system with the database layer.
// Provides async functions to load and save theme preferences.

use crate::db::core::connection::DatabaseManager;
use crate::db::preferences::PreferencesRepository;
use crate::theme::ThemeType;
use anyhow::Result;
use std::sync::Arc;

/// Load the saved theme preference from the database
///
/// Returns the stored ThemeType or None if not set.
pub async fn load_theme_preference(db: Arc<DatabaseManager>) -> Result<Option<ThemeType>> {
    let prefs = PreferencesRepository::new(db);
    let theme_str = prefs.get_theme().await?;

    match theme_str {
        Some(s) => match s.parse::<ThemeType>() {
            Ok(theme_type) => Ok(Some(theme_type)),
            Err(_) => {
                // Invalid stored value, return None to use default
                Ok(None)
            }
        },
        None => Ok(None),
    }
}

/// Save the theme preference to the database
pub async fn save_theme_preference(db: Arc<DatabaseManager>, theme: ThemeType) -> Result<()> {
    let prefs = PreferencesRepository::new(db);
    prefs.set_theme(&theme.to_string()).await
}

/// Load theme preference synchronously using tokio runtime
///
/// This is a convenience wrapper for use in synchronous contexts.
/// If no runtime exists, it will create a blocking one.
pub fn load_theme_preference_sync(db: Arc<DatabaseManager>) -> Option<ThemeType> {
    // Try to use existing runtime, otherwise create blocking one
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => {
            // We're inside a tokio runtime, use block_in_place
            tokio::task::block_in_place(|| handle.block_on(load_theme_preference(db))).ok()?
        }
        Err(_) => {
            // No runtime, create a simple blocking executor
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .ok()?;
            rt.block_on(load_theme_preference(db)).ok()?
        }
    }
}

/// Save theme preference synchronously using tokio runtime
///
/// This is a convenience wrapper for use in synchronous contexts.
pub fn save_theme_preference_sync(db: Arc<DatabaseManager>, theme: ThemeType) -> Result<()> {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => {
            tokio::task::block_in_place(|| handle.block_on(save_theme_preference(db, theme)))
        }
        Err(_) => {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            rt.block_on(save_theme_preference(db, theme))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_theme_persistence_roundtrip() -> Result<()> {
        let db = DatabaseManager::new_in_memory().await?;
        db.run_migrations().await?;
        let db = Arc::new(db);

        // Initially no theme preference
        let loaded = load_theme_preference(db.clone()).await?;
        assert!(loaded.is_none());

        // Save a theme preference
        save_theme_preference(db.clone(), ThemeType::Matrix).await?;

        // Should now load it back
        let loaded = load_theme_preference(db.clone()).await?;
        assert_eq!(loaded, Some(ThemeType::Matrix));

        // Update to different theme
        save_theme_preference(db.clone(), ThemeType::Classic).await?;
        let loaded = load_theme_preference(db.clone()).await?;
        assert_eq!(loaded, Some(ThemeType::Classic));

        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_theme_returns_none() -> Result<()> {
        let db = DatabaseManager::new_in_memory().await?;
        db.run_migrations().await?;
        let db = Arc::new(db);

        // Manually insert invalid theme value
        let prefs = PreferencesRepository::new(db.clone());
        prefs.set_theme("invalid_theme_name").await?;

        // Should return None, not error
        let loaded = load_theme_preference(db).await?;
        assert!(loaded.is_none());

        Ok(())
    }
}
