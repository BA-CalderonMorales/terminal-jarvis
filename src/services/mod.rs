// Services module - Domain-based architecture for service layer functionality
//
// This module coordinates package management and GitHub integration services
// using a domain-based architecture pattern for maintainability.

// Domain modules
mod services_entry_point;
mod services_github_integration;
mod services_npm_operations;
mod services_package_operations;
mod services_tool_configuration;

// Re-export main interfaces
pub use services_entry_point::{GitHubService, PackageService};
