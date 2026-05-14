use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use terminal_jarvis::tools::tools_config::ToolDefinition;

#[derive(Debug, Deserialize)]
struct ToolFile {
    tool: ToolDefinition,
}

#[derive(Debug, Deserialize)]
struct RegistryExpectations {
    #[serde(default)]
    tools: HashMap<String, ToolExpectation>,
    #[serde(default)]
    deprecated_packages: HashMap<String, DeprecatedPackage>,
}

#[derive(Debug, Deserialize)]
struct ToolExpectation {
    expected_npm_package: Option<String>,
    expected_bin: Option<String>,
    known_collision: Option<String>,
    allowed_bin_mismatch_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeprecatedPackage {
    replacement: String,
    temporary_allow_reason: Option<String>,
}

fn load_tool_configs() -> Vec<(String, PathBuf, ToolDefinition)> {
    let mut configs = Vec::new();

    for entry in fs::read_dir("config/tools").expect("config/tools must be readable") {
        let entry = entry.expect("config/tools entry must be readable");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }

        let content = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("{} must be readable: {err}", path.display()));
        let parsed: ToolFile = toml::from_str(&content)
            .unwrap_or_else(|err| panic!("{} must parse as a tool config: {err}", path.display()));
        let tool_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .expect("tool config filename must be UTF-8")
            .to_string();

        configs.push((tool_name, path, parsed.tool));
    }

    configs.sort_by(|left, right| left.0.cmp(&right.0));
    configs
}

fn load_expectations() -> RegistryExpectations {
    let content = fs::read_to_string("tests/fixtures/tool_registry_expectations.toml")
        .expect("tool registry expectations fixture must be readable");
    toml::from_str(&content).expect("tool registry expectations fixture must parse")
}

fn assert_non_empty(value: &str, label: &str, path: &Path) {
    assert!(
        !value.trim().is_empty(),
        "{label} must be set in {}",
        path.display()
    );
}

fn npm_package_from_args(args: &[String]) -> Option<&str> {
    args.iter()
        .rev()
        .find(|arg| !arg.starts_with('-') && arg.as_str() != "install" && arg.as_str() != "update")
        .map(String::as_str)
}

fn normalize_npm_package(package: &str) -> &str {
    package.strip_suffix("@latest").unwrap_or(package)
}

fn interactive_token_in_part(part: &str, interactive_tokens: &[&str]) -> Option<String> {
    part.to_ascii_lowercase()
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .find(|token| interactive_tokens.contains(token))
        .map(str::to_string)
}

#[test]
fn tool_catalog_configs_parse_and_define_required_commands() {
    let configs = load_tool_configs();
    assert!(
        !configs.is_empty(),
        "tool catalog must not be empty; expected config/tools/*.toml"
    );

    for (tool_name, path, tool) in configs {
        assert_eq!(
            tool.config_key,
            tool_name,
            "tool.config_key must match the filename for {}",
            path.display()
        );

        assert_non_empty(&tool.display_name, "display_name", &path);
        assert_non_empty(&tool.description, "description", &path);
        assert_non_empty(&tool.cli_command, "cli_command", &path);

        assert_non_empty(&tool.install.command, "tool.install.command", &path);
        assert!(
            !tool.install.args.is_empty() || tool.install.pipe_to.is_some(),
            "tool.install must have args or pipe_to in {}",
            path.display()
        );
        assert_non_empty(
            tool.install.verify_command.as_deref().unwrap_or_default(),
            "tool.install.verify_command",
            &path,
        );

        assert_non_empty(&tool.update.command, "tool.update.command", &path);
        assert!(
            !tool.update.args.is_empty() || tool.update.pipe_to.is_some(),
            "tool.update must have args or pipe_to in {}",
            path.display()
        );
        assert_non_empty(
            tool.update.verify_command.as_deref().unwrap_or_default(),
            "tool.update.verify_command",
            &path,
        );
    }
}

