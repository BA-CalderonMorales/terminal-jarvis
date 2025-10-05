// JSON Bridge Tests - TDD Implementation
//
// These tests define the expected behavior for JSON-exportable benchmark results.
// They serve as the bridge between Rust benchmark execution and TypeScript validation.

use serde_json;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use terminal_jarvis::evals::benchmarks::{BenchmarkResult, TestCaseResult, ValidationResult};

#[test]
fn test_benchmark_result_serializes_to_json() {
    // Arrange: Create a sample benchmark result
    let result = BenchmarkResult {
        benchmark_id: "code-completion-basic-001".to_string(),
        tool_name: "claude".to_string(),
        scenario_version: "1.0.0".to_string(),
        execution_timestamp: "2025-10-05T01:30:00Z".to_string(),
        execution_time_ms: 1234,
        passed: true,
        score: 9.5,
        output: "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        validation_details: ValidationResult {
            passed: true,
            score: 9.5,
            test_case_results: vec![TestCaseResult {
                test_name: "pattern_match_addition".to_string(),
                passed: true,
                expected: "a \\+ b".to_string(),
                actual: "a + b".to_string(),
                error: None,
            }],
            errors: vec![],
        },
    };

    // Act: Serialize to JSON
    let json = serde_json::to_string_pretty(&result).expect("Failed to serialize");

    // Assert: Verify JSON structure contains expected fields
    assert!(json.contains("\"benchmark_id\""));
    assert!(json.contains("\"code-completion-basic-001\""));
    assert!(json.contains("\"tool_name\""));
    assert!(json.contains("\"claude\""));
    assert!(json.contains("\"passed\": true"));
    assert!(json.contains("\"score\": 9.5"));
    assert!(json.contains("\"validation_details\""));
}

#[test]
fn test_benchmark_result_deserializes_from_json() {
    // Arrange: Create JSON string
    let json = r#"{
        "benchmark_id": "code-completion-basic-001",
        "tool_name": "claude",
        "scenario_version": "1.0.0",
        "execution_timestamp": "2025-10-05T01:30:00Z",
        "execution_time_ms": 1234,
        "passed": true,
        "score": 9.5,
        "output": "fn add(a: i32, b: i32) -> i32 { a + b }",
        "validation_details": {
            "passed": true,
            "score": 9.5,
            "test_case_results": [
                {
                    "test_name": "pattern_match_addition",
                    "passed": true,
                    "expected": "a \\+ b",
                    "actual": "a + b",
                    "error": null
                }
            ],
            "errors": []
        }
    }"#;

    // Act: Deserialize from JSON
    let result: BenchmarkResult =
        serde_json::from_str(json).expect("Failed to deserialize");

    // Assert: Verify fields
    assert_eq!(result.benchmark_id, "code-completion-basic-001");
    assert_eq!(result.tool_name, "claude");
    assert_eq!(result.scenario_version, "1.0.0");
    assert_eq!(result.execution_time_ms, 1234);
    assert!(result.passed);
    assert_eq!(result.score, 9.5);
    assert!(result.validation_details.passed);
    assert_eq!(result.validation_details.test_case_results.len(), 1);
}

#[test]
fn test_export_json_creates_file() {
    // Arrange: Create temporary directory and benchmark result
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let result = create_sample_result();

    // Act: Export to JSON
    let exported_path = result
        .export_json(temp_dir.path())
        .expect("Failed to export JSON");

    // Assert: File exists and is readable
    assert!(exported_path.exists());
    let content = fs::read_to_string(&exported_path).expect("Failed to read file");
    assert!(content.contains("code-completion-basic-001"));
}

#[test]
fn test_export_json_filename_format() {
    // Arrange: Create temporary directory and benchmark result
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let result = create_sample_result();

    // Act: Export to JSON
    let exported_path = result
        .export_json(temp_dir.path())
        .expect("Failed to export JSON");

    // Assert: Filename matches expected format: {benchmark_id}_{tool_name}_{timestamp}.json
    let filename = exported_path
        .file_name()
        .and_then(|n| n.to_str())
        .expect("Invalid filename");

    assert!(filename.starts_with("code-completion-basic-001_claude_"));
    assert!(filename.ends_with(".json"));
    assert!(filename.contains("2025"));
}

#[test]
fn test_validation_result_structure() {
    // Arrange: Create validation result with multiple test cases
    let validation = ValidationResult {
        passed: false,
        score: 6.5,
        test_case_results: vec![
            TestCaseResult {
                test_name: "test_1".to_string(),
                passed: true,
                expected: "expected_1".to_string(),
                actual: "expected_1".to_string(),
                error: None,
            },
            TestCaseResult {
                test_name: "test_2".to_string(),
                passed: false,
                expected: "expected_2".to_string(),
                actual: "actual_2".to_string(),
                error: Some("Mismatch".to_string()),
            },
        ],
        errors: vec!["Validation error".to_string()],
    };

    // Act: Serialize to JSON
    let json = serde_json::to_string_pretty(&validation).expect("Failed to serialize");

    // Assert: Verify structure
    assert!(json.contains("\"passed\": false"));
    assert!(json.contains("\"score\": 6.5"));
    assert!(json.contains("\"test_case_results\""));
    assert!(json.contains("\"test_1\""));
    assert!(json.contains("\"test_2\""));
    assert!(json.contains("\"errors\""));
    assert!(json.contains("\"Validation error\""));
}

#[test]
fn test_export_json_creates_output_directory() {
    // Arrange: Create path to non-existent directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let non_existent = temp_dir.path().join("nested").join("output");
    let result = create_sample_result();

    // Act: Export to JSON (should create directory)
    let exported_path = result
        .export_json(&non_existent)
        .expect("Failed to export JSON");

    // Assert: Directory and file exist
    assert!(non_existent.exists());
    assert!(exported_path.exists());
}

// Helper function to create sample result for testing
fn create_sample_result() -> BenchmarkResult {
    BenchmarkResult {
        benchmark_id: "code-completion-basic-001".to_string(),
        tool_name: "claude".to_string(),
        scenario_version: "1.0.0".to_string(),
        execution_timestamp: "2025-10-05T01:30:00Z".to_string(),
        execution_time_ms: 1234,
        passed: true,
        score: 9.5,
        output: "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        validation_details: ValidationResult {
            passed: true,
            score: 9.5,
            test_case_results: vec![TestCaseResult {
                test_name: "pattern_match_addition".to_string(),
                passed: true,
                expected: "a \\+ b".to_string(),
                actual: "a + b".to_string(),
                error: None,
            }],
            errors: vec![],
        },
    }
}

#[test]
fn test_json_output_format_demonstration() {
    // This test demonstrates the actual JSON output format
    let result = create_sample_result();
    let json = serde_json::to_string_pretty(&result).expect("Failed to serialize");
    
    // Print to verify format (visible with --nocapture)
    println!("Generated JSON:");
    println!("{}", json);
    
    // Verify it's valid JSON and contains all required fields
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Invalid JSON");
    
    assert!(parsed["benchmark_id"].is_string());
    assert!(parsed["tool_name"].is_string());
    assert!(parsed["scenario_version"].is_string());
    assert!(parsed["execution_timestamp"].is_string());
    assert!(parsed["execution_time_ms"].is_number());
    assert!(parsed["passed"].is_boolean());
    assert!(parsed["score"].is_number());
    assert!(parsed["output"].is_string());
    assert!(parsed["validation_details"].is_object());
    assert!(parsed["validation_details"]["test_case_results"].is_array());
}
