use crate::auth_manager::AuthManager;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ToolCommand {
    pub command: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Debug)]
pub struct ToolInfo {
    pub command: &'static str,
    pub is_installed: bool,
}

pub struct ToolManager;

impl ToolManager {
    /// Map display names to actual CLI command names
    fn get_command_mapping() -> HashMap<&'static str, &'static str> {
        let mut mapping = HashMap::new();
        // Map display names to actual CLI commands (no API key enforcement)
        mapping.insert("claude", "claude"); // Assuming claude-code installs as 'claude'
        mapping.insert("gemini", "gemini"); // Assuming gemini-cli installs as 'gemini'
        mapping.insert("qwen", "qwen"); // Assuming qwen-code installs as 'qwen'
        mapping.insert("opencode", "opencode"); // OpenCode installs as 'opencode'
        mapping.insert("llxprt", "llxprt"); // LLxprt Code installs as 'llxprt'
        mapping
    }

    /// Get actual CLI command from display name
    pub fn get_cli_command(display_name: &str) -> &str {
        Self::get_command_mapping()
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
                description:
                    "LLxprt Code - Multi-provider AI coding assistant with enhanced features",
            },
        ]
    }

    /// Get all available tools with their installation status
    pub fn get_available_tools() -> HashMap<&'static str, ToolInfo> {
        let mut tools = HashMap::new();
        let mapping = Self::get_command_mapping();

        // Use display names (keys from mapping) for consistency with InstallationManager
        for (display_name, cli_command) in mapping {
            let is_installed = Self::check_tool_installed(cli_command);
            tools.insert(
                display_name,
                ToolInfo {
                    command: cli_command,
                    is_installed,
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

    /// Run a tool with arguments
    pub async fn run_tool(display_name: &str, args: &[String]) -> Result<()> {
        let cli_command = Self::get_cli_command(display_name);

        if !Self::check_tool_installed(cli_command) {
            return Err(anyhow::anyhow!(
                "Tool '{}' is not installed. Use 'terminal-jarvis install {}' to install it.",
                display_name,
                display_name
            ));
        }

        // Prepare authentication-safe environment and warn about browser opening
        AuthManager::prepare_auth_safe_environment()?;
        AuthManager::warn_if_browser_likely(display_name)?;

        // Special terminal preparation for opencode to ensure proper input focus
        if display_name == "opencode" {
            Self::prepare_opencode_terminal_state()?;
        } else {
            // Clear any remaining progress indicators and ensure clean terminal state for other tools
            print!("\x1b[2K\r"); // Clear current line
            print!("\x1b[?25h"); // Show cursor
            std::io::stdout().flush().unwrap_or_default();
        }

        let mut cmd = Command::new(cli_command);

        // Special handling for opencode which has different command structure
        if display_name == "opencode" {
            if args.is_empty() {
                // No arguments - start TUI mode in current directory
                cmd.arg(".");
            } else {
                // Arguments provided - use 'run' subcommand
                cmd.arg("run");
                cmd.args(args);
            }
        } else if display_name == "llxprt" {
            // For llxprt, when no arguments are provided, it opens the interactive TUI
            // This is expected behavior and should work seamlessly
            cmd.args(args);
        } else {
            // For other tools, pass arguments directly
            cmd.args(args);
        }

        // For interactive tools, we MUST inherit all stdio streams
        // This is critical for tools like claude-code that use Ink/React components
        cmd.stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        let status = cmd
            .status()
            .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", cli_command, e))?;

        // Restore environment after tool execution
        AuthManager::restore_environment()?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Tool '{}' exited with error code: {:?}",
                display_name,
                status.code()
            ));
        }

        Ok(())
    }

    /// Prepare terminal state specifically for opencode to ensure proper input focus
    fn prepare_opencode_terminal_state() -> Result<()> {
        use std::io::Write;

        // For opencode, we need a very careful terminal preparation sequence
        // to ensure the input box gets proper focus on fresh installs
        // Use only the most essential terminal sequences to avoid strange output

        // 1. Just clear the screen and ensure cursor is visible - minimal approach
        print!("\x1b[H\x1b[2J"); // Home cursor + clear screen (combined)
        std::io::stdout().flush()?;

        // 2. Brief delay to let opencode initialize properly
        std::thread::sleep(std::time::Duration::from_millis(75));

        Ok(())
    }

    /// Get list of installed tools (display names)
    pub fn get_installed_tools() -> Vec<&'static str> {
        let mapping = Self::get_command_mapping();
        mapping
            .iter()
            .filter(|(_, cli_command)| Self::check_tool_installed(cli_command))
            .map(|(display_name, _)| *display_name)
            .collect()
    }

    /// Get list of uninstalled tools (display names)
    pub fn get_uninstalled_tools() -> Vec<&'static str> {
        let mapping = Self::get_command_mapping();
        mapping
            .iter()
            .filter(|(_, cli_command)| !Self::check_tool_installed(cli_command))
            .map(|(display_name, _)| *display_name)
            .collect()
    }

    /// Get tool information by name
    #[allow(dead_code)]
    pub fn get_tool_info(tool_name: &str) -> Option<ToolCommand> {
        Self::get_all_tools()
            .into_iter()
            .find(|tool| tool.command == tool_name)
    }
}
