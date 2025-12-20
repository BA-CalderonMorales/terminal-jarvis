// Database Schema Definitions
//
// SINGLE SOURCE OF TRUTH for all database tables and columns.
// All SQL generation derives from these definitions.
//
// To add a new table:
// 1. Define the table constant with columns
// 2. Add to SCHEMA.tables
// 3. Run migration to apply

use std::fmt;

/// Column data types supported by libSQL/SQLite
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnType {
    Text,
    Integer,
    Real,
    Blob,
    Boolean,   // Stored as INTEGER 0/1
    Timestamp, // Stored as TEXT (ISO 8601)
    Json,      // Stored as TEXT
}

impl fmt::Display for ColumnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnType::Text => write!(f, "TEXT"),
            ColumnType::Integer => write!(f, "INTEGER"),
            ColumnType::Real => write!(f, "REAL"),
            ColumnType::Blob => write!(f, "BLOB"),
            ColumnType::Boolean => write!(f, "INTEGER"),
            ColumnType::Timestamp => write!(f, "TEXT"),
            ColumnType::Json => write!(f, "TEXT"),
        }
    }
}

/// Column definition
#[derive(Debug, Clone)]
pub struct Column {
    pub name: &'static str,
    pub col_type: ColumnType,
    pub nullable: bool,
    pub primary_key: bool,
    pub default: Option<&'static str>,
    pub references: Option<(&'static str, &'static str)>, // (table, column)
}

impl Column {
    /// Create a new column builder
    pub const fn new(name: &'static str, col_type: ColumnType) -> Self {
        Self {
            name,
            col_type,
            nullable: true,
            primary_key: false,
            default: None,
            references: None,
        }
    }

    /// Mark as primary key
    pub const fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self.nullable = false;
        self
    }

    /// Mark as not null
    pub const fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    /// Set default value
    pub const fn default(mut self, value: &'static str) -> Self {
        self.default = Some(value);
        self
    }

    /// Add foreign key reference
    pub const fn references(mut self, table: &'static str, column: &'static str) -> Self {
        self.references = Some((table, column));
        self
    }

    /// Generate column DDL
    pub fn to_ddl(&self) -> String {
        let mut ddl = format!("{} {}", self.name, self.col_type);

        if self.primary_key {
            ddl.push_str(" PRIMARY KEY");
        }

        if !self.nullable && !self.primary_key {
            ddl.push_str(" NOT NULL");
        }

        if let Some(default) = self.default {
            ddl.push_str(&format!(" DEFAULT {}", default));
        }

        if let Some((ref_table, ref_col)) = self.references {
            ddl.push_str(&format!(" REFERENCES {}({})", ref_table, ref_col));
        }

        ddl
    }
}

/// Table definition
#[derive(Debug, Clone)]
pub struct Table {
    pub name: &'static str,
    pub columns: &'static [Column],
}

impl Table {
    /// Create a new table
    pub const fn new(name: &'static str, columns: &'static [Column]) -> Self {
        Self { name, columns }
    }

    /// Generate CREATE TABLE statement
    pub fn create_table_sql(&self) -> String {
        let columns: Vec<String> = self.columns.iter().map(|c| c.to_ddl()).collect();
        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n    {}\n)",
            self.name,
            columns.join(",\n    ")
        )
    }

    /// Get column names for SELECT
    pub fn column_names(&self) -> Vec<&'static str> {
        self.columns.iter().map(|c| c.name).collect()
    }

    /// Get column by name
    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == name)
    }

    /// Get primary key column
    pub fn primary_key(&self) -> Option<&Column> {
        self.columns.iter().find(|c| c.primary_key)
    }
}

// =============================================================================
// SCHEMA DEFINITIONS - Single Source of Truth
// =============================================================================

/// Schema migrations table
pub static MIGRATIONS_TABLE: Table = Table::new(
    "schema_migrations",
    &[
        Column::new("version", ColumnType::Integer).primary_key(),
        Column::new("name", ColumnType::Text).not_null(),
        Column::new("applied_at", ColumnType::Timestamp).default("CURRENT_TIMESTAMP"),
    ],
);

/// Tools table
pub static TOOLS_TABLE: Table = Table::new(
    "tools",
    &[
        Column::new("id", ColumnType::Text).primary_key(),
        Column::new("display_name", ColumnType::Text).not_null(),
        Column::new("cli_command", ColumnType::Text).not_null(),
        Column::new("description", ColumnType::Text),
        Column::new("homepage", ColumnType::Text),
        Column::new("documentation", ColumnType::Text),
        Column::new("requires_npm", ColumnType::Boolean).default("0"),
        Column::new("requires_sudo", ColumnType::Boolean).default("0"),
        Column::new("status", ColumnType::Text).default("'stable'"),
        Column::new("enabled", ColumnType::Boolean).default("1"),
        Column::new("auto_update", ColumnType::Boolean).default("1"),
        Column::new("created_at", ColumnType::Timestamp).default("CURRENT_TIMESTAMP"),
        Column::new("updated_at", ColumnType::Timestamp).default("CURRENT_TIMESTAMP"),
    ],
);

