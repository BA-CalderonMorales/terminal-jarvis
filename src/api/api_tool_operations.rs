#![allow(dead_code)]

use crate::api::api_client::ApiClient;
use anyhow::Result;
use std::collections::HashMap;

/// API layer for tool-related operations
pub struct ToolApi {
    #[allow(dead_code)]
    client: ApiClient,
}

impl Default for ToolApi {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::new(),
        }
    }

    /// Get description for a tool
    pub async fn get_tool_description(&self, tool: &str) -> Result<String> {
        let descriptions = self.get_tool_descriptions().await?;
        Ok(descriptions
            .get(tool)
            .cloned()
            .unwrap_or_else(|| "No description available".to_string()))
    }

    /// Get all tool descriptions
    async fn get_tool_descriptions(&self) -> Result<HashMap<String, String>> {
        // For now, return static descriptions
        // In the future, this could fetch from an API
        let mut descriptions = HashMap::new();
        descriptions.insert(
            "claude-code".to_string(),
            "Anthropic's Claude for code assistance".to_string(),
        );
        descriptions.insert(
            "gemini-cli".to_string(),
            "Google's Gemini CLI tool".to_string(),
        );
        descriptions.insert("qwen-code".to_string(), "Qwen coding assistant".to_string());
        descriptions.insert(
            "opencode".to_string(),
            "Open-source coding tool".to_string(),
        );

        Ok(descriptions)
    }

    /// Check tool availability
    pub async fn check_tool_availability(&self, tool: &str) -> Result<bool> {
        // This could make an API call to check if a tool is available
        // For now, just check against supported tools
        let supported_tools = ["claude-code", "gemini-cli", "qwen-code", "opencode"];
        Ok(supported_tools.contains(&tool))
    }

    /// Get tool metadata
    pub async fn get_tool_metadata(&self, tool: &str) -> Result<ToolMetadata> {
        let descriptions = self.get_tool_descriptions().await?;
        let description = descriptions.get(tool).cloned().unwrap_or_default();

        Ok(ToolMetadata {
            name: tool.to_string(),
            description,
            version: "latest".to_string(),
            homepage: format!("https://github.com/example/{tool}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub homepage: String,
}
