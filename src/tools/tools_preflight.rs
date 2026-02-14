// Tools Preflight - Comprehensive pre-launch checks for AI tools
//
// Orchestrates prerequisite, installation, auth, and runtime checks
// into a single unified result used by the execution engine and dashboard.

use anyhow::Result;

use crate::auth_manager::auth_preflight::{AuthPreflight, AuthPreflightResult};

use super::tools_command_mapping::get_cli_command;
use super::tools_detection::{infer_package_manager, PackageManager};

/// Unified result of all pre-flight checks for a single tool.
#[derive(Debug, Clone)]
pub struct PreflightResult {
    pub tool_name: String,
    pub package_manager_ok: bool,
    pub package_manager_label: String,
    pub package_manager_message: Option<String>,
    pub is_installed: bool,
    pub install_message: Option<String>,
    pub tool_version: Option<String>,
    pub auth: AuthPreflightResult,
    pub runtime_ok: bool,
    pub runtime_message: Option<String>,
    pub blockers: Vec<String>,
    pub can_proceed: bool,
    pub can_fix: bool,
}

/// Zero-sized struct providing static preflight utilities.
pub struct ToolPreflight;

impl ToolPreflight {
    /// Run all pre-flight checks for a tool.
    pub fn check(tool_name: &str) -> PreflightResult {
        let (pm_ok, pm_label, pm_message, mut blockers) = Self::check_prerequisites(tool_name);
        let (installed, install_msg, version) = Self::check_installation(tool_name);
        let auth = AuthPreflight::check(tool_name);
        let (runtime_ok, runtime_msg) = if installed {
            Self::check_runtime(tool_name)
        } else {
            (false, Some("Tool not installed".to_string()))
        };

        // Collect blockers from each stage
        if !installed {
            if let Some(ref msg) = install_msg {
                blockers.push(msg.clone());
            }
        }
        if !auth.is_ready {
            blockers.push(format!(
                "Authentication not configured (missing: {})",
                auth.missing_vars.join(", ")
            ));
        }
        if installed && !runtime_ok {
            if let Some(ref msg) = runtime_msg {
                blockers.push(msg.clone());
            }
        }

        let can_proceed = pm_ok && installed && auth.is_ready && runtime_ok;
        // Terminal Jarvis can help fix package manager, installation, and auth issues
        let can_fix = !pm_ok || !installed || !auth.is_ready;

        PreflightResult {
            tool_name: tool_name.to_string(),
            package_manager_ok: pm_ok,
            package_manager_label: pm_label,
            package_manager_message: pm_message,
            is_installed: installed,
            install_message: install_msg,
            tool_version: version,
            auth,
            runtime_ok,
            runtime_message: runtime_msg,
            blockers,
            can_proceed,
            can_fix,
        }
    }

    /// Return Ok(PreflightResult) if the tool can proceed, or an error with suggestions.
    pub fn require_ready(tool_name: &str) -> Result<PreflightResult> {
        let result = Self::check(tool_name);
        if result.can_proceed {
            return Ok(result);
        }

        let mut lines: Vec<String> = vec![format!("Tool '{}' is not ready to launch:", tool_name)];

        for blocker in &result.blockers {
            lines.push(format!("  - {}", blocker));
        }

        lines.push(String::new());
        lines.push("Suggestions:".to_string());

        if !result.is_installed {
            lines.push(format!("  Run: terminal-jarvis install {}", tool_name));
        }
        if !result.auth.is_ready {
            let help = AuthPreflight::get_help_message(tool_name);
            for line in help.lines() {
                lines.push(format!("  {}", line));
            }
        }
        if !result.package_manager_ok {
            if let Some(ref msg) = result.package_manager_message {
                lines.push(format!("  {}", msg));
            }
        }

        Err(anyhow::anyhow!("{}", lines.join("\n")))
    }

    /// Check package manager prerequisites for a tool.
    /// Returns (ok, label, message, blockers).
    fn check_prerequisites(tool_name: &str) -> (bool, String, Option<String>, Vec<String>) {
        let pm = infer_package_manager(tool_name);
        let pm_ok = pm.is_available();
        let pm_label = pm.label().to_string();
        let mut blockers = Vec::new();
        let mut message = None;

        if !pm_ok {
            message = Some(format!(
                "{} is not available. {}",
                pm.label(),
                pm.install_hint()
            ));
            blockers.push(format!("Missing {}: {}", pm.label(), pm.install_hint()));
        }

        // For npm tools, check Node.js version >= 18
        if pm == PackageManager::Npm && pm_ok {
            if let Ok(output) = std::process::Command::new("node").arg("--version").output() {
                if output.status.success() {
                    let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    let major = ver
                        .trim_start_matches('v')
                        .split('.')
                        .next()
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                    if major < 18 {
                        blockers.push(format!("Node.js {} found, >= 18 required", ver));
                    }
                }
            }
        }

        (pm_ok, pm_label, message, blockers)
    }

    /// Check whether the tool binary is installed and attempt to read its version.
    /// Returns (installed, install_message, version).
    fn check_installation(tool_name: &str) -> (bool, Option<String>, Option<String>) {
        let cli_command = get_cli_command(tool_name);
        let installed = super::tools_detection::check_tool_installed(cli_command);
        let mut version = None;
        let message = if !installed {
            Some(format!("'{}' is not installed", tool_name))
        } else {
            // Try to get version
            if let Ok(output) = std::process::Command::new(cli_command)
                .arg("--version")
                .output()
            {
                if output.status.success() {
                    version = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
                }
            }
            None
        };
        (installed, message, version)
    }

    /// Verify the tool can actually execute (runtime sanity check).
    /// Returns (ok, message).
    fn check_runtime(tool_name: &str) -> (bool, Option<String>) {
        let cli_command = get_cli_command(tool_name);
        match std::process::Command::new(cli_command)
            .arg("--version")
            .output()
        {
            Ok(output) if output.status.success() => (true, None),
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                (
                    false,
                    Some(format!("Runtime check failed: {}", stderr.trim())),
                )
            }
            Err(e) => (
                false,
                Some(format!("Cannot execute '{}': {}", cli_command, e)),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_unknown_tool_returns_populated_result() {
        let result = ToolPreflight::check("nonexistent_tool_xyz_42");
        assert_eq!(result.tool_name, "nonexistent_tool_xyz_42");
        // Unknown tool has no known package manager
        assert!(result.package_manager_ok);
        // Tool is not installed
        assert!(!result.is_installed);
        assert!(result.install_message.is_some());
        // Cannot proceed without installation
        assert!(!result.can_proceed);
    }

    #[test]
    fn test_require_ready_unknown_tool_returns_error() {
        let result = ToolPreflight::require_ready("nonexistent_tool_xyz_42");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("not ready to launch"));
        assert!(err_msg.contains("terminal-jarvis install"));
    }

    #[test]
    fn test_check_result_has_blockers_for_missing_tool() {
        let result = ToolPreflight::check("nonexistent_tool_xyz_42");
        assert!(!result.blockers.is_empty());
    }

    #[test]
    fn test_check_can_fix_is_true_when_not_installed() {
        let result = ToolPreflight::check("nonexistent_tool_xyz_42");
        assert!(result.can_fix);
    }
}
