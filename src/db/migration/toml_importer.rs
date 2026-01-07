// TOML to Database Importer
//
// Reads existing TOML tool configurations and imports them into the database.
// Preserves all tool metadata, installation info, and auth configuration.

use crate::db::core::connection::DatabaseManager;
use crate::db::tools::{Tool, ToolAuth, ToolInstall};
use crate::db::ToolsRepository;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Result of an import operation
#[derive(Debug, Clone)]
pub struct ImportResult {
    pub tool_id: String,
    pub success: bool,
    pub message: String,
}

/// Statistics from import operation
#[derive(Debug, Default)]
pub struct ImportStats {
    pub total: usize,
    pub imported: usize,
    pub skipped: usize,
    pub errors: usize,
    pub results: Vec<ImportResult>,
}

impl ImportStats {
    /// Check if all imports succeeded
    pub fn all_success(&self) -> bool {
        self.errors == 0
    }

    /// Get summary message
    pub fn summary(&self) -> String {
        format!(
            "Imported {}/{} tools ({} skipped, {} errors)",
            self.imported, self.total, self.skipped, self.errors
        )
    }
}

/// TOML tool configuration structure (matches config/tools/*.toml)
#[derive(Debug, Deserialize)]
struct TomlToolConfig {
    tool: TomlTool,
}

#[allow(dead_code)] // Some fields for TOML compat, not all used in conversion
#[derive(Debug, Deserialize)]
struct TomlTool {
    display_name: String,
    config_key: String,
    description: Option<String>,
    homepage: Option<String>,
    documentation: Option<String>,
    cli_command: String,
    #[serde(default)]
    requires_npm: bool,
    #[serde(default)]
    requires_sudo: bool,
    #[serde(default = "default_status")]
    status: String,
    install: Option<TomlInstall>,
    #[allow(dead_code)] // Will be used when tool_update integration is added
    update: Option<TomlUpdate>,
    auth: Option<TomlAuth>,
    #[allow(dead_code)] // Will be used when tool_features integration is added
    features: Option<TomlFeatures>,
}

fn default_status() -> String {
    "stable".to_string()
}

#[derive(Debug, Deserialize)]
struct TomlInstall {
    command: String,
    args: Vec<String>,
    verify_command: Option<String>,
    post_install_message: Option<String>,
}

#[allow(dead_code)] // Will be used when tool_update table integration is added
#[derive(Debug, Deserialize)]
struct TomlUpdate {
    command: String,
    args: Vec<String>,
    verify_command: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TomlAuth {
    #[serde(default)]
    env_vars: Vec<String>,
    setup_url: Option<String>,
    #[serde(default)]
    browser_auth: bool,
    auth_instructions: Option<String>,
}

#[allow(dead_code)] // Will be used when tool_features table integration is added
#[derive(Debug, Deserialize)]
struct TomlFeatures {
    #[serde(default)]
    supports_files: bool,
    #[serde(default)]
    supports_streaming: bool,
    #[serde(default)]
    supports_conversation: bool,
    max_context_tokens: Option<i64>,
    #[serde(default)]
    supported_languages: Vec<String>,
}

/// TOML to Database importer
pub struct TomlImporter {
    tools_repo: ToolsRepository,
    config_dir: PathBuf,
}

impl TomlImporter {
    /// Create a new importer
    pub async fn new(db: Arc<DatabaseManager>) -> Result<Self> {
        let config_dir = Self::find_config_dir()?;
        Ok(Self {
            tools_repo: ToolsRepository::new(db),
            config_dir,
        })
    }

    /// Create importer with custom config directory
    pub fn with_config_dir(db: Arc<DatabaseManager>, config_dir: PathBuf) -> Self {
        Self {
            tools_repo: ToolsRepository::new(db),
            config_dir,
        }
    }

    /// Find the config/tools directory
    fn find_config_dir() -> Result<PathBuf> {
        // Allow environment override for packaged/bundled installs
        if let Ok(override_dir) = std::env::var("TERMINAL_JARVIS_CONFIG_DIR") {
            let override_path = PathBuf::from(&override_dir);
            if override_path.exists() && override_path.is_dir() {
                return Ok(override_path);
            } else {
                eprintln!(
                    "Warning: TERMINAL_JARVIS_CONFIG_DIR='{}' is not a valid directory",
                    override_dir
                );
            }
        }

        // Try relative to current directory first
        let mut paths = vec![
            PathBuf::from("config/tools"),
            PathBuf::from("./config/tools"),
        ];

        // Try from executable location and its parent (npm package layout: bin/../config/tools)
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                paths.push(parent.join("config/tools"));
                if let Some(grandparent) = parent.parent() {
                    paths.push(grandparent.join("config/tools"));
                }
            }
        }

        for path in paths {
            if path.exists() && path.is_dir() {
                return Ok(path);
            }
        }

