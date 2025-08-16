// Configuration Entry Point - Main configuration interface and coordination
//
// This module provides the main configuration interfaces and coordinates
// between different configuration domains for a unified API.

// Re-export core structures
#[allow(unused_imports)]
pub use crate::config::config_manager::ConfigManager;
pub use crate::config::config_structures::{ApiConfig, Config, TemplateConfig, ToolConfig};
#[allow(unused_imports)]
pub use crate::config::config_version_cache::VersionCache;
