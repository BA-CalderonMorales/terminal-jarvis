// Configuration module - Domain-based architecture for configuration management
//
// This module provides configuration management functionality using a
// domain-based architecture pattern for maintainability and testability.

// Domain modules
mod config_entry_point;
mod config_file_operations;
mod config_manager;
mod config_structures;
mod config_version_cache;

// Re-export main interfaces
#[allow(unused_imports)]
pub use config_entry_point::{ApiConfig, Config, TemplateConfig, ToolConfig};
pub use config_manager::ConfigManager;
pub use config_version_cache::VersionCache;
