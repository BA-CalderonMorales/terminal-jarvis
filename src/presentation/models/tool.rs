//! Tool data models and structures
//!
//! Contains the core data structures for representing AI coding tools,
//! their configurations, and authentication requirements.

use crate::tools::tools_config::ToolDefinition;

/// Authentication configuration for a tool
#[derive(Debug, Clone, PartialEq)]
pub struct ToolAuth {
    pub env_vars: Vec<String>,
    pub setup_url: String,
    pub browser_auth: bool,
    pub auth_instructions: Option<String>,
}

/// Complete tool configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ToolConfig {
    pub homepage: String,
    pub documentation: String,
    pub features: Option<Vec<String>>,
    pub auth: ToolAuth,
}

/// Core tool representation
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub cli_command: String,
    pub config: ToolConfig,
    pub is_installed: bool,
    pub is_available: bool,
}

impl Tool {
    pub fn new(
        name: String,
        display_name: String,
        description: String,
        cli_command: String,
        config: ToolConfig,
        is_installed: bool,
        is_available: bool,
    ) -> Self {
        Self {
            name,
            display_name,
            description,
            cli_command,
            config,
            is_installed,
            is_available,
        }
    }

    pub fn from_tool_definition(
        name: String,
        definition: &ToolDefinition,
        is_installed: bool,
    ) -> Self {
        let config = ToolConfig {
            homepage: definition.homepage.clone(),
            documentation: definition.documentation.clone(),
            features: definition.features.as_ref().map(|f| {
                vec![
                    format!("Supports Files: {}", f.supports_files),
                    format!("Supports Streaming: {}", f.supports_streaming),
                    format!("Supports Conversation: {}", f.supports_conversation),
                ]
            }),
            auth: ToolAuth {
                env_vars: definition.auth.env_vars.clone(),
                setup_url: definition.auth.setup_url.clone(),
                browser_auth: definition.auth.browser_auth,
                auth_instructions: definition.auth.auth_instructions.clone(),
            },
        };

        Self::new(
            name.clone(),
            definition.display_name.clone(),
            definition.description.clone(),
            definition.cli_command.clone(),
            config,
            is_installed,
            true, // Assume available unless specified otherwise
        )
    }

    pub fn set_installed(&mut self, installed: bool) {
        self.is_installed = installed;
    }

    pub fn is_available(&self) -> bool {
        self.is_available
    }
}
