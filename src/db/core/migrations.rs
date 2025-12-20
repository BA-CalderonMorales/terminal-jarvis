// Database Migrations
//
// Schema versioning and migrations using schema definitions.
// No hardcoded SQL - all DDL generated from schema.rs
// All queries use QueryBuilder for consistency.

use super::connection::DatabaseManager;
use super::query_builder::QueryBuilder;
use super::schema::{MIGRATIONS_TABLE, SCHEMA};
use anyhow::Result;
use std::sync::Arc;

/// Migration entry
struct Migration {
    version: i32,
    name: &'static str,
}

/// All migrations in order
const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "initial_schema",
}];

/// Run all pending migrations
pub async fn run_migrations(db: &Arc<DatabaseManager>) -> Result<()> {
    // First, ensure migrations table exists (bootstrap)
    let migrations_ddl = MIGRATIONS_TABLE.create_table_sql();
    db.execute(&migrations_ddl, ()).await?;

    // Get current version
    let current_version = get_current_version(db).await?;

    // Apply pending migrations
    for migration in MIGRATIONS {
        if migration.version > current_version {
            apply_migration(db, migration).await?;
        }
    }

    Ok(())
}

/// Get the current schema version
async fn get_current_version(db: &Arc<DatabaseManager>) -> Result<i32> {
    // Use QueryBuilder for SELECT MAX
    let sql = QueryBuilder::select(&MIGRATIONS_TABLE)
        .columns(&["version"])
        .order_by("version", false) // DESC
        .limit(1)
        .build();

    let mut rows = db.query(&sql, ()).await?;

    if let Some(row) = rows.next().await? {
        let version: Option<i32> = row.get(0)?;
        Ok(version.unwrap_or(0))
    } else {
        Ok(0)
    }
}

/// Apply a specific migration
async fn apply_migration(db: &Arc<DatabaseManager>, migration: &Migration) -> Result<()> {
    match migration.version {
        1 => migrate_v1_initial_schema(db).await?,
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown migration version: {}",
                migration.version
            ))
        }
    }

    // Record the migration using QueryBuilder
    let sql = QueryBuilder::insert(&MIGRATIONS_TABLE)
        .columns(&["version", "name"])
        .build();

    db.execute(&sql, libsql::params![migration.version, migration.name])
        .await?;

    Ok(())
}

/// Migration v1: Create all initial tables from schema definitions
async fn migrate_v1_initial_schema(db: &Arc<DatabaseManager>) -> Result<()> {
    // Create all tables defined in SCHEMA (except migrations table, already exists)
    for table in SCHEMA.tables {
        if table.name == "schema_migrations" {
            continue; // Already created in bootstrap
        }

        let ddl = table.create_table_sql();
        db.execute(&ddl, ()).await?;
    }

    Ok(())
}

/// Get schema version info
pub fn get_schema_version() -> i32 {
    SCHEMA.version
}

/// Get list of all tables in schema
pub fn get_table_names() -> Vec<&'static str> {
    SCHEMA.tables.iter().map(|t| t.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations_defined() {
        assert!(!MIGRATIONS.is_empty());
        assert_eq!(MIGRATIONS[0].version, 1);
    }

    #[test]
    fn test_schema_version() {
        assert!(get_schema_version() >= 1);
    }

    #[test]
    fn test_table_names() {
        let names = get_table_names();
        assert!(names.contains(&"tools"));
        assert!(names.contains(&"preferences"));
    }
}
