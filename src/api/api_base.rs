#![allow(dead_code)]

//! Base API configuration and URL management
//!
//! **STATUS: FUTURE-USE MODULE** - Foundation for remote service integration
//!
//! This module provides foundational API configuration that will be activated when
//! Terminal Jarvis connects to remote services for tool discovery and metadata.
//!
//! # Purpose
//!
//! The [`ApiBase`] struct centralizes all HTTP client configuration, including:
//! - Base URL management for different environments
//! - Timeout and retry logic configuration
//! - Connection pooling settings
//! - Authentication endpoint configuration
//!
//! # Future Usage Example
//!
//! ```rust,ignore
//! let api_config = ApiBase::new()
//!     .with_base_url("https://api.terminal-jarvis.dev")
//!     .with_timeout(60)
//!     .with_max_retries(5);
//!
//! let client = ApiClient::with_config(api_config);
//! ```

/// Default base URL for Terminal Jarvis API services
///
/// This constant ensures consistency across all API-related modules and
/// provides a single source of truth for the base URL configuration.
const DEFAULT_API_BASE_URL: &str = "https://api.terminal-jarvis.dev";

/// Base configuration for API clients
///
/// Centralizes HTTP client settings including base URL, timeouts, and retry logic.
/// Designed to support different environments (dev, staging, prod) and provide
/// consistent configuration across all API operations.
///
/// # Fields
///
/// * `base_url` - The root URL for all API endpoints
/// * `timeout_seconds` - Request timeout in seconds
/// * `max_retries` - Maximum number of retry attempts for failed requests
pub struct ApiBase {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for ApiBase {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_API_BASE_URL.to_string(),
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}

impl ApiBase {
    /// Creates a new ApiBase with default configuration
    ///
    /// # Returns
    ///
    /// An ApiBase instance configured with:
    /// - Base URL: `https://api.terminal-jarvis.dev`
    /// - Timeout: 30 seconds
    /// - Max retries: 3 attempts
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates ApiBase with custom base URL
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for API requests
    ///
    /// # Returns
    ///
    /// An ApiBase instance with the specified base URL and default settings
    #[allow(dead_code)]
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    /// Sets request timeout in seconds
    ///
    /// # Arguments
    ///
    /// * `timeout_seconds` - Timeout duration in seconds
    ///
    /// # Returns
    ///
    /// Self for method chaining
    #[allow(dead_code)]
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    /// Sets maximum retry attempts
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of retry attempts
    ///
    /// # Returns
    ///
    /// Self for method chaining
    #[allow(dead_code)]
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Get the full URL for an endpoint
    pub fn endpoint_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

/// API route definitions
pub mod routes {
    pub const TOOLS: &str = "/api/v1/tools";
    pub const TOOL_INFO: &str = "/api/v1/tools/{tool}";
    pub const TOOL_VERSIONS: &str = "/api/v1/tools/{tool}/versions";
    pub const TEMPLATES: &str = "/api/v1/templates";
    pub const TEMPLATE_INFO: &str = "/api/v1/templates/{template}";
}
