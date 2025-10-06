//! Tool Card Component
//!
//! A component for displaying individual tool information in a card format.
//! Supports both detailed and compact rendering modes.

use crate::presentation::models::Tool;

/// Component for displaying tool information
#[derive(Debug, Clone)]
pub struct ToolCard {
    tool: Tool,
}

impl ToolCard {
    pub fn new(tool: Tool) -> Self {
        Self { tool }
    }

    pub fn render(&self) -> String {
        let status = if self.tool.is_installed {
            "Installed"
        } else {
            "Available"
        };

        format!(
            "Tool: {}\nStatus: {}\nDescription: {}\nCommand: {}\nHomepage: {}\nDocumentation: {}",
            self.tool.display_name,
            status,
            self.tool.description,
            self.tool.cli_command,
            self.tool.config.homepage,
            self.tool.config.documentation
        )
    }

    pub fn render_compact(&self) -> String {
        let status = if self.tool.is_installed { "✓" } else { "○" };

        format!(
            "{} {} - {}",
            status, self.tool.display_name, self.tool.description
        )
    }
}
