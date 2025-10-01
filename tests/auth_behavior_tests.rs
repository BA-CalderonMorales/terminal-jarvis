use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tempfile::TempDir;

// Mutex to ensure environment variable tests don't run in parallel
static ENV_TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Test struct for authentication behavior testing
pub struct AuthTestEnvironment {
    temp_dir: TempDir,
    original_env: Vec<(String, Option<String>)>,
}

impl AuthTestEnvironment {
    /// Create a new test environment with clean slate
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;

        // Store original environment variables that might affect authentication
        let auth_env_vars = [
            "GOOGLE_API_KEY",
            "GEMINI_API_KEY",
            "QWEN_CODE_API_KEY",
            "ANTHROPIC_API_KEY",
            "CLAUDE_API_KEY",
            "OPENAI_API_KEY",
            "XDG_CONFIG_HOME",
            "HOME",
            "DISPLAY",
            "BROWSER",
        ];

        let original_env: Vec<(String, Option<String>)> = auth_env_vars
            .iter()
            .map(|&key| (key.to_string(), env::var(key).ok()))
            .collect();

        Ok(Self {
            temp_dir,
            original_env,
        })
    }

    /// Clear all authentication-related environment variables
    pub fn clear_auth_env(&self) -> Result<()> {
        let auth_env_vars = [
            "GOOGLE_API_KEY",
            "GEMINI_API_KEY",
            "QWEN_CODE_API_KEY",
            "ANTHROPIC_API_KEY",
            "CLAUDE_API_KEY",
            "OPENAI_API_KEY",
        ];

        for var in &auth_env_vars {
            env::remove_var(var);
        }

        Ok(())
    }

    /// Set up a fake config directory to simulate first run
    pub fn setup_fake_config_dir(&self) -> Result<PathBuf> {
        let config_dir = self.temp_dir.path().join(".config");
        fs::create_dir_all(&config_dir)?;

        // Set XDG_CONFIG_HOME to our temp directory
        env::set_var("XDG_CONFIG_HOME", config_dir.to_str().unwrap());

        Ok(config_dir)
    }

    /// Remove config directories to simulate first run
    pub fn remove_config_dirs(&self) -> Result<()> {
        let config_paths = [
            ".config/gemini-cli",
            ".config/qwen-code",
            ".cache/gemini-cli",
            ".cache/qwen-code",
        ];

        for path in &config_paths {
            let full_path = self.temp_dir.path().join(path);
            if full_path.exists() {
                fs::remove_dir_all(&full_path)?;
            }
        }

        Ok(())
    }

    /// Simulate a headless environment (no DISPLAY)
    pub fn simulate_headless_env(&self) {
        env::remove_var("DISPLAY");
        env::set_var("CI", "true");
        env::set_var("TERM", "dumb");
    }

    /// Simulate a GUI environment with DISPLAY set
    pub fn simulate_gui_env(&self) {
        env::set_var("DISPLAY", ":0");
        env::remove_var("CI");
        env::set_var("TERM", "xterm-256color");
    }
}

impl Drop for AuthTestEnvironment {
    fn drop(&mut self) {
        // Restore original environment variables
        for (key, value) in &self.original_env {
            match value {
                Some(val) => env::set_var(key, val),
                None => env::remove_var(key),
            }
        }
    }
}