/// Tool installation commands
pub static TOOL_INSTALL_TABLE: Table = Table::new(
    "tool_install",
    &[
        Column::new("tool_id", ColumnType::Text)
            .primary_key()
            .references("tools", "id"),
        Column::new("command", ColumnType::Text).not_null(),
        Column::new("args", ColumnType::Json),
        Column::new("verify_command", ColumnType::Text),
        Column::new("post_install_message", ColumnType::Text),
    ],
);

/// Tool update commands
pub static TOOL_UPDATE_TABLE: Table = Table::new(
    "tool_update",
    &[
        Column::new("tool_id", ColumnType::Text)
            .primary_key()
            .references("tools", "id"),
        Column::new("command", ColumnType::Text).not_null(),
        Column::new("args", ColumnType::Json),
        Column::new("verify_command", ColumnType::Text),
    ],
);

/// Tool authentication configuration
pub static TOOL_AUTH_TABLE: Table = Table::new(
    "tool_auth",
    &[
        Column::new("tool_id", ColumnType::Text)
            .primary_key()
            .references("tools", "id"),
        Column::new("env_vars", ColumnType::Json),
        Column::new("setup_url", ColumnType::Text),
        Column::new("browser_auth", ColumnType::Boolean).default("0"),
        Column::new("auth_instructions", ColumnType::Text),
    ],
);

/// Tool features/capabilities
pub static TOOL_FEATURES_TABLE: Table = Table::new(
    "tool_features",
    &[
        Column::new("tool_id", ColumnType::Text)
            .primary_key()
            .references("tools", "id"),
        Column::new("supports_files", ColumnType::Boolean).default("0"),
        Column::new("supports_streaming", ColumnType::Boolean).default("0"),
        Column::new("supports_conversation", ColumnType::Boolean).default("0"),
        Column::new("max_context_tokens", ColumnType::Integer),
        Column::new("supported_languages", ColumnType::Json),
    ],
);

/// User preferences (key-value store)
pub static PREFERENCES_TABLE: Table = Table::new(
    "preferences",
    &[
        Column::new("key", ColumnType::Text).primary_key(),
        Column::new("value", ColumnType::Text),
        Column::new("updated_at", ColumnType::Timestamp).default("CURRENT_TIMESTAMP"),
    ],
);

/// Stored credentials (encrypted)
pub static CREDENTIALS_TABLE: Table = Table::new(
    "credentials",
    &[
        Column::new("id", ColumnType::Integer).primary_key(),
        Column::new("tool_id", ColumnType::Text).not_null(),
        Column::new("env_var", ColumnType::Text).not_null(),
        Column::new("encrypted_value", ColumnType::Text),
        Column::new("updated_at", ColumnType::Timestamp).default("CURRENT_TIMESTAMP"),
    ],
);

/// Session tracking
pub static SESSIONS_TABLE: Table = Table::new(
    "sessions",
    &[
        Column::new("id", ColumnType::Text).primary_key(),
        Column::new("tool_id", ColumnType::Text),
        Column::new("started_at", ColumnType::Timestamp).default("CURRENT_TIMESTAMP"),
        Column::new("ended_at", ColumnType::Timestamp),
        Column::new("state", ColumnType::Json),
    ],
);

/// Complete schema - all tables in migration order
pub struct Schema {
    pub tables: &'static [&'static Table],
    pub version: i32,
}

pub static SCHEMA: Schema = Schema {
    tables: &[
        &MIGRATIONS_TABLE,
        &TOOLS_TABLE,
        &TOOL_INSTALL_TABLE,
        &TOOL_UPDATE_TABLE,
        &TOOL_AUTH_TABLE,
        &TOOL_FEATURES_TABLE,
        &PREFERENCES_TABLE,
        &CREDENTIALS_TABLE,
        &SESSIONS_TABLE,
    ],
    version: 1,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_ddl_generation() {
        let col = Column::new("name", ColumnType::Text).not_null();
        assert_eq!(col.to_ddl(), "name TEXT NOT NULL");
    }

    #[test]
    fn test_column_with_default() {
        let col = Column::new("enabled", ColumnType::Boolean).default("1");
        assert_eq!(col.to_ddl(), "enabled INTEGER DEFAULT 1");
    }

    #[test]
    fn test_primary_key_column() {
        let col = Column::new("id", ColumnType::Text).primary_key();
        assert_eq!(col.to_ddl(), "id TEXT PRIMARY KEY");
    }

    #[test]
    fn test_table_create_sql() {
        let sql = PREFERENCES_TABLE.create_table_sql();
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS preferences"));
        assert!(sql.contains("key TEXT PRIMARY KEY"));
    }

    #[test]
    fn test_schema_has_all_tables() {
        assert!(SCHEMA.tables.len() >= 9);
    }

    #[test]
    fn test_table_column_names() {
        let names = TOOLS_TABLE.column_names();
        assert!(names.contains(&"id"));
        assert!(names.contains(&"display_name"));
        assert!(names.contains(&"cli_command"));
    }
}
