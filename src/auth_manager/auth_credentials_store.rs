// Authentication Credentials Store - Persisted API key management
//
// CURRENT: Uses TOML file storage at ~/.config/terminal-jarvis/credentials.toml
//
// MIGRATION NOTE: Database-backed credentials storage is available via
// CredentialsRepository in src/db/credentials/. For async contexts, prefer
// using the repository directly. This module provides sync compatibility.
//
// The TOML storage is kept as the primary sync storage because:
// 1. Many auth flows are synchronous
// 2. Nested tokio runtimes panic ("Cannot start a runtime from within a runtime")
// 3. Simple file-based storage is reliable and debuggable

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CredentialsFile {
    pub tools: HashMap<String, HashMap<String, String>>, // tool_name -> env_var -> value
}

pub struct CredentialsStore;

impl CredentialsStore {
    fn creds_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join("terminal-jarvis");
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("credentials.toml"))
    }

    pub fn load() -> Result<CredentialsFile> {
        let path = Self::creds_path()?;
        if !path.exists() {
            return Ok(CredentialsFile::default());
        }
        let content = fs::read_to_string(&path)?;
        let data: CredentialsFile = toml::from_str(&content)?;
        Ok(data)
    }

    pub fn save(creds: &CredentialsFile) -> Result<()> {
        let path = Self::creds_path()?;
        let content = toml::to_string_pretty(creds)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Merge new env vars for a tool (overwriting provided keys) and persist
    pub fn upsert_tool_env_vars(tool: &str, vars: &HashMap<String, String>) -> Result<()> {
        let mut data = Self::load()?;
        let entry = data.tools.entry(tool.to_string()).or_default();
        for (k, v) in vars {
            entry.insert(k.clone(), v.clone());
        }
        Self::save(&data)
    }

    /// Retrieve saved env vars for a tool
    pub fn get_tool_env_vars(tool: &str) -> Result<HashMap<String, String>> {
        let data = Self::load()?;
        Ok(data.tools.get(tool).cloned().unwrap_or_default())
    }

    /// Delete specific env vars for a tool (if keys empty, delete entire tool entry)
    pub fn delete_tool_env_vars(tool: &str, keys: &[String]) -> Result<()> {
        let mut data = Self::load()?;
        if keys.is_empty() {
            data.tools.remove(tool);
        } else if let Some(entry) = data.tools.get_mut(tool) {
            for k in keys {
                entry.remove(k);
            }
            // If empty after removal, drop the tool
            if entry.is_empty() {
                data.tools.remove(tool);
            }
        }
        Self::save(&data)
    }

    /// Clear all saved credentials across all tools
    pub fn clear_all() -> Result<()> {
        let mut data = Self::load()?;
        data.tools.clear();
        Self::save(&data)
    }
}
