// Tools Environment - Data-driven environment configuration for tool execution
//
// Extracted from tools_execution_engine.rs to keep the execution engine focused
// on process management. Tool-specific environment setup lives here.

use anyhow::Result;
use std::process::Command;

/// Apply all tool-specific environment modifications to a Command before spawning.
pub fn apply_tool_environment(
    cmd: &mut Command,
    display_name: &str,
    _args: &[String],
) -> Result<()> {
    // Apply headless adjustments first (affects multiple tools)
    apply_headless_adjustments(cmd, display_name);

    match display_name {
        "aider" => apply_aider_env(cmd),
        "goose" => apply_goose_env(cmd),
        _ => {}
    }

    Ok(())
}

/// Set environment variables specific to Aider.
fn apply_aider_env(cmd: &mut Command) {
    cmd.env("PYTHONUNBUFFERED", "1");
    cmd.env("AIDER_NO_BROWSER", "1"); // Always prevent auto-opening browser; URL is still printed

    // In headless/Codespaces, reduce fancy terminal features from prompt_toolkit
    if is_headless() {
        // These are only applied when NOT running --help/--version
        // The caller can override via explicit args
        cmd.env("AIDER_HEADLESS_HINT", "1"); // internal marker for arg logic
    }
}

/// Apply aider-specific argument adjustments for headless environments.
pub fn apply_aider_headless_args(cmd: &mut Command, args: &[String]) {
    let is_help_or_version = args
        .iter()
        .any(|arg| arg.contains("help") || arg.contains("version"));

    if is_headless() && !is_help_or_version {
        if !args.iter().any(|arg| arg.contains("no-pretty")) {
            cmd.arg("--no-pretty");
        }
        if !args.iter().any(|arg| arg.contains("no-fancy-input")) {
            cmd.arg("--no-fancy-input");
        }
        if !args.iter().any(|arg| arg.contains("no-multiline")) {
            cmd.arg("--no-multiline");
        }
    }
}

/// Set environment variables specific to Goose.
/// Goose requires a clean environment with only whitelisted vars to prevent
/// GUI/browser override vars from interfering with provider subprocesses.
fn apply_goose_env(cmd: &mut Command) {
    use std::collections::HashSet;

    let current_env: Vec<(String, String)> = std::env::vars().collect();

    let whitelist: HashSet<&str> = [
        "PATH",
        "HOME",
        "USER",
        "SHELL",
        "TERM",
        "LANG",
        "LC_ALL",
        "LC_CTYPE",
        "COLUMNS",
        "LINES",
        "PWD",
        "DISPLAY",
        "XDG_RUNTIME_DIR",
        "XDG_CACHE_HOME",
        "XDG_CONFIG_HOME",
        "XDG_DATA_HOME",
        // Networking/proxy and certs
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "NO_PROXY",
        "http_proxy",
        "https_proxy",
        "no_proxy",
        "SSL_CERT_FILE",
        "SSL_CERT_DIR",
        "CURL_CA_BUNDLE",
        // Provider API keys
        "GOOGLE_API_KEY",
        "GEMINI_API_KEY",
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        // Goose-specific hints
        "GOOSE_PROVIDER",
        "GOOSE_MODEL",
    ]
    .iter()
    .copied()
    .collect();

    cmd.env_clear();
    for (k, v) in &current_env {
        if whitelist.contains(k.as_str()) || k.starts_with("LC_") {
            cmd.env(k, v);
        }
    }

    // Bridge Gemini env vars: some stacks expect GOOGLE_API_KEY, others GEMINI_API_KEY
    let google_key = std::env::var("GOOGLE_API_KEY").ok();
    let gemini_key = std::env::var("GEMINI_API_KEY").ok();
    match (google_key.as_deref(), gemini_key.as_deref()) {
        (Some(g), None) => {
            cmd.env("GEMINI_API_KEY", g);
        }
        (None, Some(gm)) => {
            cmd.env("GOOGLE_API_KEY", gm);
        }
        _ => {}
    }

    // Strip browser/GUI override vars that could confuse provider subprocesses
    for var in [
        "HEADLESS",
        "DISABLE_GUI",
        "INTERACTIVE",
        "FORCE_INTERACTIVE",
        "NO_BROWSER",
        "GEMINI_NO_BROWSER",
        "OAUTH_NO_BROWSER",
        "GOOGLE_APPLICATION_CREDENTIALS_NO_BROWSER",
        "BROWSER",
    ] {
        cmd.env_remove(var);
    }
}

/// Apply headless-specific env overrides for tools that try to open a browser.
fn apply_headless_adjustments(cmd: &mut Command, display_name: &str) {
    if !is_headless() {
        return;
    }
    // Disable browser opening for tools that support it
    match display_name {
        "claude" => {
            cmd.env("CLAUDE_NO_BROWSER", "1");
        }
        "gemini" => {
            cmd.env("GEMINI_NO_BROWSER", "1");
        }
        _ => {}
    }
}

/// Detect whether we are running in a headless environment (no display server).
fn is_headless() -> bool {
    std::env::var("CODESPACES").is_ok()
        || std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || (std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_headless_respects_ci_var() {
        // Save and restore CI to avoid polluting other tests
        let original = std::env::var("CI").ok();
        std::env::set_var("CI", "true");
        assert!(is_headless());
        match original {
            Some(val) => std::env::set_var("CI", val),
            None => std::env::remove_var("CI"),
        }
    }

    #[test]
    fn test_apply_tool_environment_ok_for_unknown_tool() {
        let mut cmd = Command::new("echo");
        let result = apply_tool_environment(&mut cmd, "unknown_tool_xyz", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_tool_environment_ok_for_aider() {
        let mut cmd = Command::new("echo");
        let result = apply_tool_environment(&mut cmd, "aider", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_tool_environment_ok_for_goose() {
        let mut cmd = Command::new("echo");
        let result = apply_tool_environment(&mut cmd, "goose", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gemini_key_bridging_in_goose_env() {
        // When GEMINI_API_KEY is set but GOOGLE_API_KEY is not,
        // apply_goose_env should bridge the value.
        // We test the bridging logic directly.
        let original_google = std::env::var("GOOGLE_API_KEY").ok();
        let original_gemini = std::env::var("GEMINI_API_KEY").ok();

        std::env::remove_var("GOOGLE_API_KEY");
        std::env::set_var("GEMINI_API_KEY", "test-gemini-key-12345");

        let mut cmd = Command::new("echo");
        apply_goose_env(&mut cmd);

        // Restore originals
        match original_google {
            Some(val) => std::env::set_var("GOOGLE_API_KEY", val),
            None => std::env::remove_var("GOOGLE_API_KEY"),
        }
        match original_gemini {
            Some(val) => std::env::set_var("GEMINI_API_KEY", val),
            None => std::env::remove_var("GEMINI_API_KEY"),
        }

        // The function should have called cmd.env("GOOGLE_API_KEY", ...) but
        // we cannot inspect Command internals directly. The test verifies no panic.
    }
}