        Err(anyhow!(
            "Could not find config/tools directory. Run from project root."
        ))
    }

    /// Import all TOML files from config/tools/
    pub async fn import_all(&self) -> Result<ImportStats> {
        let mut stats = ImportStats::default();

        // Find all .toml files in config directory
        let toml_files = self.find_toml_files()?;
        stats.total = toml_files.len();

        for path in toml_files {
            let result = self.import_file(&path).await;
            match result {
                Ok(import_result) => {
                    if import_result.success {
                        stats.imported += 1;
                    } else {
                        stats.skipped += 1;
                    }
                    stats.results.push(import_result);
                }
                Err(e) => {
                    let tool_id = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    stats.errors += 1;
                    stats.results.push(ImportResult {
                        tool_id,
                        success: false,
                        message: format!("Error: {}", e),
                    });
                }
            }
        }

        Ok(stats)
    }

    /// Import a single TOML file
    pub async fn import_file(&self, path: &Path) -> Result<ImportResult> {
        let tool_id = path
            .file_stem()
            .ok_or_else(|| anyhow!("Invalid file path"))?
            .to_string_lossy()
            .to_string();

        // Read and parse TOML
        let content = std::fs::read_to_string(path)?;
        let config: TomlToolConfig = toml::from_str(&content)?;

        // Check if tool already exists
        if let Some(_existing) = self.tools_repo.find_by_id(&tool_id).await? {
            return Ok(ImportResult {
                tool_id,
                success: false,
                message: "Tool already exists in database (skipped)".to_string(),
            });
        }

        // Convert and save tool
        let tool = self.convert_tool(&tool_id, &config.tool);
        self.tools_repo.save(&tool).await?;

        // Save install info if present
        if let Some(install) = &config.tool.install {
            let install_info = self.convert_install(&tool_id, install);
            self.tools_repo.save_install_info(&install_info).await?;
        }

        // Save auth info if present
        if let Some(auth) = &config.tool.auth {
            let auth_info = self.convert_auth(&tool_id, auth);
            self.tools_repo.save_auth_info(&auth_info).await?;
        }

        Ok(ImportResult {
            tool_id,
            success: true,
            message: "Imported successfully".to_string(),
        })
    }

    /// Find all TOML files in config directory
    fn find_toml_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if !self.config_dir.exists() {
            return Err(anyhow!(
                "Config directory not found: {}",
                self.config_dir.display()
            ));
        }

        for entry in std::fs::read_dir(&self.config_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "toml").unwrap_or(false) {
                files.push(path);
            }
        }

        files.sort();
        Ok(files)
    }

    /// Convert TOML tool to database entity
    fn convert_tool(&self, tool_id: &str, toml: &TomlTool) -> Tool {
        Tool {
            id: tool_id.to_string(),
            display_name: toml.display_name.clone(),
            cli_command: toml.cli_command.clone(),
            description: toml.description.clone(),
            homepage: toml.homepage.clone(),
            documentation: toml.documentation.clone(),
            requires_npm: toml.requires_npm,
            requires_sudo: toml.requires_sudo,
            status: toml.status.clone(),
            enabled: true,
            auto_update: true,
        }
    }

    /// Convert TOML install to database entity
    fn convert_install(&self, tool_id: &str, toml: &TomlInstall) -> ToolInstall {
        ToolInstall {
            tool_id: tool_id.to_string(),
            command: toml.command.clone(),
            args: toml.args.clone(),
            verify_command: toml.verify_command.clone(),
            post_install_message: toml.post_install_message.clone(),
        }
    }

    /// Convert TOML auth to database entity
    fn convert_auth(&self, tool_id: &str, toml: &TomlAuth) -> ToolAuth {
        ToolAuth {
            tool_id: tool_id.to_string(),
            env_vars: toml.env_vars.clone(),
            setup_url: toml.setup_url.clone(),
            browser_auth: toml.browser_auth,
            auth_instructions: toml.auth_instructions.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_import_stats_summary() {
        let stats = ImportStats {
            total: 10,
            imported: 8,
            skipped: 1,
            errors: 1,
            results: vec![],
        };

        assert!(!stats.all_success());
        assert!(stats.summary().contains("8/10"));
    }

    #[test]
    fn test_toml_parsing() {
        let toml_content = r#"
[tool]
display_name = "Test Tool"
config_key = "test"
cli_command = "test-cmd"
requires_npm = false
"#;

        let config: TomlToolConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.tool.display_name, "Test Tool");
        assert_eq!(config.tool.cli_command, "test-cmd");
        assert!(!config.tool.requires_npm);
    }

    #[test]
    fn test_find_config_dir_respects_env_override() {
        let temp_dir = TempDir::new().unwrap();
        let tools_dir = temp_dir.path().join("config").join("tools");
        std::fs::create_dir_all(&tools_dir).unwrap();

        std::env::set_var("TERMINAL_JARVIS_CONFIG_DIR", &tools_dir);
        let found = TomlImporter::find_config_dir().unwrap();
        std::env::remove_var("TERMINAL_JARVIS_CONFIG_DIR");

        assert_eq!(found, tools_dir);
    }
}