#[test]
fn tool_catalog_update_commands_are_non_interactive() {
    let configs = load_tool_configs();
    let interactive_tokens = ["login", "auth", "configure", "wizard", "prompt"];

    for (tool_name, path, tool) in configs {
        let mut update_parts = vec![tool.update.command.as_str()];
        update_parts.extend(tool.update.args.iter().map(String::as_str));
        if let Some(pipe_to) = tool.update.pipe_to.as_deref() {
            update_parts.push(pipe_to);
        }

        for part in update_parts {
            let interactive_token = interactive_token_in_part(part, &interactive_tokens);
            assert!(
                interactive_token.is_none(),
                "tool.update for '{tool_name}' in {} includes interactive token '{}' in command part '{part}'. Remove login/auth/configure/wizard/prompt flows from non-interactive update commands.",
                path.display(),
                interactive_token.unwrap_or_default()
            );
        }
    }
}

#[test]
fn tool_catalog_known_cli_collisions_are_explicitly_documented() {
    let configs = load_tool_configs();
    let expectations = load_expectations();
    let known_collisions = ["code", "cursor"];

    for (tool_name, path, tool) in configs {
        if known_collisions.contains(&tool.cli_command.as_str()) {
            let collision_reason = expectations
                .tools
                .get(&tool_name)
                .and_then(|expectation| expectation.known_collision.as_deref())
                .unwrap_or_default();

            assert!(
                !collision_reason.trim().is_empty(),
                "cli_command '{}' in {} is a known collision and needs an explicit fixture reason",
                tool.cli_command,
                path.display()
            );
        }
    }
}

#[test]
fn tool_catalog_npm_expectations_match_install_and_cli_metadata() {
    let configs = load_tool_configs();
    let expectations = load_expectations();

    for (tool_name, path, tool) in configs {
        if !tool.requires_npm {
            continue;
        }

        let package = npm_package_from_args(&tool.install.args)
            .unwrap_or_else(|| panic!("npm tool '{tool_name}' must install a package"));
        let normalized_package = normalize_npm_package(package);
        let expectation = expectations.tools.get(&tool_name).unwrap_or_else(|| {
            panic!(
                "npm tool '{tool_name}' needs an expectation in tests/fixtures/tool_registry_expectations.toml"
            )
        });

        if let Some(expected_package) = expectation.expected_npm_package.as_deref() {
            assert_eq!(
                normalized_package,
                expected_package,
                "npm package mismatch in {}",
                path.display()
            );
        }

        if let Some(expected_bin) = expectation.expected_bin.as_deref() {
            if tool.cli_command != expected_bin {
                let reason = expectation
                    .allowed_bin_mismatch_reason
                    .as_deref()
                    .unwrap_or_default();
                assert!(
                    !reason.trim().is_empty(),
                    "cli_command '{}' in {} does not match expected npm bin '{expected_bin}' and needs an explicit fixture reason",
                    tool.cli_command,
                    path.display()
                );
            }
        }
    }
}

#[test]
fn tool_catalog_deprecated_npm_packages_are_visible_until_cleaned_up() {
    let configs = load_tool_configs();
    let expectations = load_expectations();

    for (tool_name, path, tool) in configs {
        if !tool.requires_npm {
            continue;
        }

        let package = npm_package_from_args(&tool.install.args)
            .unwrap_or_else(|| panic!("npm tool '{tool_name}' must install a package"));
        let normalized_package = normalize_npm_package(package);

        if let Some(deprecated) = expectations.deprecated_packages.get(normalized_package) {
            assert!(
                !deprecated.replacement.trim().is_empty(),
                "deprecated package '{normalized_package}' in {} must document its replacement",
                path.display()
            );
            assert!(
                deprecated
                    .temporary_allow_reason
                    .as_deref()
                    .unwrap_or_default()
                    .contains("cleanup"),
                "deprecated package '{normalized_package}' in {} needs a temporary cleanup reason",
                path.display()
            );
        }
    }
}
