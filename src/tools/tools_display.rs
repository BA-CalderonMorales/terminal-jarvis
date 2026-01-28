//! Tool Information Display System
//!
//! This module provides a unified, reusable system for displaying tool information
//! across all Terminal Jarvis commands. It ensures consistent formatting and theming
//! for tool details whether displayed in info, list, or other contexts.
//!
//! Key Features:
//! - Consistent separator formatting using simple "===" lines
//! - Unified status indicators with proper theming
//! - Multiple display modes for different contexts (detailed, compact, inline)
//! - Centralized NPM requirement and installation status handling
//! - Easy to extend for new tools without duplicating display logic

use crate::installation_arguments::{InstallCommand, InstallationManager};
use crate::theme::theme_global_config;
use crate::tools::tools_detection::PackageManager;
use crate::tools::ToolInfo;

/// Display modes for tool information
#[derive(Clone)]
#[allow(dead_code)] // Allow future display modes even if not yet used
pub enum ToolDisplayMode {
    /// Full detailed view with separator lines
    Detailed,
    /// Compact list view for multiple tools
    Compact,
    /// Simple inline format
    Inline,
}

/// Unified tool information display system
pub struct ToolDisplayFormatter;

impl ToolDisplayFormatter {
    /// Display a single tool's information in the specified format
    pub fn display_tool_info(
        tool_name: &str,
        tool_info: &ToolInfo,
        install_info: &InstallCommand,
        mode: ToolDisplayMode,
    ) {
        match mode {
            ToolDisplayMode::Detailed => Self::display_detailed(tool_name, tool_info, install_info),
            ToolDisplayMode::Compact => Self::display_compact(tool_name, tool_info, install_info),
            ToolDisplayMode::Inline => Self::display_inline(tool_name, tool_info, install_info),
        }
    }

    /// Display multiple tools in a consistent list format
    pub fn display_tool_list<'a, I>(tools_iter: I)
    where
        I: Iterator<Item = (&'a str, &'a ToolInfo, &'a InstallCommand)>,
    {
        println!("Available AI Coding Tools:\n");

        for (tool_name, tool_info, install_info) in tools_iter {
            Self::display_compact(tool_name, tool_info, install_info);
            println!(); // Add spacing between tools
        }
    }

    /// Detailed format with separator lines (for `info` command)
    fn display_detailed(tool_name: &str, tool_info: &ToolInfo, install_info: &InstallCommand) {
        let theme = theme_global_config::current_theme();

        println!();
        println!(
            "{}",
            theme.primary(&format!("=== Tool Information: {tool_name} ==="))
        );
        println!();

        println!(
            "{}",
            theme.secondary(&format!("Description: {}", install_info.description))
        );
        println!(
            "{}",
            theme.secondary(&format!("Command: {}", tool_info.command))
        );

        let status_text = Self::format_installation_status(tool_info.is_installed);
        println!("Status: {status_text}");

        println!(
            "{}",
            theme.secondary(&format!(
                "Installation: {} {}",
                install_info.command,
                install_info.args.join(" ")
            ))
        );

        if install_info.requires_npm {
            let npm_status = Self::format_npm_status();
            println!("NPM Required: {npm_status}");
        }

        // Show authentication info
        Self::display_auth_info(tool_name);

        println!();
        println!("{}", theme.primary("==================================="));
        println!();
    }

    /// Display authentication information for a tool
    fn display_auth_info(tool_name: &str) {
        use crate::tools::tools_config::get_tool_config_loader;

        let theme = theme_global_config::current_theme();
        let loader = get_tool_config_loader();

        if let Some(auth) = loader.get_auth_info(tool_name) {
            println!();
            println!("{}", theme.secondary("Authentication:"));

            // Check which env vars are set
            let mut has_any_key = false;
            for var in &auth.env_vars {
                let is_set = std::env::var(var).is_ok();
                if is_set {
                    has_any_key = true;
                }
                let status = if is_set {
                    theme.primary("[SET]")
                } else {
                    theme.accent("[NOT SET]")
                };
                println!("  {var} {status}");
            }

            if !has_any_key && !auth.setup_url.is_empty() {
                println!("{}", theme.accent(&format!("  Setup: {}", auth.setup_url)));
            }
        }
    }

    /// Compact format for list views
    fn display_compact(tool_name: &str, tool_info: &ToolInfo, install_info: &InstallCommand) {
        let status_text = if tool_info.is_installed {
            "Installed"
        } else {
            "Not installed"
        };

        println!(" {} - {}", tool_name, install_info.description);
        println!("  Status: {status_text}");
        println!("  Command: {}", tool_info.command);
        if install_info.requires_npm {
            println!("  Requires: NPM");
        }
    }

    /// Inline format for brief mentions
    fn display_inline(tool_name: &str, tool_info: &ToolInfo, install_info: &InstallCommand) {
        let status = if tool_info.is_installed { "✓" } else { "✗" };
        println!("{} [{}] - {}", tool_name, status, install_info.description);
    }

    /// Format installation status with consistent theming
    fn format_installation_status(is_installed: bool) -> String {
        let theme = theme_global_config::current_theme();
        if is_installed {
            theme.primary("Installed ✓")
        } else {
            theme.accent("Not installed ✗")
        }
    }

    /// Format NPM availability status with consistent theming
    fn format_npm_status() -> String {
        let theme = theme_global_config::current_theme();
        if InstallationManager::check_npm_available() {
            theme.primary("Available ✓")
        } else {
            theme.accent("Not available ✗")
        }
    }

    /// Display system requirements advisory (for list command)
    pub fn show_system_requirements_advisory() {
        if !InstallationManager::check_npm_available() {
            let theme = theme_global_config::current_theme();
            println!(
                "{} {}",
                theme.secondary("⚠ ADVISORY:"),
                theme.primary("Node.js ecosystem not detected")
            );
            println!("  Most AI tools are distributed via NPM. Install from: https://nodejs.org/");
        }
    }

    /// Format a tool name with requirement hint for menu display
    ///
    /// Returns a formatted string like "claude [npm]" or "aider [uv]"
    /// with visual indicators if the required package manager is missing.
    pub fn format_menu_item(tool_name: &str, tool_info: &ToolInfo) -> String {
        let label = tool_info.package_manager.label();
        if label.is_empty() {
            return tool_name.to_string();
        }

        let is_available = tool_info.package_manager.is_available();
        let hint = if is_available {
            format!("[{}]", label)
        } else {
            format!("[{} ⚠]", label)
        };

        // Pad tool name to align hints
        let padded_name = format!("{:<10}", tool_name);
        format!("{} {}", padded_name, hint)
    }

    /// Get a summary of missing package managers for the tool set
    pub fn get_missing_requirements(tools: &[(String, ToolInfo)]) -> Vec<(PackageManager, String)> {
        let all_package_managers = [
            PackageManager::Npm,
            PackageManager::Uv,
            PackageManager::Cargo,
            PackageManager::Curl,
        ];

        all_package_managers
            .into_iter()
            .filter(|pm| {
                let is_needed = tools.iter().any(|(_, t)| t.package_manager == *pm);
                is_needed && !pm.is_available()
            })
            .map(|pm| {
                let msg = format!(
                    "{} required for some tools. {}",
                    pm.label(),
                    pm.install_hint()
                );
                (pm, msg)
            })
            .collect()
    }
}
