use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InstallCommand {
    pub command: &'static str,
    pub args: Vec<&'static str>,
    pub description: &'static str,
    pub requires_npm: bool,
}

pub struct InstallationManager;

impl InstallationManager {
    pub fn get_install_commands() -> HashMap<&'static str, InstallCommand> {
        let mut commands = HashMap::new();

        commands.insert(
            "claude",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@anthropic-ai/claude-code"],
                description: "Anthropic's Claude for code assistance",
                requires_npm: true,
            },
        );

        commands.insert(
            "gemini",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@google/gemini-cli"],
                description: "Google's Gemini CLI tool",
                requires_npm: true,
            },
        );

        commands.insert(
            "qwen",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@qwen-code/qwen-code"],
                description: "Qwen coding assistant",
                requires_npm: true,
            },
        );

        commands.insert(
            "opencode",
            InstallCommand {
                command: "sh",
                args: vec!["-c", "curl -fsSL https://opencode.ai/install | bash && export PATH=\"$HOME/.local/bin:$PATH\""],
                description: "OpenCode AI coding agent built for the terminal",
                requires_npm: false,
            },
        );

        commands
    }

    pub fn check_npm_available() -> bool {
        std::process::Command::new("npm")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn get_tool_names() -> Vec<&'static str> {
        Self::get_install_commands().keys().copied().collect()
    }

    pub fn get_install_command(tool: &str) -> Option<InstallCommand> {
        Self::get_install_commands().get(tool).cloned()
    }
}
