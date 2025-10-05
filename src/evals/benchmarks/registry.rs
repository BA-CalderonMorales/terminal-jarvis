// Benchmark Scenario Registry
//
// Loads and manages benchmark scenarios from TOML files.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use super::scenario::BenchmarkScenario;

/// Registry for managing benchmark scenarios
#[derive(Debug, Clone)]
pub struct BenchmarkRegistry {
    /// Map of scenario ID -> scenario (Phase 2 - not yet used)
    #[allow(dead_code)]
    scenarios: HashMap<String, BenchmarkScenario>,
}

impl BenchmarkRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            scenarios: HashMap::new(),
        }
    }

    /// Load all scenarios from a directory
    ///
    /// Recursively scans the directory for .toml files and loads them as scenarios.
    /// Phase 2 API - not yet used
    #[allow(dead_code)]
    pub fn from_directory<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir = dir.as_ref();
        let mut registry = Self::new();

        if !dir.exists() {
            return Ok(registry); // Return empty registry if directory doesn't exist
        }

        registry.load_scenarios_recursive(dir)?;
        Ok(registry)
    }

    /// Recursively load scenarios from directory (Phase 2)
    #[allow(dead_code)]
    fn load_scenarios_recursive(&mut self, dir: &Path) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recurse into subdirectories
                self.load_scenarios_recursive(&path)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                // Load TOML file
                self.load_scenario_file(&path)?;
            }
        }

        Ok(())
    }

    /// Load a single scenario from a TOML file (Phase 2)
    #[allow(dead_code)]
    fn load_scenario_file(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read scenario file: {}", path.display()))?;

        let scenario: BenchmarkScenario = toml::from_str(&content)
            .with_context(|| format!("Failed to parse scenario TOML: {}", path.display()))?;

        let id = scenario.metadata.id.clone();
        self.scenarios.insert(id, scenario);

        Ok(())
    }

    /// Add a scenario to the registry (Phase 2 API)
    #[allow(dead_code)]
    pub fn add_scenario(&mut self, scenario: BenchmarkScenario) {
        let id = scenario.metadata.id.clone();
        self.scenarios.insert(id, scenario);
    }

    /// Get a scenario by ID (Phase 2 API)
    #[allow(dead_code)]
    pub fn get_scenario(&self, id: &str) -> Option<&BenchmarkScenario> {
        self.scenarios.get(id)
    }

    /// List all scenarios (Phase 2 API)
    #[allow(dead_code)]
    pub fn list_scenarios(&self) -> Vec<&BenchmarkScenario> {
        self.scenarios.values().collect()
    }

    /// Get number of scenarios in registry (Phase 2 API)
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.scenarios.len()
    }

    /// Get all scenario IDs (Phase 2 API)
    #[allow(dead_code)]
    pub fn scenario_ids(&self) -> Vec<String> {
        self.scenarios.keys().cloned().collect()
    }

    /// Filter scenarios by category (Phase 2 API)
    #[allow(dead_code)]
    pub fn by_category(&self, category: &str) -> Vec<&BenchmarkScenario> {
        self.scenarios
            .values()
            .filter(|s| s.metadata.category == category)
            .collect()
    }

    /// Filter scenarios by difficulty (Phase 2 API)
    #[allow(dead_code)]
    pub fn by_difficulty(&self, difficulty: &str) -> Vec<&BenchmarkScenario> {
        self.scenarios
            .values()
            .filter(|s| s.metadata.difficulty == difficulty)
            .collect()
    }
}

impl Default for BenchmarkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = BenchmarkRegistry::new();
        assert_eq!(registry.count(), 0);
        assert!(registry.list_scenarios().is_empty());
    }

    #[test]
    fn test_add_and_get_scenario() {
        let mut registry = BenchmarkRegistry::new();

        let toml_content = r#"
[metadata]
id = "test-001"
name = "Test"
category = "testing"
version = "1.0.0"
difficulty = "basic"

[prompt]
template = "Test prompt"

[validation]
type = "pattern-match"

[scoring]
points_possible = 10
pass_threshold = 0.75
"#;

        let scenario: BenchmarkScenario = toml::from_str(toml_content).unwrap();
        registry.add_scenario(scenario);

        assert_eq!(registry.count(), 1);
        assert!(registry.get_scenario("test-001").is_some());
        assert!(registry.get_scenario("nonexistent").is_none());
    }

    #[test]
    fn test_filter_by_category() {
        let mut registry = BenchmarkRegistry::new();

        // Add scenarios with different categories
        let toml1 = r#"
[metadata]
id = "code-001"
name = "Code Test"
category = "code-completion"
version = "1.0.0"
difficulty = "basic"

[prompt]
template = "Test"

[validation]
type = "pattern"

[scoring]
points_possible = 10
pass_threshold = 0.5
"#;

        let toml2 = r#"
[metadata]
id = "debug-001"
name = "Debug Test"
category = "debugging"
version = "1.0.0"
difficulty = "basic"

[prompt]
template = "Test"

[validation]
type = "pattern"

[scoring]
points_possible = 10
pass_threshold = 0.5
"#;

        registry.add_scenario(toml::from_str(toml1).unwrap());
        registry.add_scenario(toml::from_str(toml2).unwrap());

        let code_scenarios = registry.by_category("code-completion");
        assert_eq!(code_scenarios.len(), 1);
        assert_eq!(code_scenarios[0].metadata.id, "code-001");
    }
}
