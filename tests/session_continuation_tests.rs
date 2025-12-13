// Session Continuation Tests
// Validates the core session continuation logic that prevents users from being
// kicked out during authentication workflows - THE defining feature of Terminal Jarvis

use std::time::Duration;

// Helper function to create test args
fn args(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

// Import the session continuation function for testing
// This tests the actual implementation logic
use terminal_jarvis::tools::tools_execution_engine::should_continue_session;

#[cfg(test)]
mod session_continuation_logic_tests {
    use super::*;

    // ============================================================
    // EXIT COMMAND TESTS - These should NEVER continue sessions
    // ============================================================

    #[test]
    fn test_exit_command_slash_exit_does_not_continue() {
        // /exit is the most common way to exit AI tools
        let result = should_continue_session("claude", &args(&["/exit"]), Duration::from_secs(1));
        assert!(!result, "/exit command should never continue the session");
    }

    #[test]
    fn test_exit_command_slash_quit_does_not_continue() {
        let result = should_continue_session("gemini", &args(&["/quit"]), Duration::from_secs(1));
        assert!(!result, "/quit command should never continue the session");
    }

    #[test]
    fn test_exit_command_slash_bye_does_not_continue() {
        let result = should_continue_session("qwen", &args(&["/bye"]), Duration::from_secs(1));
        assert!(!result, "/bye command should never continue the session");
    }

    #[test]
    fn test_exit_command_double_dash_exit_does_not_continue() {
        let result =
            should_continue_session("opencode", &args(&["--exit"]), Duration::from_secs(1));
        assert!(!result, "--exit command should never continue the session");
    }

    #[test]
    fn test_exit_command_double_dash_quit_does_not_continue() {
        let result = should_continue_session("codex", &args(&["--quit"]), Duration::from_secs(1));
        assert!(!result, "--quit command should never continue the session");
    }

    #[test]
    fn test_exit_command_bare_exit_does_not_continue() {
        let result = should_continue_session("aider", &args(&["exit"]), Duration::from_secs(1));
        assert!(
            !result,
            "bare 'exit' command should never continue the session"
        );
    }

    #[test]
    fn test_exit_command_bare_quit_does_not_continue() {
        let result = should_continue_session("goose", &args(&["quit"]), Duration::from_secs(1));
        assert!(
            !result,
            "bare 'quit' command should never continue the session"
        );
    }

    #[test]
    fn test_exit_command_bare_bye_does_not_continue() {
        let result = should_continue_session("llxprt", &args(&["bye"]), Duration::from_secs(1));
        assert!(
            !result,
            "bare 'bye' command should never continue the session"
        );
    }

    // ============================================================
    // AUTH/SETUP COMMANDS - These SHOULD continue sessions
    // ============================================================

    #[test]
    fn test_auth_command_slash_auth_continues_session() {
        let result = should_continue_session("claude", &args(&["/auth"]), Duration::from_secs(1));
        assert!(
            result,
            "/auth command should continue the session for re-authentication"
        );
    }

    #[test]
    fn test_auth_command_slash_login_continues_session() {
        let result = should_continue_session("gemini", &args(&["/login"]), Duration::from_secs(1));
        assert!(result, "/login command should continue the session");
    }

    #[test]
    fn test_auth_command_double_dash_auth_continues_session() {
        let result = should_continue_session("qwen", &args(&["--auth"]), Duration::from_secs(1));
        assert!(result, "--auth command should continue the session");
    }

    #[test]
    fn test_auth_command_double_dash_login_continues_session() {
        let result =
            should_continue_session("opencode", &args(&["--login"]), Duration::from_secs(1));
        assert!(result, "--login command should continue the session");
    }

    #[test]
    fn test_setup_command_slash_setup_continues_session() {
        let result = should_continue_session("codex", &args(&["/setup"]), Duration::from_secs(1));
        assert!(result, "/setup command should continue the session");
    }

    #[test]
    fn test_setup_command_double_dash_setup_continues_session() {
        let result = should_continue_session("aider", &args(&["--setup"]), Duration::from_secs(1));
        assert!(result, "--setup command should continue the session");
    }

    #[test]
    fn test_config_command_slash_config_continues_session() {
        let result = should_continue_session("goose", &args(&["/config"]), Duration::from_secs(1));
        assert!(result, "/config command should continue the session");
    }

    #[test]
    fn test_config_command_double_dash_config_continues_session() {
        let result =
            should_continue_session("llxprt", &args(&["--config"]), Duration::from_secs(1));
        assert!(result, "--config command should continue the session");
    }

    #[test]
    fn test_authenticate_substring_continues_session() {
        let result =
            should_continue_session("amp", &args(&["authenticate-user"]), Duration::from_secs(1));
        assert!(
            result,
            "Commands containing 'authenticate' should continue the session"
        );
    }

    #[test]
    fn test_oauth_substring_continues_session() {
        let result =
            should_continue_session("crush", &args(&["oauth-login"]), Duration::from_secs(1));
        assert!(
            result,
            "Commands containing 'oauth' should continue the session"
        );
    }

    // ============================================================
    // NORMAL OPERATIONS - These should NOT continue sessions
    // ============================================================

    #[test]
    fn test_empty_args_does_not_continue() {
        let result = should_continue_session("claude", &[], Duration::from_secs(5));
        assert!(
            !result,
            "Empty args (normal tool exit) should not continue the session"
        );
    }

    #[test]
    fn test_regular_command_does_not_continue() {
        let result = should_continue_session("gemini", &args(&["--help"]), Duration::from_secs(1));
        assert!(
            !result,
            "Regular --help command should not continue the session"
        );
    }

    #[test]
    fn test_file_argument_does_not_continue() {
        let result = should_continue_session("qwen", &args(&["file.txt"]), Duration::from_secs(1));
        assert!(!result, "File arguments should not continue the session");
    }

    #[test]
    fn test_version_command_does_not_continue() {
        let result =
            should_continue_session("opencode", &args(&["--version"]), Duration::from_secs(1));
        assert!(!result, "--version command should not continue the session");
    }

    // ============================================================
    // EDGE CASES AND MIXED SCENARIOS
    // ============================================================

    #[test]
    fn test_exit_takes_precedence_over_auth_in_same_args() {
        // If both exit and auth are in args, exit should take precedence
        let result =
            should_continue_session("claude", &args(&["/auth", "/exit"]), Duration::from_secs(1));
        assert!(
            !result,
            "Exit command should take precedence over auth command"
        );
    }

    #[test]
    fn test_multiple_exit_commands_does_not_continue() {
        let result = should_continue_session(
            "gemini",
            &args(&["/exit", "/quit", "/bye"]),
            Duration::from_secs(1),
        );
        assert!(
            !result,
            "Multiple exit commands should not continue the session"
        );
    }

    #[test]
    fn test_multiple_auth_commands_continues() {
        let result =
            should_continue_session("qwen", &args(&["/auth", "/login"]), Duration::from_secs(1));
        assert!(result, "Multiple auth commands should continue the session");
    }

    #[test]
    fn test_short_execution_time_without_auth_does_not_continue() {
        // Even with very short execution time, normal commands don't continue
        let result = should_continue_session(
            "claude",
            &args(&["some-command"]),
            Duration::from_millis(100),
        );
        assert!(
            !result,
            "Short execution time alone should not trigger continuation"
        );
    }

    #[test]
    fn test_long_execution_time_without_auth_does_not_continue() {
        // Long execution time without auth commands should not continue
        let result = should_continue_session(
            "gemini",
            &args(&["work", "on", "files"]),
            Duration::from_secs(300),
        );
        assert!(
            !result,
            "Long execution time alone should not trigger continuation"
        );
    }

    #[test]
    fn test_works_for_all_supported_tools() {
        // Verify behavior works consistently across all supported tools
        let tools = [
            "claude", "gemini", "qwen", "opencode", "codex", "aider", "goose", "llxprt", "amp",
            "crush",
        ];

        for tool in &tools {
            // Exit should not continue for any tool
            let exit_result =
                should_continue_session(tool, &args(&["/exit"]), Duration::from_secs(1));
            assert!(!exit_result, "Exit should not continue session for {tool}");

            // Auth should continue for any tool
            let auth_result =
                should_continue_session(tool, &args(&["/auth"]), Duration::from_secs(1));
            assert!(auth_result, "Auth should continue session for {tool}");
        }
    }

    // ============================================================
    // CASE SENSITIVITY TESTS
    // ============================================================

    #[test]
    fn test_exit_commands_are_case_sensitive() {
        // Current implementation is case-sensitive - "/EXIT" is not recognized
        let result = should_continue_session("claude", &args(&["/EXIT"]), Duration::from_secs(1));
        // Based on current implementation, uppercase is NOT recognized as exit
        // This is intentional to avoid false positives
        assert!(
            !result,
            "Uppercase /EXIT should not continue (treated as unknown command)"
        );
    }

    #[test]
    fn test_authenticate_substring_case_insensitive_check() {
        // The contains() check is case-sensitive
        let lower_result =
            should_continue_session("claude", &args(&["authenticate"]), Duration::from_secs(1));
        let upper_result =
            should_continue_session("claude", &args(&["AUTHENTICATE"]), Duration::from_secs(1));

        assert!(
            lower_result,
            "Lowercase 'authenticate' should continue session"
        );
        // contains() is case-sensitive, so AUTHENTICATE won't match
        assert!(
            !upper_result,
            "Uppercase 'AUTHENTICATE' is not matched by contains()"
        );
    }
}

// ============================================================
// DOCUMENTATION TESTS
// ============================================================

/// Test that demonstrates the session continuation behavior
/// This serves as living documentation for the feature
#[test]
fn test_session_continuation_documentation() {
    // The session continuation system is THE defining feature of Terminal Jarvis.
    // It prevents users from being kicked out during authentication workflows.

    // When a user runs an auth command like /auth or /login, the tool exits
    // to complete authentication, but Terminal Jarvis automatically restarts
    // the tool so the user can continue working.

    // When a user explicitly exits with /exit, /quit, or /bye, the session
    // does NOT continue - respecting the user's intent to exit.

    // This test validates that the core logic works as documented.

    let auth_continues =
        should_continue_session("claude", &args(&["/auth"]), Duration::from_secs(1));
    let exit_stops = !should_continue_session("claude", &args(&["/exit"]), Duration::from_secs(1));

    assert!(auth_continues, "Auth commands should continue sessions");
    assert!(exit_stops, "Exit commands should stop sessions");

    println!("[SUCCESS] Session continuation behavior verified:");
    println!("  - Auth/setup commands: Continue session for seamless re-authentication");
    println!("  - Exit commands: Properly terminate and return to Terminal Jarvis menu");
}
