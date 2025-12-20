// Tool Entities
//
// Data structures representing tools and related information.

use serde::{Deserialize, Serialize};

/// Tool entity - core tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub display_name: String,
    pub cli_command: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub requires_npm: bool,
    pub requires_sudo: bool,
    pub status: String,
    pub enabled: bool,
    pub auto_update: bool,
}

impl Tool {
    /// Create a new tool with required fields
    pub fn new(id: &str, display_name: &str, cli_command: &str) -> Self {
        Self {
            id: id.to_string(),
            display_name: display_name.to_string(),
            cli_command: cli_command.to_string(),
            description: None,
            homepage: None,
            documentation: None,
            requires_npm: false,
            requires_sudo: false,
            status: "stable".to_string(),
            enabled: true,
            auto_update: true,
        }
    }

    /// Builder: set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Builder: set homepage
    pub fn with_homepage(mut self, url: &str) -> Self {
        self.homepage = Some(url.to_string());
        self
    }

    /// Builder: mark as requiring npm
    pub fn requires_npm(mut self) -> Self {
        self.requires_npm = true;
        self
    }
}

/// Tool installation info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInstall {
    pub tool_id: String,
    pub command: String,
    pub args: Vec<String>,
    pub verify_command: Option<String>,
    pub post_install_message: Option<String>,
}

impl ToolInstall {
    /// Create new install info
    pub fn new(tool_id: &str, command: &str, args: Vec<String>) -> Self {
        Self {
            tool_id: tool_id.to_string(),
            command: command.to_string(),
            args,
            verify_command: None,
            post_install_message: None,
        }
    }

    /// Builder: set verify command
    pub fn with_verify(mut self, cmd: &str) -> Self {
        self.verify_command = Some(cmd.to_string());
        self
    }

    /// Builder: set post-install message
    pub fn with_message(mut self, msg: &str) -> Self {
        self.post_install_message = Some(msg.to_string());
        self
    }
}

/// Tool authentication info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolAuth {
    pub tool_id: String,
    pub env_vars: Vec<String>,
    pub setup_url: Option<String>,
    pub browser_auth: bool,
    pub auth_instructions: Option<String>,
}

impl ToolAuth {
    /// Create new auth info
    pub fn new(tool_id: &str, env_vars: Vec<String>) -> Self {
        Self {
            tool_id: tool_id.to_string(),
            env_vars,
            setup_url: None,
            browser_auth: false,
            auth_instructions: None,
        }
    }

    /// Builder: set setup URL
    pub fn with_setup_url(mut self, url: &str) -> Self {
        self.setup_url = Some(url.to_string());
        self
    }

    /// Builder: enable browser auth
    pub fn with_browser_auth(mut self) -> Self {
        self.browser_auth = true;
        self
    }

    /// Builder: set auth instructions
    pub fn with_instructions(mut self, instructions: &str) -> Self {
        self.auth_instructions = Some(instructions.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_builder() {
        let tool = Tool::new("claude", "Claude", "claude")
            .with_description("AI assistant")
            .with_homepage("https://claude.ai")
            .requires_npm();

        assert_eq!(tool.id, "claude");
        assert_eq!(tool.display_name, "Claude");
        assert!(tool.requires_npm);
        assert!(tool.description.is_some());
    }

    #[test]
    fn test_tool_install_builder() {
        let install = ToolInstall::new("claude", "npm", vec!["install".into(), "-g".into()])
            .with_verify("claude --version")
            .with_message("Claude installed!");

        assert_eq!(install.tool_id, "claude");
        assert!(install.verify_command.is_some());
    }

    #[test]
    fn test_tool_auth_builder() {
        let auth = ToolAuth::new("claude", vec!["ANTHROPIC_API_KEY".into()])
            .with_setup_url("https://console.anthropic.com")
            .with_browser_auth();

        assert!(auth.browser_auth);
        assert!(auth.setup_url.is_some());
    }
}
