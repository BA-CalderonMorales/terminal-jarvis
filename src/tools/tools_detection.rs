// Tools Detection Domain
// Handles tool installation detection and status checking

use std::collections::BTreeMap;
use std::process::Command;

use super::tools_command_mapping::get_command_mapping;
use super::tools_config::get_tool_config_loader;

/// Package manager or runtime required to install/run a tool
#[derive(Clone, Debug, PartialEq)]
pub enum PackageManager {
    Npm,
    Uv,
    Cargo,
    Curl,
    Unknown,
}

impl PackageManager {
    /// Get a short label for display in menus
    pub fn label(&self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Uv => "uv",
            Self::Cargo => "cargo",
            Self::Curl => "curl",
            Self::Unknown => "",
        }
    }

    /// Check if this package manager is available on the system
    pub fn is_available(&self) -> bool {
        match self {
            Self::Npm => check_tool_installed("npm"),
            Self::Uv => check_tool_installed("uv"),
            Self::Cargo => check_tool_installed("cargo"),
            Self::Curl => check_tool_installed("curl"),
            Self::Unknown => true,
        }
    }

    /// Get installation hint for users who need this package manager
    pub fn install_hint(&self) -> &'static str {
        match self {
            Self::Npm => "Install Node.js from: https://nodejs.org/",
            Self::Uv => "Install uv from: https://docs.astral.sh/uv/",
            Self::Cargo => "Install Rust from: https://rustup.rs/",
            Self::Curl => "Install curl via your system package manager",
            Self::Unknown => "Check tool documentation for requirements",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ToolInfo {
    pub command: &'static str,
    pub is_installed: bool,
    pub package_manager: PackageManager,
}

/// Infer package manager from install command
pub fn infer_package_manager(tool_name: &str) -> PackageManager {
    let config_loader = get_tool_config_loader();
    let Some(install_cmd) = config_loader.get_install_command(tool_name) else {
        return PackageManager::Unknown;
    };

    match install_cmd.command.as_str() {
        "npm" => PackageManager::Npm,
        "uv" => PackageManager::Uv,
        "cargo" => PackageManager::Cargo,
        "curl" => PackageManager::Curl,
        _ => PackageManager::Unknown,
    }
}

/// Known tool names for static string mapping
const KNOWN_TOOLS: &[&str] = &[
    "aider",
    "amp",
    "claude",
    "codex",
    "copilot",
    "crush",
    "gemini",
    "goose",
    "llxprt",
    "opencode",
    "qwen",
    "ollama",
    "vibe",
    "droid",
    "forge",
    "cursor-agent",
    "jules",
    "kilocode",
    "letta",
    "nanocoder",
    "pi",
    "code",
    "eca",
];

/// Get all available tools with their installation status and command
pub fn get_available_tools() -> BTreeMap<&'static str, ToolInfo> {
    let mut tools = BTreeMap::new();
    let mapping = get_command_mapping();
    let config_loader = get_tool_config_loader();
    let tool_names = config_loader.get_tool_names();

    for tool_name in tool_names {
        let Some(cli_command) = mapping.get(tool_name.as_str()) else {
            continue;
        };

        let Some(static_name) = KNOWN_TOOLS.iter().find(|&&name| name == tool_name) else {
            continue;
        };

        let is_installed = check_tool_installed(cli_command);
        let package_manager = infer_package_manager(&tool_name);

        tools.insert(
            *static_name,
            ToolInfo {
                command: cli_command,
                is_installed,
                package_manager,
            },
        );
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
