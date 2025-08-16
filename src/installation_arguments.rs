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
                args: vec!["install", "-g", "@qwen-code/qwen-code@latest"],
                description: "Qwen coding assistant",
                requires_npm: true,
            },
        );

        commands.insert(
            "opencode",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "opencode-ai@latest"],
                description: "OpenCode AI coding agent built for the terminal",
                requires_npm: true,
            },
        );

        commands.insert(
            "llxprt",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@vybestack/llxprt-code"],
                description:
                    "LLxprt Code - Multi-provider AI coding assistant with enhanced features",
                requires_npm: true,
            },
        );

        commands.insert(
            "codex",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@openai/codex"],
                description: "OpenAI Codex CLI - AI coding agent that runs locally",
                requires_npm: true,
            },
        );

        commands.insert(
            "crush",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@charmland/crush"],
                description: "Charm's multi-model AI coding assistant with LSP support",
                requires_npm: true,
            },
        );

        commands
    }
}
