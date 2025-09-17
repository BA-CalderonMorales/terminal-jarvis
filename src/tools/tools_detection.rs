// Tools Detection Domain
// Handles tool installation detection and status checking

use std::collections::BTreeMap;
use std::process::Command;

use super::tools_command_mapping::get_command_mapping;
use super::tools_config::get_tool_config_loader;

#[derive(Clone, Debug)]
pub struct ToolInfo {
    pub command: &'static str,
    pub is_installed: bool,
}

/// Get all available tools with their installation status and command
pub fn get_available_tools() -> BTreeMap<&'static str, ToolInfo> {
    let mut tools = BTreeMap::new();
    let mapping = get_command_mapping();
    let config_loader = get_tool_config_loader();

    // Get all tools from configuration (dynamic approach)
    let tool_names = config_loader.get_tool_names();

    // Insert tools with installation status
    for tool_name in tool_names {
        if let Some(cli_command) = mapping.get(tool_name.as_str()) {
            let is_installed = check_tool_installed(cli_command);

            // We need to use &'static str for the BTreeMap, so we'll use a different approach
            // For now, let's create a mapping that includes the new tools
            let static_name = match tool_name.as_str() {
                "claude" => "claude",
                "gemini" => "gemini",
                "qwen" => "qwen",
                "opencode" => "opencode",
                "llxprt" => "llxprt",
                "codex" => "codex",
                "crush" => "crush",
                "goose" => "goose",
                "amp" => "amp",
                "aider" => "aider",
                _ => continue, // Skip unknown tools
            };

            tools.insert(
                static_name,
                ToolInfo {
                    command: cli_command,
                    is_installed,
                },
            );
        }
    }

    tools
}

/// Check if a tool is installed by trying to run it
pub fn check_tool_installed(tool: &str) -> bool {
    // Try 'which' command first (Unix-like systems)
    if let Ok(output) = Command::new("which").arg(tool).output() {
        if output.status.success() && !output.stdout.is_empty() {
            return true;
        }
    }

    // Try 'where' command (Windows)
    if let Ok(output) = Command::new("where").arg(tool).output() {
        if output.status.success() && !output.stdout.is_empty() {
            return true;
        }
    }

    // For opencode specifically, check common installation paths
    if tool == "opencode" {
        let common_paths = [
            "/usr/local/bin/opencode",
            "/home/vscode/.local/bin/opencode",
            "/root/.local/bin/opencode",
            &format!(
                "{}/.local/bin/opencode",
                std::env::var("HOME").unwrap_or_default()
            ),
        ];

        for path in &common_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(output) = Command::new(path).arg("--version").output() {
                    if output.status.success() {
                        return true;
                    }
                }
            }
        }

        // Try with shell environment loaded
        if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("source ~/.bashrc 2>/dev/null; source ~/.profile 2>/dev/null; which opencode 2>/dev/null")
        .output()
      {
        if output.status.success() && !output.stdout.is_empty() {
          return true;
        }
      }
    }

    // Try running the tool with --version
    if let Ok(output) = Command::new(tool).arg("--version").output() {
        if output.status.success() {
            return true;
        }
    }

    // Try running the tool with --help as fallback
    if let Ok(output) = Command::new(tool).arg("--help").output() {
        if output.status.success() {
            return true;
        }
    }

    false
}

/// Get list of installed tools (display names)
pub fn get_installed_tools() -> Vec<&'static str> {
    let mapping = get_command_mapping();
    mapping
        .iter()
        .filter(|(_, cli_command)| check_tool_installed(cli_command))
        .map(|(display_name, _)| *display_name)
        .collect()
}

/// Get list of uninstalled tools (display names)
pub fn get_uninstalled_tools() -> Vec<&'static str> {
    let mapping = get_command_mapping();
    mapping
        .iter()
        .filter(|(_, cli_command)| !check_tool_installed(cli_command))
        .map(|(display_name, _)| *display_name)
        .collect()
}
