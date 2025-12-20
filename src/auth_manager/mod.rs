// Authentication Manager module - Domain-based architecture for authentication management
//
// This module provides authentication management functionality using a
// domain-based architecture pattern for maintainability and testability.

// Domain modules
mod auth_api_key_management;
pub mod auth_credentials_store;
mod auth_entry_point;
mod auth_environment_detection;
mod auth_environment_setup;
mod auth_warning_system;

// Re-export main interface
pub use auth_entry_point::AuthManager;
