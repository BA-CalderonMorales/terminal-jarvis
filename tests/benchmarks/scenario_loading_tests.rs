// Scenario Loading Tests - TDD Approach
// Write tests FIRST, then implement to make them pass

use std::path::PathBuf;
use terminal_jarvis::evals::benchmarks::{BenchmarkRegistry, BenchmarkScenario};

#[test]
fn test_load_benchmark_scenario_from_toml() {
    // Test: Load a valid benchmark scenario from a TOML file
    // Expected: Should deserialize all fields correctly

    let toml_content = r#"
[metadata]
id = "code-completion-basic-001"
name = "Basic Code Completion"
category = "code-completion"
version = "1.0.0"
difficulty = "basic"

[prompt]
template = "Complete this function: fn add(a: i32, b: i32) -> i32 {"

[validation]
type = "pattern-match"
expected_patterns = ["a \\+ b", "return"]

[scoring]
points_possible = 10
pass_threshold = 0.75
"#;

    let scenario: BenchmarkScenario =
        toml::from_str(toml_content).expect("Failed to deserialize TOML");

    // Validate metadata
    assert_eq!(scenario.metadata.id, "code-completion-basic-001");
    assert_eq!(scenario.metadata.name, "Basic Code Completion");
    assert_eq!(scenario.metadata.category, "code-completion");
    assert_eq!(scenario.metadata.version, "1.0.0");
    assert_eq!(scenario.metadata.difficulty, "basic");

    // Validate prompt
    assert!(scenario.prompt.template.contains("fn add"));

    // Validate validation config
    assert_eq!(scenario.validation.validation_type, "pattern-match");
    assert_eq!(scenario.validation.expected_patterns.len(), 2);

    // Validate scoring
    assert_eq!(scenario.scoring.points_possible, 10);
    assert_eq!(scenario.scoring.pass_threshold, 0.75);
}

#[test]
fn test_scenario_registry_loads_from_directory() {
    // Test: BenchmarkRegistry should load all .toml files from a directory
    // Expected: Should find and parse all scenario files

    // For this test to pass, we need at least one .toml file in config/benchmarks/scenarios/
    let scenarios_dir = PathBuf::from("config/benchmarks/scenarios");

    let registry = BenchmarkRegistry::from_directory(&scenarios_dir)
        .expect("Failed to load scenarios from directory");

    // Should load at least the example scenario
    assert!(
        !registry.list_scenarios().is_empty(),
        "Registry should contain at least one scenario"
    );
}

#[test]
fn test_scenario_has_required_fields() {
    // Test: Verify all required fields are present and validated
    // Expected: Missing fields should fail deserialization

    let incomplete_toml = r#"
[metadata]
id = "test-001"
# Missing name, category, version, difficulty

[prompt]
template = "Test prompt"
"#;

    let result: Result<BenchmarkScenario, _> = toml::from_str(incomplete_toml);
    assert!(
        result.is_err(),
        "Should fail when required fields are missing"
    );
}

#[test]
fn test_invalid_toml_returns_error() {
    // Test: Invalid TOML syntax should return clear error
    // Expected: toml::from_str should return Err

    let invalid_toml = r#"
[metadata
id = "broken"  # Missing closing bracket
"#;

    let result: Result<BenchmarkScenario, _> = toml::from_str(invalid_toml);
    assert!(result.is_err(), "Should fail on invalid TOML syntax");
}

#[test]
fn test_scenario_registry_get_by_id() {
    // Test: Registry should retrieve scenario by ID
    // Expected: get_scenario() should return Some(scenario) for valid ID

    let scenarios_dir = PathBuf::from("config/benchmarks/scenarios");

    let registry =
        BenchmarkRegistry::from_directory(&scenarios_dir).expect("Failed to load scenarios");

    // Try to get the example scenario
    let _scenario = registry.get_scenario("code-completion-basic-001");

    if !registry.list_scenarios().is_empty() {
        // If we have scenarios, at least one should be retrievable
        let first_id = &registry.list_scenarios()[0].metadata.id;
        assert!(registry.get_scenario(first_id).is_some());
    }
}
