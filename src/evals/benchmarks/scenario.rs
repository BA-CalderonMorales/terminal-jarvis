// Benchmark Scenario Data Structures
//
// TOML-based scenario definitions for automated benchmarks.
// Each scenario represents a specific test case with validation and scoring.

use serde::Deserialize;

/// Root structure for a benchmark scenario loaded from TOML
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BenchmarkScenario {
    /// Scenario metadata (id, name, category, etc.)
    pub metadata: ScenarioMetadata,

    /// Prompt configuration for the benchmark
    pub prompt: PromptConfig,

    /// Validation rules for the response
    pub validation: ValidationConfig,

    /// Scoring configuration
    pub scoring: ScoringConfig,
}

/// Metadata about the benchmark scenario
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ScenarioMetadata {
    /// Unique identifier for this scenario (e.g., "code-completion-basic-001")
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Category (e.g., "code-completion", "refactoring", "debugging")
    pub category: String,

    /// Semantic version of this scenario
    pub version: String,

    /// Difficulty level (e.g., "basic", "intermediate", "advanced")
    pub difficulty: String,
}

/// Prompt configuration for the benchmark
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PromptConfig {
    /// Template for the prompt to send to the AI tool
    pub template: String,
}

/// Validation configuration for response checking
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ValidationConfig {
    /// Type of validation (e.g., "pattern-match", "syntax-check", "execution")
    #[serde(rename = "type")]
    pub validation_type: String,

    /// Expected patterns for pattern-match validation
    #[serde(default)]
    pub expected_patterns: Vec<String>,

    /// Optional validation rules (for future extensibility)
    #[serde(default)]
    pub rules: Vec<String>,
}

/// Scoring configuration
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ScoringConfig {
    /// Total points possible for this scenario
    pub points_possible: u32,

    /// Pass threshold (0.0 - 1.0)
    pub pass_threshold: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_complete_scenario() {
        let toml_content = r#"
[metadata]
id = "test-001"
name = "Test Scenario"
category = "testing"
version = "1.0.0"
difficulty = "basic"

[prompt]
template = "Test prompt"

[validation]
type = "pattern-match"
expected_patterns = ["pattern1", "pattern2"]

[scoring]
points_possible = 10
pass_threshold = 0.75
"#;

        let scenario: BenchmarkScenario = toml::from_str(toml_content).unwrap();

        assert_eq!(scenario.metadata.id, "test-001");
        assert_eq!(scenario.metadata.name, "Test Scenario");
        assert_eq!(scenario.validation.validation_type, "pattern-match");
        assert_eq!(scenario.validation.expected_patterns.len(), 2);
        assert_eq!(scenario.scoring.points_possible, 10);
        assert_eq!(scenario.scoring.pass_threshold, 0.75);
    }

    #[test]
    fn test_deserialize_minimal_scenario() {
        // Test with minimal required fields
        let toml_content = r#"
[metadata]
id = "minimal-001"
name = "Minimal"
category = "test"
version = "1.0.0"
difficulty = "basic"

[prompt]
template = "Test"

[validation]
type = "simple"

[scoring]
points_possible = 5
pass_threshold = 0.5
"#;

        let scenario: BenchmarkScenario = toml::from_str(toml_content).unwrap();

        assert_eq!(scenario.metadata.id, "minimal-001");
        assert!(scenario.validation.expected_patterns.is_empty());
        assert!(scenario.validation.rules.is_empty());
    }
}
