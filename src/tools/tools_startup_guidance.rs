// Tools Startup Guidance Domain
// Handles tool-specific startup messages and user guidance
//
// Uses data-driven auth checks from AuthPreflight instead of
// hardcoded per-tool match arms.

use anyhow::Result;

/// Show minimal startup guidance for tools -- only when critical info is needed.
///
/// Reads auth requirements from TOML config via AuthPreflight rather than
/// maintaining per-tool match arms.
pub fn show_tool_startup_guidance(display_name: &str) -> Result<()> {
    use crate::auth_manager::auth_preflight::AuthPreflight;
    use crate::theme::theme_global_config;

    let result = AuthPreflight::check(display_name);

    // No guidance needed if auth is satisfied or tool needs no auth
    if result.is_ready || result.auth_mode == "none" {
        return Ok(());
    }

    let theme = theme_global_config::current_theme();

    // Build a concise tip from the preflight result
    if let Some(cmd) = &result.cli_auth_command {
        println!(
            "{}",
            theme.secondary(&format!(
                "Tip: Run '{}' to set up {} credentials",
                cmd, display_name
            ))
        );
    } else if !result.missing_vars.is_empty() {
        let hint = if result.auth_mode == "any" {
            // Show default or first missing var
            result
                .default_env_var
                .as_deref()
                .unwrap_or(&result.missing_vars[0])
                .to_string()
        } else {
            result.missing_vars.join(", ")
        };
        println!(
            "{}",
            theme.secondary(&format!("Tip: Set {} for {} access", hint, display_name))
        );
    }

    Ok(())
}
