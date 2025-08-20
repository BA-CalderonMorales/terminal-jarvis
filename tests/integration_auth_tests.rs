use std::env;
use std::process::Command;
use std::sync::Mutex;
use terminal_jarvis::auth_manager::AuthManager;

// Mutex to ensure environment variable tests don't run in parallel
static ENV_TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Integration test to reproduce actual browser-opening behavior
/// This will install and test real NPM packages
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_install_and_run_tools_for_browser_behavior() {
        // Acquire mutex to prevent parallel environment variable manipulation
        let _guard = ENV_TEST_MUTEX.lock().unwrap();
        
        // Clear all API keys to force authentication
        clear_all_auth_env_vars();

        // Install the tools if not already present
        println!("Installing tools for testing...");
        install_tools_for_testing();

        // Test each tool for browser opening behavior
        test_gemini_cli_auth_behavior();
        test_qwen_code_auth_behavior();
    }

    #[test]
    fn test_auth_manager_integration() {
        // Acquire mutex to prevent parallel environment variable manipulation
        let _guard = ENV_TEST_MUTEX.lock().unwrap();
        
        // Test that our AuthManager correctly detects the environment
        println!("Testing AuthManager environment detection...");

        // Test in current environment
        let should_prevent = AuthManager::should_prevent_browser_opening();
        println!("Should prevent browser opening: {should_prevent}");

        // Test API key detection
        clear_all_auth_env_vars();
        assert!(!AuthManager::check_api_keys_for_tool("gemini"));
        assert!(!AuthManager::check_api_keys_for_tool("qwen"));

        // Set API keys and test again
        env::set_var("GOOGLE_API_KEY", "test-key");
        assert!(AuthManager::check_api_keys_for_tool("gemini"));

        env::set_var("QWEN_CODE_API_KEY", "test-key");
        assert!(AuthManager::check_api_keys_for_tool("qwen"));

        println!("✅ AuthManager tests passed");
    }

    #[test]
    fn test_no_browser_environment_setup() {
        // Acquire mutex to prevent parallel environment variable manipulation
        let _guard = ENV_TEST_MUTEX.lock().unwrap();
        
        // Test that we can set up a no-browser environment
        AuthManager::prepare_auth_safe_environment()
            .expect("Failed to prepare auth safe environment");

        // Verify no-browser environment variables are set
        assert_eq!(env::var("NO_BROWSER").unwrap(), "1");
        assert_eq!(
            env::var("BROWSER").unwrap(),
            "echo 'Browser prevented by Terminal Jarvis:'"
        );

        println!("✅ No-browser environment setup successful");
    }

    fn clear_all_auth_env_vars() {
        let auth_vars = [
            "GOOGLE_API_KEY",
            "GEMINI_API_KEY",
            "GOOGLE_APPLICATION_CREDENTIALS",
            "QWEN_CODE_API_KEY",
            "DASHSCOPE_API_KEY",
            "ANTHROPIC_API_KEY",
            "CLAUDE_API_KEY",
            "OPENAI_API_KEY",
        ];

        for var in &auth_vars {
            env::remove_var(var);
        }
    }

    fn install_tools_for_testing() {
        // Try to install gemini-cli
        println!("Attempting to install @google/gemini-cli...");
        let gemini_install = Command::new("npm")
            .args(["install", "-g", "@google/gemini-cli"])
            .output();

        match gemini_install {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ Gemini CLI installed successfully");
                } else {
                    println!(
                        "⚠️ Gemini CLI installation failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => println!("⚠️ Failed to run npm for gemini: {e}"),
        }

        // Try to install qwen-code
        println!("Attempting to install @qwen-code/qwen-code...");
        let qwen_install = Command::new("npm")
            .args(["install", "-g", "@qwen-code/qwen-code"])
            .output();

        match qwen_install {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ Qwen Code installed successfully");
                } else {
                    println!(
                        "⚠️ Qwen Code installation failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => println!("⚠️ Failed to run npm for qwen: {e}"),
        }
    }

    fn test_gemini_cli_auth_behavior() {
        println!("\n=== Testing Gemini CLI Authentication Behavior ===");

        // Try different commands that might trigger authentication
        let test_commands = [
            vec!["--version"],
            vec!["--help"],
            // vec!["login"], // This would definitely trigger browser opening
            // vec!["auth", "login"], // Alternative auth command
        ];

        for cmd_args in test_commands.iter() {
            println!("Testing gemini with args: {cmd_args:?}");

            let result = Command::new("gemini").args(cmd_args).output();

            match result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    // Look for browser-opening indicators
                    let browser_indicators = [
                        "Opening browser",
                        "Please visit",
                        "https://accounts.google.com",
                        "oauth",
                        "browser will open",
                        "authentication required",
                        "Please go to",
                        "authorize this application",
                    ];

                    let has_browser_behavior = browser_indicators.iter().any(|&indicator| {
                        stdout.to_lowercase().contains(&indicator.to_lowercase())
                            || stderr.to_lowercase().contains(&indicator.to_lowercase())
                    });

                    if has_browser_behavior {
                        println!("🚨 FOUND BROWSER OPENING BEHAVIOR!");
                        println!("Command: gemini {cmd_args:?}");
                        println!("STDOUT: {stdout}");
                        println!("STDERR: {stderr}");
                    } else {
                        println!("✅ No browser opening detected for: gemini {cmd_args:?}");
                    }

                    // Check for API key prompts
                    let api_key_indicators = [
                        "API key",
                        "GOOGLE_API_KEY",
                        "GEMINI_API_KEY",
                        "Please set",
                        "environment variable",
                    ];

                    let has_api_key_prompt = api_key_indicators.iter().any(|&indicator| {
                        stdout.to_lowercase().contains(&indicator.to_lowercase())
                            || stderr.to_lowercase().contains(&indicator.to_lowercase())
                    });

                    if has_api_key_prompt {
                        println!("📝 Tool prompts for API key: gemini {cmd_args:?}");
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to run gemini {cmd_args:?}: {e}");

                    // Try with npx as fallback
                    println!("Trying with npx...");
                    let npx_result = Command::new("npx")
                        .args(["@google/gemini-cli"].iter().chain(cmd_args.iter()))
                        .output();

                    if let Ok(output) = npx_result {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("npx output - STDOUT: {stdout}");
                        println!("npx output - STDERR: {stderr}");
                    }
                }
            }
        }
    }

    fn test_qwen_code_auth_behavior() {
        println!("\n=== Testing Qwen Code Authentication Behavior ===");

        let test_commands = [
            vec!["--version"],
            vec!["--help"],
            // vec!["login"], // This might trigger browser opening
        ];

        for cmd_args in test_commands.iter() {
            println!("Testing qwen with args: {cmd_args:?}");

            let result = Command::new("qwen").args(cmd_args).output();

            match result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    let browser_indicators = [
                        "Opening browser",
                        "Please visit",
                        "oauth",
                        "browser will open",
                        "authentication required",
                        "Please go to",
                        "authorize this application",
                        "dashscope.console.aliyun.com",
                    ];

                    let has_browser_behavior = browser_indicators.iter().any(|&indicator| {
                        stdout.to_lowercase().contains(&indicator.to_lowercase())
                            || stderr.to_lowercase().contains(&indicator.to_lowercase())
                    });

                    if has_browser_behavior {
                        println!("🚨 FOUND BROWSER OPENING BEHAVIOR!");
                        println!("Command: qwen {cmd_args:?}");
                        println!("STDOUT: {stdout}");
                        println!("STDERR: {stderr}");
                    } else {
                        println!("✅ No browser opening detected for: qwen {cmd_args:?}");
                    }

                    // Check for API key prompts
                    let api_key_indicators = [
                        "API key",
                        "QWEN_CODE_API_KEY",
                        "DASHSCOPE_API_KEY",
                        "Please set",
                        "environment variable",
                    ];

                    let has_api_key_prompt = api_key_indicators.iter().any(|&indicator| {
                        stdout.to_lowercase().contains(&indicator.to_lowercase())
                            || stderr.to_lowercase().contains(&indicator.to_lowercase())
                    });

                    if has_api_key_prompt {
                        println!("📝 Tool prompts for API key: qwen {cmd_args:?}");
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to run qwen {cmd_args:?}: {e}");

                    // Try with npx as fallback
                    println!("Trying with npx...");
                    let npx_result = Command::new("npx")
                        .args(["@qwen-code/qwen-code"].iter().chain(cmd_args.iter()))
                        .output();

                    if let Ok(output) = npx_result {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("npx output - STDOUT: {stdout}");
                        println!("npx output - STDERR: {stderr}");
                    }
                }
            }
        }
    }
}
