// Database Module - Turso/libSQL based configuration storage
//
// Architecture (Domain-Based):
// - core/           # Shared infrastructure
//   - schema.rs       # Table/column definitions (single source of truth)
//   - query_builder.rs # Fluent API for SQL construction
//   - repository.rs    # Base repository pattern
//   - connection.rs    # Connection lifecycle
//   - migrations.rs    # Version-controlled schema changes
//
// - preferences/    # User preferences domain
// - tools/          # Tool configurations domain
// - credentials/    # Encrypted credentials domain (future)
// - sessions/       # Session tracking domain (future)
//
// Benefits:
// - No hardcoded SQL scattered across codebase
// - Schema changes in one place
// - Type-safe query construction
// - Easy to test and maintain
// - Clear domain boundaries

pub mod core;
pub mod preferences;
pub mod tools;

// Re-export commonly used items from core
pub use core::connection::DatabaseManager;
pub use core::query_builder::{QueryBuilder, QueryType};
pub use core::repository::BaseRepository;
pub use core::schema::{Column, ColumnType, Table, SCHEMA};

// Re-export domain repositories
pub use preferences::PreferencesRepository;
pub use tools::ToolsRepository;
