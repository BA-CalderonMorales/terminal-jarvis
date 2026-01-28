// Database Connection Manager
//
// Handles libSQL database connection lifecycle, including:
// - Embedded local database (default)
// - Optional Turso cloud sync
// - Connection pooling

use anyhow::Result;
use libsql::{Builder, Connection, Database};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Global database instance
static DB_INSTANCE: OnceCell<Arc<DatabaseManager>> = OnceCell::const_new();

/// Database manager for Terminal Jarvis
pub struct DatabaseManager {
    db: Database,
}

impl DatabaseManager {
    /// Get the database file path
    pub fn get_db_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".terminal-jarvis").join("jarvis.db"))
    }

    /// Initialize the database (creates if not exists)
    pub async fn init() -> Result<Arc<Self>> {
        DB_INSTANCE
            .get_or_try_init(|| async {
                let db_path = Self::get_db_path()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

                // Ensure directory exists
                if let Some(parent) = db_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                let db = Builder::new_local(db_path).build().await?;

                let manager = Arc::new(Self { db });

                // Run migrations
                super::migrations::run_migrations(&manager).await?;

                Ok(manager)
            })
            .await
            .cloned()
    }

    /// Get the global database instance (must call init first)
    pub async fn get() -> Result<Arc<Self>> {
        Self::init().await
    }

    /// Create an in-memory database for testing
    #[cfg(test)]
    pub async fn new_in_memory() -> Result<Self> {
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::time::{SystemTime, UNIX_EPOCH};

        // Use unique temp files for each test to avoid conflicts
        // Include timestamp and counter to ensure uniqueness across runs
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_path =
            std::env::temp_dir().join(format!("terminal_jarvis_test_{timestamp}_{id}.db"));

        // Clean up any existing file from previous runs
        let _ = std::fs::remove_file(&temp_path);

        let db = Builder::new_local(temp_path).build().await?;
        Ok(Self { db })
    }

    /// Run migrations on the database
    #[cfg(test)]
    pub async fn run_migrations(&self) -> Result<()> {
        super::migrations::run_migrations_on_connection(self).await
    }

    /// Get a connection to the database
    pub async fn connection(&self) -> Result<Connection> {
        Ok(self.db.connect()?)
    }

    /// Execute a query that doesn't return rows
    pub async fn execute(&self, sql: &str, params: impl libsql::params::IntoParams) -> Result<u64> {
        let conn = self.connection().await?;
        let rows_affected = conn.execute(sql, params).await?;
        Ok(rows_affected)
    }

    /// Query and return rows
    pub async fn query(
        &self,
        sql: &str,
        params: impl libsql::params::IntoParams,
    ) -> Result<libsql::Rows> {
        let conn = self.connection().await?;
        let rows = conn.query(sql, params).await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_path_exists() {
        let path = DatabaseManager::get_db_path();
        assert!(path.is_some());
        assert!(path.unwrap().to_string_lossy().contains("jarvis.db"));
    }
}
