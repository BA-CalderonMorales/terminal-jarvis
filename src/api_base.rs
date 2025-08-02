/// Base API configuration and route definitions
pub struct ApiBase {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for ApiBase {
    fn default() -> Self {
        Self {
            base_url: "https://api.terminal-jarvis.dev".to_string(),
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}

impl ApiBase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

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
