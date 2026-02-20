// tool_installation_detection_tests.rs
//
// Tests for tool installation flow and detection correctness.
//
// Covers the bugs fixed in the tool installation/switching work:
//   - NPM gate must not block curl/uv tool installation
//   - Uninstall commands must be derived from TOML, not hardcoded
//   - Tool detection must fall back to shell environment and ~/.local/bin

use terminal_jarvis::installation_arguments::InstallationManager;

// ---------------------------------------------------------------------------
// Install command correctness
// ---------------------------------------------------------------------------

/// Every tool in the registry must have a valid install command.
/// This ensures the TOML configs are loadable and well-formed.
#[test]
fn test_all_tools_have_install_commands() {
    let tool_names = InstallationManager::get_tool_names();
    assert!(
        !tool_names.is_empty(),
        "Tool registry must not be empty - check config/tools/*.toml"
    );

    for tool in &tool_names {
        let cmd = InstallationManager::get_install_command(tool);
        assert!(
            cmd.is_some(),
            "Tool '{tool}' is missing an install command - check config/tools/{tool}.toml"
        );
    }
}

/// curl-based tools must NOT have requires_npm = true.
/// These tools (claude, goose, ollama, vibe, …) use native download and must
/// remain installable without Node.js present.
#[test]
fn test_curl_tools_do_not_require_npm() {
    let curl_tools = ["claude", "goose", "ollama", "vibe", "kimi"];

    for tool in curl_tools {
        if let Some(cmd) = InstallationManager::get_install_command(tool) {
            assert_eq!(
                cmd.command, "curl",
                "Expected '{tool}' to use curl installer, got '{}'",
                cmd.command
            );
            assert!(
                !cmd.requires_npm,
                "curl-based tool '{tool}' must not set requires_npm=true - \
                it should be installable without Node.js"
            );
        }
    }
}

/// kimi should install via official shell installer and not require npm.
#[test]
fn test_kimi_install_uses_official_curl_installer() {
    let cmd = InstallationManager::get_install_command("kimi")
        .expect("kimi must be in the tool registry");

    assert_eq!(cmd.command, "curl", "kimi must use curl installer");
    assert!(
        cmd.args.iter().any(|a| a.contains("code.kimi.com/install.sh")),
        "kimi installer should use official install script URL, got: {:?}",
        cmd.args
    );
    assert!(
        !cmd.requires_npm,
        "kimi must not require npm - official installation uses shell installer"
    );
}

/// npm tools must carry the correct package registry path.
/// This prevents regression of the wrong-package-name bug (e.g. gemini was
/// using @anthropic-ai/gemini-cli instead of @google/gemini-cli).
#[test]
fn test_npm_tool_package_names_are_correct() {
    struct Case {
        tool: &'static str,
        expected_pkg: &'static str,
    }

    let cases = [
        Case {
            tool: "gemini",
            expected_pkg: "@google/gemini-cli",
        },
        Case {
            tool: "amp",
            expected_pkg: "@sourcegraph/amp",
        },
        Case {
            tool: "crush",
            expected_pkg: "@charmland/crush",
        },
        Case {
            tool: "llxprt",
            expected_pkg: "@vybestack/llxprt-code",
        },
        Case {
            tool: "codex",
            expected_pkg: "@openai/codex",
        },
    ];

    for case in &cases {
        if let Some(cmd) = InstallationManager::get_install_command(case.tool) {
            let pkg = cmd.args.last().expect("install command must have args");
            assert_eq!(
                pkg, case.expected_pkg,
                "Tool '{}' has wrong npm package name: got '{}', expected '{}'",
                case.tool, pkg, case.expected_pkg
            );
        }
    }
}

/// NPM tools must not require sudo (NVM users lack system npm in sudo PATH).
#[test]
fn test_npm_tools_do_not_require_sudo() {
    let tool_names = InstallationManager::get_tool_names();

    for tool in tool_names {
        if let Some(cmd) = InstallationManager::get_install_command(&tool) {
            if cmd.requires_npm {
                assert!(
                    !cmd.requires_sudo,
                    "Tool '{tool}' requires npm but also requires_sudo=true - \
                    sudo breaks NVM npm installs; set requires_sudo=false in config/tools/{tool}.toml"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Install method availability checks
// ---------------------------------------------------------------------------

/// Check functions must return a bool and never panic.
#[test]
fn test_availability_checks_do_not_panic() {
    let _ = InstallationManager::check_npm_available();
    let _ = InstallationManager::check_curl_available();
    let _ = InstallationManager::check_uv_available();
}

/// Node.js version helpers must be well-behaved regardless of environment.
#[test]
fn test_node_version_check_is_safe() {
    // check_node_version_compatible() must return Ok or Err, never panic.
    let _ = InstallationManager::check_node_version_compatible();
    // get_node_version() must return Some or None.
    let _ = InstallationManager::get_node_version();
}

// ---------------------------------------------------------------------------
// Tool detection correctness
// ---------------------------------------------------------------------------

/// check_tool_installed must return false for obviously-not-installed names.
#[test]
fn test_check_tool_installed_returns_false_for_unknown() {
    use terminal_jarvis::tools::tools_detection::check_tool_installed;
    // Use a name that is guaranteed not to exist on any CI box
    assert!(
        !check_tool_installed("__terminal_jarvis_nonexistent_tool_xyz_123__"),
        "check_tool_installed should return false for a nonexistent binary"
    );
}

/// check_tool_installed must return true for system utilities that are always
/// present in the test environment (sh, ls).
#[test]
fn test_check_tool_installed_finds_system_tools() {
    use terminal_jarvis::tools::tools_detection::check_tool_installed;
    // `sh` is always available; if this fails the whole test environment is broken
    assert!(
        check_tool_installed("sh"),
        "check_tool_installed should detect 'sh' which is always installed"
    );
}

// ---------------------------------------------------------------------------
// Regression: uninstall derives correct package from TOML
// ---------------------------------------------------------------------------

/// Verify that install command args contain the expected package scope for
/// tools that previously had wrong hardcoded uninstall paths.
/// (The uninstall is now derived from the install command, so fixing the
/// install command also fixes uninstall.)
#[test]
fn test_gemini_install_uses_google_scope() {
    let cmd = InstallationManager::get_install_command("gemini")
        .expect("gemini must be in the tool registry");

    let has_google_pkg = cmd.args.iter().any(|a| a.contains("@google/"));
    let has_wrong_anthropic_pkg = cmd.args.iter().any(|a| a.contains("@anthropic-ai/gemini"));

    assert!(
        has_google_pkg,
        "gemini install args must reference @google/ scope, got: {:?}",
        cmd.args
    );
    assert!(
        !has_wrong_anthropic_pkg,
        "gemini install args must NOT reference @anthropic-ai/gemini-cli (old wrong package)"
    );
}

/// goose uses curl, so its install command must use 'curl', not 'pip'.
/// This guards against the previous bug where goose was being uninstalled via pip.
#[test]
fn test_goose_install_uses_curl_not_pip() {
    let cmd = InstallationManager::get_install_command("goose")
        .expect("goose must be in the tool registry");

    assert_eq!(
        cmd.command, "curl",
        "goose must use curl installer, got '{}' - \
        the old code incorrectly used pip which is wrong for a curl-installed binary",
        cmd.command
    );
    assert!(
        !cmd.requires_npm,
        "goose must not require npm - it uses a curl-based installer"
    );
}
