// Credential Entities
//
// Data structures for storing API keys and tokens securely.

use chrono::{DateTime, Utc};

/// Represents a stored credential (API key, token, etc.)
#[derive(Debug, Clone)]
pub struct Credential {
    pub id: Option<i64>,
    pub tool_id: String,
    pub env_var: String,
    pub encrypted_value: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Credential {
    /// Create a new credential builder
    pub fn builder(tool_id: &str, env_var: &str) -> CredentialBuilder {
        CredentialBuilder::new(tool_id, env_var)
    }

    /// Check if this credential has a value stored
    pub fn has_value(&self) -> bool {
        self.encrypted_value.is_some()
    }

    /// Get display-safe representation (masked value)
    pub fn masked_value(&self) -> String {
        match &self.encrypted_value {
            Some(v) if v.len() > 8 => {
                format!("{}...{}", &v[..4], &v[v.len() - 4..])
            }
            Some(_) => "****".to_string(),
            None => "(not set)".to_string(),
        }
    }
}

/// Builder for Credential entities
#[derive(Debug, Default)]
pub struct CredentialBuilder {
    tool_id: String,
    env_var: String,
    encrypted_value: Option<String>,
}

impl CredentialBuilder {
    pub fn new(tool_id: &str, env_var: &str) -> Self {
        Self {
            tool_id: tool_id.to_string(),
            env_var: env_var.to_string(),
            encrypted_value: None,
        }
    }

    /// Set the (encrypted) value
    pub fn value(mut self, value: &str) -> Self {
        self.encrypted_value = Some(value.to_string());
        self
    }

    /// Build the Credential
    pub fn build(self) -> Credential {
        Credential {
            id: None,
            tool_id: self.tool_id,
            env_var: self.env_var,
            encrypted_value: self.encrypted_value,
            updated_at: None,
        }
    }
}

/// Summary of authentication status for a tool
#[derive(Debug, Clone)]
pub struct ToolAuthStatus {
    pub tool_id: String,
    pub tool_name: String,
    pub required_vars: Vec<String>,
    pub configured_vars: Vec<String>,
    pub missing_vars: Vec<String>,
    pub is_complete: bool,
}

impl ToolAuthStatus {
    pub fn new(tool_id: &str, tool_name: &str, required: Vec<String>) -> Self {
        Self {
            tool_id: tool_id.to_string(),
            tool_name: tool_name.to_string(),
            required_vars: required,
            configured_vars: Vec::new(),
            missing_vars: Vec::new(),
            is_complete: false,
        }
    }

    /// Update status based on what's configured
    pub fn with_configured(mut self, configured: Vec<String>) -> Self {
        self.configured_vars = configured.clone();
        self.missing_vars = self
            .required_vars
            .iter()
            .filter(|v| !configured.contains(v))
            .cloned()
            .collect();
        self.is_complete = self.missing_vars.is_empty() && !self.required_vars.is_empty();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_builder() {
        let cred = Credential::builder("claude", "ANTHROPIC_API_KEY")
            .value("sk-ant-test123")
            .build();

        assert_eq!(cred.tool_id, "claude");
        assert_eq!(cred.env_var, "ANTHROPIC_API_KEY");
        assert!(cred.has_value());
    }

    #[test]
    fn test_masked_value() {
        let cred = Credential::builder("claude", "ANTHROPIC_API_KEY")
            .value("sk-ant-api03-abcdefghijk")
            .build();

        let masked = cred.masked_value();
        assert!(masked.contains("..."));
        assert!(masked.starts_with("sk-a"));
    }

    #[test]
    fn test_auth_status() {
        let status = ToolAuthStatus::new("claude", "Claude", vec!["ANTHROPIC_API_KEY".to_string()])
            .with_configured(vec!["ANTHROPIC_API_KEY".to_string()]);

        assert!(status.is_complete);
        assert!(status.missing_vars.is_empty());
    }

    #[test]
    fn test_auth_status_incomplete() {
        let status = ToolAuthStatus::new(
            "openai",
            "OpenAI",
            vec!["OPENAI_API_KEY".to_string(), "OPENAI_ORG_ID".to_string()],
        )
        .with_configured(vec!["OPENAI_API_KEY".to_string()]);

        assert!(!status.is_complete);
        assert_eq!(status.missing_vars, vec!["OPENAI_ORG_ID".to_string()]);
    }
}