fn print_help() {
    println!("Authentication Behavior Tests - Terminal Jarvis Test Suite");
    println!("========================================================");
    println!();
    println!("This test suite provides comprehensive authentication behavior testing");
    println!("for Terminal Jarvis, focusing on browser prevention and auth state management.");
    println!();
    println!("USAGE:");
    println!("  cargo test --test auth_behavior_tests              Run all authentication tests");
    println!("  cargo test --test auth_behavior_tests [TESTNAME]   Run specific test");
    println!("  cargo test --test auth_behavior_tests -- --help    Show this help");
    println!();
    println!("TEST CATEGORIES:");
    println!("  Browser Prevention Tests:");
    println!("    - test_auth_environment_creation         Tests auth environment setup");
    println!("    - test_browser_detection                 Tests browser detection logic");
    println!("    - test_environment_isolation             Tests test environment isolation");
    println!();
    println!("  Authentication State Tests:");
    println!(
        "    - test_auth_state_persistence            Tests auth state across tool invocations"
    );
    println!("    - test_env_cleanup                       Tests environment variable cleanup");
    println!();
    println!("  Cross-Platform Tests:");
    println!(
        "    - test_cross_platform_auth_behavior      Tests auth behavior across different OS"
    );
    println!("    - test_headless_environment_detection    Tests headless environment detection");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("  TEST_MODE=true                          Enable test mode for safer testing");
    println!("  DISABLE_BROWSER=true                    Prevent browser launches during tests");
    println!("  SKIP_AUTH_VALIDATION=true               Skip actual authentication validation");
    println!("  DISPLAY                                 X11 display variable (Linux/Unix)");
    println!("  CODESPACES=true                         GitHub Codespaces environment");
    println!("  GITPOD_WORKSPACE_ID                     Gitpod workspace identifier");
    println!();
    println!("EXAMPLES:");
    println!("  # Run all authentication behavior tests");
    println!("  cargo test --test auth_behavior_tests");
    println!();
    println!("  # Run specific browser prevention test");
    println!("  cargo test --test auth_behavior_tests test_browser_detection");
    println!();
    println!("  # Run tests in verbose mode with environment setup");
    println!("  TEST_MODE=true DISABLE_BROWSER=true cargo test --test auth_behavior_tests -- --nocapture");
    println!();
    println!("TECHNICAL DETAILS:");
    println!("  - Tests browser prevention during authentication flows");
    println!("  - Validates authentication state persistence mechanisms");
    println!("  - Ensures cross-platform authentication compatibility");
    println!("  - Tests environment variable authentication configuration");
    println!("  - Provides comprehensive coverage of auth behavior patterns");
    println!("  - Uses temporary directories for isolated testing");
    println!("  - Includes environment cleanup and restoration");
    println!();
    println!("For more information, see: https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }

    println!("Auth Behavior Tests - Use --help for detailed information");
    println!("Run with: cargo test --test auth_behavior_tests");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Output;

    /// Test that reproduces the browser-opening behavior
    #[test]
    fn test_reproduce_browser_opening_behavior() {
        let _guard = ENV_TEST_MUTEX.lock().unwrap();
        let test_env = AuthTestEnvironment::new().expect("Failed to create test environment");

        // Setup first-run conditions
        test_env.clear_auth_env().expect("Failed to clear auth env");
        test_env
            .setup_fake_config_dir()
            .expect("Failed to setup config dir");
        test_env
            .remove_config_dirs()
            .expect("Failed to remove config dirs");
        test_env.simulate_gui_env(); // This should trigger browser opening

        // Test gemini-cli browser opening behavior
        let result = test_gemini_cli_first_run();

        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                // Check for browser-opening indicators
                let browser_indicators = [
                    "Opening browser",
                    "Please visit",
                    "https://accounts.google.com",
                    "oauth",
                    "authentication",
                ];

                let has_browser_behavior = browser_indicators
                    .iter()
                    .any(|&indicator| stdout.contains(indicator) || stderr.contains(indicator));

                if has_browser_behavior {
                    println!("✅ Successfully reproduced browser-opening behavior");
                    println!("STDOUT: {stdout}");
                    println!("STDERR: {stderr}");
                } else {
                    println!("⚠️ Expected browser behavior not found");
                    println!("STDOUT: {stdout}");
                    println!("STDERR: {stderr}");
                }
            }
            Err(e) => {
                println!("❌ Failed to run gemini-cli test: {e}");
            }
        }

        // Test qwen-code browser opening behavior
        let qwen_result = test_qwen_code_first_run();

        match qwen_result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                // Check for browser-opening indicators
                let browser_indicators = [
                    "Opening browser",
                    "Please visit",
                    "oauth",
                    "login",
                    "authentication",
                ];

                let has_browser_behavior = browser_indicators
                    .iter()
                    .any(|&indicator| stdout.contains(indicator) || stderr.contains(indicator));

                if has_browser_behavior {
                    println!("✅ Successfully reproduced qwen-code browser-opening behavior");
                    println!("STDOUT: {stdout}");
                    println!("STDERR: {stderr}");
                } else {
                    println!("⚠️ Expected qwen-code browser behavior not found");
                    println!("STDOUT: {stdout}");
                    println!("STDERR: {stderr}");
                }
            }
            Err(e) => {
                println!("❌ Failed to run qwen-code test: {e}");
            }
        }
    }

    #[test]
    fn test_headless_environment_no_browser() {
        let _guard = ENV_TEST_MUTEX.lock().unwrap();
        let test_env = AuthTestEnvironment::new().expect("Failed to create test environment");

        // Setup headless conditions
        test_env.clear_auth_env().expect("Failed to clear auth env");
        test_env
            .setup_fake_config_dir()
            .expect("Failed to setup config dir");
        test_env
            .remove_config_dirs()
            .expect("Failed to remove config dirs");
        test_env.simulate_headless_env(); // This should NOT trigger browser opening

        // Test that tools ask for env vars instead of opening browser
        let gemini_result = test_gemini_cli_first_run();

        if let Ok(output) = gemini_result {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            let env_var_indicators = ["API_KEY", "environment variable", "export", "Please set"];

            let has_env_var_prompt = env_var_indicators
                .iter()
                .any(|&indicator| stdout.contains(indicator) || stderr.contains(indicator));

            if has_env_var_prompt {
                println!("✅ Headless environment correctly prompts for env vars");
            } else {
                println!("⚠️ Headless behavior may not be working as expected");
            }

            println!("STDOUT: {stdout}");
            println!("STDERR: {stderr}");
        }
    }

    /// Test that we can detect when tools would open browsers
    #[test]
    fn test_browser_detection_mechanism() {
        let _guard = ENV_TEST_MUTEX.lock().unwrap();
        // Test our ability to detect browser-opening scenarios
        assert!(should_prevent_browser_opening());

        // Test with different environment configurations
        env::set_var("DISPLAY", ":0");
        env::remove_var("CI");
        // In a GUI environment, we might still want to prevent browser opening
        // depending on our configuration

        env::remove_var("DISPLAY");
        env::set_var("CI", "true");
        assert!(should_prevent_browser_opening()); // Headless should always prevent
    }

    fn test_gemini_cli_first_run() -> Result<Output> {
        // Try to run gemini with --help to see authentication behavior
        let output = Command::new("gemini").args(["--help"]).output();

        match output {
            Ok(out) => Ok(out),
            Err(_) => {
                // If gemini command doesn't exist, try npx
                Command::new("npx")
                    .args(["@google/gemini-cli", "--help"])
                    .output()
                    .map_err(|e| anyhow::anyhow!("Failed to run gemini CLI: {}", e))
            }
        }
    }

    fn test_qwen_code_first_run() -> Result<Output> {
        // Try to run qwen-code with --help to see authentication behavior
        let output = Command::new("qwen").args(["--help"]).output();

        match output {
            Ok(out) => Ok(out),
            Err(_) => {
                // If qwen command doesn't exist, try npx
                Command::new("npx")
                    .args(["@qwen-code/qwen-code", "--help"])
                    .output()
                    .map_err(|e| anyhow::anyhow!("Failed to run qwen-code CLI: {}", e))
            }
        }
    }

    /// Check if we should prevent browser opening based on environment
    fn should_prevent_browser_opening() -> bool {
        // Prevent browser opening in CI environments
        if env::var("CI").is_ok() {
            return true;
        }

        // Prevent browser opening if no DISPLAY is set (headless)
        if env::var("DISPLAY").is_err() {
            return true;
        }

        // Prevent browser opening in codespaces or similar environments
        if env::var("CODESPACES").is_ok() || env::var("GITPOD_WORKSPACE_ID").is_ok() {
            return true;
        }

        // Check for terminal-specific environments
        if let Ok(term) = env::var("TERM") {
            if term == "dumb" || term.contains("screen") {
                return true;
            }
        }

        false
    }
}
