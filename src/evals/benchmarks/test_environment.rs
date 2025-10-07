// Benchmark Test Environment Domain
// Provides isolated execution environments for benchmarks
//
// Pattern: Similar to AuthTestEnvironment in tests/auth_behavior_tests.rs
// Purpose: Ensure benchmarks run in clean, isolated, reproducible environments

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

/// Isolated test environment for benchmark execution
///
/// Provides:
/// - Temporary workspace directory
/// - Environment variable isolation
/// - Automatic cleanup on drop
/// - Workspace structure creation
///
/// # Example
/// ```no_run
/// use terminal_jarvis::evals::benchmarks::BenchmarkTestEnvironment;
///
/// let env = BenchmarkTestEnvironment::new().unwrap();
/// assert!(env.workspace_dir.exists());
/// // Workspace automatically cleaned up when env goes out of scope
/// ```
pub struct BenchmarkTestEnvironment {
    /// Temporary directory (cleaned up on drop) - Phase 2
    #[allow(dead_code)]
    pub temp_dir: TempDir,

    /// Workspace directory for benchmark execution - Phase 2
    #[allow(dead_code)]
    pub workspace_dir: PathBuf,

    /// Environment variables for this benchmark - Phase 2
    #[allow(dead_code)]
    env_vars: HashMap<String, String>,
}

impl BenchmarkTestEnvironment {
    /// Create a new isolated benchmark environment
    ///
    /// Creates:
    /// - Temporary directory
    /// - workspace/ subdirectory
    /// - workspace/src/ subdirectory
    ///
    /// # Errors
    /// Returns error if:
    /// - Cannot create temporary directory
    /// - Cannot create workspace structure
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let workspace_dir = temp_dir.path().join("workspace");

        // Create standard workspace structure
        std::fs::create_dir_all(&workspace_dir)?;
        std::fs::create_dir_all(workspace_dir.join("src"))?;

        Ok(Self {
            temp_dir,
            workspace_dir,
            env_vars: HashMap::new(),
        })
    }

    /// Set an environment variable for benchmark execution (Phase 2 API)
    ///
    /// # Arguments
    /// * `key` - Environment variable name
    /// * `value` - Environment variable value
    ///
    /// # Example
    /// ```no_run
    /// use terminal_jarvis::evals::benchmarks::BenchmarkTestEnvironment;
    ///
    /// let mut env = BenchmarkTestEnvironment::new().unwrap();
    /// env.set_env_var("API_KEY", "test-key-123").unwrap();
    /// ```
    #[allow(dead_code)]
    pub fn set_env_var(&mut self, key: &str, value: &str) -> Result<()> {
        self.env_vars.insert(key.to_string(), value.to_string());
        Ok(())
    }

    /// Clear an environment variable (Phase 2 API)
    ///
    /// # Arguments
    /// * `key` - Environment variable name to clear
    #[allow(dead_code)]
    pub fn clear_env_var(&mut self, key: &str) -> Result<()> {
        self.env_vars.remove(key);
        Ok(())
    }

    /// Get all environment variables (Phase 2 API)
    ///
    /// # Returns
    /// Reference to environment variables HashMap
    #[allow(dead_code)]
    pub fn get_env_vars(&self) -> &HashMap<String, String> {
        &self.env_vars
    }

    /// Get the workspace directory path (Phase 2 API)
    ///
    /// # Returns
    /// PathBuf to the workspace directory
    #[allow(dead_code)]
    pub fn workspace_path(&self) -> &PathBuf {
        &self.workspace_dir
    }
}

impl Default for BenchmarkTestEnvironment {
    fn default() -> Self {
        Self::new().expect("Failed to create default BenchmarkTestEnvironment")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_workspace_structure() {
        let env = BenchmarkTestEnvironment::new().unwrap();
        assert!(env.workspace_dir.exists());
        assert!(env.workspace_dir.join("src").exists());
    }

    #[test]
    fn test_workspace_is_writable() {
        let env = BenchmarkTestEnvironment::new().unwrap();
        let test_file = env.workspace_dir.join("src").join("test.rs");
        std::fs::write(&test_file, "// test content").unwrap();
        assert!(test_file.exists());
    }

    #[test]
    fn test_environment_variables() {
        let mut env = BenchmarkTestEnvironment::new().unwrap();

        env.set_env_var("TEST_VAR", "test_value").unwrap();
        assert_eq!(
            env.get_env_vars().get("TEST_VAR"),
            Some(&"test_value".to_string())
        );

        env.clear_env_var("TEST_VAR").unwrap();
        assert!(!env.get_env_vars().contains_key("TEST_VAR"));
    }

    #[test]
    fn test_cleanup_on_drop() {
        let workspace_path;
        {
            let env = BenchmarkTestEnvironment::new().unwrap();
            workspace_path = env.workspace_dir.clone();
            assert!(workspace_path.exists());
        }
        // After drop, temp_dir should clean up automatically
        assert!(!workspace_path.exists());
    }
}
