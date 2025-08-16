// Tools Command Mapping Domain
// Handles tool name resolution and command mapping

use std::collections::HashMap;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ToolCommand {
    pub command: &'static str,
    pub description: &'static str,
}

/// Map display names to actual CLI command names
pub fn get_command_mapping() -> HashMap<&'static str, &'static str> {
    let mut mapping = HashMap::new();
    // Map display names to actual CLI commands (no API key enforcement)
    mapping.insert("claude", "claude"); // Assuming claude-code installs as 'claude'
    mapping.insert("gemini", "gemini"); // Assuming gemini-cli installs as 'gemini'
    mapping.insert("qwen", "qwen"); // Assuming qwen-code installs as 'qwen'
    mapping.insert("opencode", "opencode"); // OpenCode installs as 'opencode'
    mapping.insert("llxprt", "llxprt"); // LLxprt Code installs as 'llxprt'
    mapping.insert("codex", "codex"); // OpenAI Codex CLI installs as 'codex'
    mapping.insert("crush", "crush"); // Crush installs as 'crush'
    mapping
}

/// Get actual CLI command from display name
pub fn get_cli_command(display_name: &str) -> &str {
    get_command_mapping()
        .get(display_name)
        .unwrap_or(&display_name)
}

/// Get all available tools as ToolCommand structs
#[allow(dead_code)]
pub fn get_all_tools() -> Vec<ToolCommand> {
    vec![
        ToolCommand {
            command: "claude",
            description: "Anthropic's Claude for code assistance",
        },
        ToolCommand {
            command: "gemini",
            description: "Google's Gemini CLI tool",
        },
        ToolCommand {
            command: "qwen",
            description: "Qwen coding assistant",
        },
        ToolCommand {
            command: "opencode",
            description: "OpenCode AI coding agent built for the terminal",
        },
        ToolCommand {
            command: "llxprt",
            description: "LLxprt Code - Multi-provider AI coding assistant with enhanced features",
        },
        ToolCommand {
            command: "codex",
            description: "OpenAI Codex CLI - AI coding agent that runs locally",
        },
        ToolCommand {
            command: "crush",
            description: "Charm's Crush - Multi-model AI coding assistant with LSP support",
        },
    ]
}

/// Get tool information by name
#[allow(dead_code)]
pub fn get_tool_info(tool_name: &str) -> Option<ToolCommand> {
    get_all_tools()
        .into_iter()
        .find(|tool| tool.command == tool_name)
}
