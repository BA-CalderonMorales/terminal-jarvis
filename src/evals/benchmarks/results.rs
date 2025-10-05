// Benchmark Results - JSON-Exportable Result Types
//
// This module provides data structures for benchmark execution results.
// Results are serializable to JSON for cross-language validation (Rust → TypeScript).

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Complete benchmark execution result with validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Unique identifier for the benchmark scenario
    pub benchmark_id: String,

    /// Name of the tool being benchmarked (e.g., "claude", "cursor", "copilot")
    pub tool_name: String,

    /// Version of the benchmark scenario used
    pub scenario_version: String,

    /// ISO 8601 timestamp of when the benchmark was executed
    pub execution_timestamp: String,

    /// Total execution time in milliseconds
    pub execution_time_ms: u64,

    /// Whether the benchmark passed overall
    pub passed: bool,

    /// Overall score (0.0 - 10.0)
    pub score: f64,

    /// Raw output from the tool
    pub output: String,

    /// Detailed validation results
    pub validation_details: ValidationResult,
}

/// Validation result containing test case details and errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed
    pub passed: bool,

    /// Overall validation score (0.0 - 10.0)
    pub score: f64,

    /// Individual test case results
    pub test_case_results: Vec<TestCaseResult>,

    /// List of validation errors
    pub errors: Vec<String>,
}

/// Result of a single test case validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseResult {
    /// Name of the test case
    pub test_name: String,

    /// Whether the test case passed
    pub passed: bool,

    /// Expected value or pattern
    pub expected: String,

    /// Actual value from tool output
    pub actual: String,

    /// Error message if test failed
    pub error: Option<String>,
}

impl BenchmarkResult {
    /// Export the benchmark result to a JSON file
    ///
    /// # Arguments
    /// * `output_dir` - Directory where the JSON file will be written
    ///
    /// # Returns
    /// * `Result<PathBuf>` - Path to the created JSON file
    ///
    /// # Filename Format
    /// `{benchmark_id}_{tool_name}_{timestamp}.json`
    ///
    /// # Example
    /// ```no_run
    /// use terminal_jarvis::evals::benchmarks::BenchmarkResult;
    /// use std::path::Path;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let result = BenchmarkResult {
    ///     benchmark_id: "test-001".to_string(),
    ///     tool_name: "claude".to_string(),
    ///     // ... other fields
    /// #   scenario_version: "1.0.0".to_string(),
    /// #   execution_timestamp: "2025-10-05T01:30:00Z".to_string(),
    /// #   execution_time_ms: 1000,
    /// #   passed: true,
    /// #   score: 9.5,
    /// #   output: "test output".to_string(),
    /// #   validation_details: terminal_jarvis::evals::benchmarks::ValidationResult {
    /// #       passed: true,
    /// #       score: 9.5,
    /// #       test_case_results: vec![],
    /// #       errors: vec![],
    /// #   },
    /// };
    ///
    /// let path = result.export_json(Path::new("./output"))?;
    /// println!("Exported to: {:?}", path);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn export_json(&self, output_dir: &Path) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        // Extract timestamp portion for filename (remove special characters)
        let timestamp_safe = self.execution_timestamp.replace(':', "-").replace(' ', "_");

        // Construct filename: {benchmark_id}_{tool_name}_{timestamp}.json
        let filename = format!(
            "{}_{}_{}.json",
            self.benchmark_id, self.tool_name, timestamp_safe
        );

        let file_path = output_dir.join(&filename);

        // Serialize to pretty-printed JSON
        let json = serde_json::to_string_pretty(self)?;

        // Write to file
        fs::write(&file_path, json)?;

        Ok(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_benchmark_result_creation() {
        let result = BenchmarkResult {
            benchmark_id: "test-001".to_string(),
            tool_name: "claude".to_string(),
            scenario_version: "1.0.0".to_string(),
            execution_timestamp: "2025-10-05T01:30:00Z".to_string(),
            execution_time_ms: 1000,
            passed: true,
            score: 9.5,
            output: "test output".to_string(),
            validation_details: ValidationResult {
                passed: true,
                score: 9.5,
                test_case_results: vec![],
                errors: vec![],
            },
        };

        assert_eq!(result.benchmark_id, "test-001");
        assert_eq!(result.tool_name, "claude");
        assert!(result.passed);
    }

    #[test]
    fn test_export_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir.path().join("nested").join("path");

        let result = BenchmarkResult {
            benchmark_id: "test-001".to_string(),
            tool_name: "claude".to_string(),
            scenario_version: "1.0.0".to_string(),
            execution_timestamp: "2025-10-05T01:30:00Z".to_string(),
            execution_time_ms: 1000,
            passed: true,
            score: 9.5,
            output: "test".to_string(),
            validation_details: ValidationResult {
                passed: true,
                score: 9.5,
                test_case_results: vec![],
                errors: vec![],
            },
        };

        let path = result.export_json(&nested).unwrap();
        assert!(path.exists());
        assert!(nested.exists());
    }
}
