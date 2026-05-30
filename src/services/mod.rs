// Services module - Domain-based architecture for service layer functionality
//
// This module coordinates package management and GitHub integration services
// using a domain-based architecture pattern for maintainability.

// Domain modules
mod services_entry_point;
mod services_github_integration;
mod services_npm_operations;

// Re-export main interfaces
pub use services_entry_point::{GitHubService, PackageService};
