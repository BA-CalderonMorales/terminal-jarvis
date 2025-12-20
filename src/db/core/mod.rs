// Database Core Module
//
// Shared infrastructure for all database operations:
// - Schema definitions
// - Query builder
// - Base repository pattern
// - Connection management
// - Migrations

pub mod connection;
pub mod migrations;
pub mod query_builder;
pub mod repository;
pub mod schema;
