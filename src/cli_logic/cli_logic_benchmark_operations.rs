// CLI Logic - Benchmark Operations Domain
// Handles all user interactions with the Benchmarks framework

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::evals::benchmarks::{BenchmarkRegistry, BenchmarkScenario};

/// List all available benchmark scenarios
pub async fn handle_benchmark_list() -> Result<()> {
    let registry = BenchmarkRegistry::from_directory("config/benchmarks/scenarios")
        .context("Failed to load benchmark scenarios")?;

    let scenarios = registry.list_scenarios();

    if scenarios.is_empty() {
        println!("\n[INFO] No benchmark scenarios found.");
        println!("       Expected location: config/benchmarks/scenarios/");
        return Ok(());
    }

    println!("\n=== AVAILABLE BENCHMARKS ===\n");

    for scenario in &scenarios {
        println!("  [{}]", scenario.metadata.id);
        println!("    Name: {}", scenario.metadata.name);
        println!(
            "    Category: {} | Difficulty: {}",
            scenario.metadata.category, scenario.metadata.difficulty
        );
        println!();
    }

    println!("  Total scenarios: {}\n", scenarios.len());

    Ok(())
}

/// Run a benchmark scenario against a tool
pub async fn handle_benchmark_run(
    scenario_id: &str,
    tool_name: &str,
    export_json: Option<&PathBuf>,
) -> Result<()> {
    use crate::evals::benchmarks::BenchmarkRunner;

    println!("\n=== RUNNING BENCHMARK ===\n");
    println!("  Scenario: {}", scenario_id);
    println!("  Tool: {}", tool_name);

    if let Some(export_path) = export_json {
        println!("  Export JSON: {}", export_path.display());
    }

    println!();

    // Load scenario from registry
    let registry = BenchmarkRegistry::from_directory("config/benchmarks/scenarios")
        .context("Failed to load benchmark scenarios")?;

    let scenario = registry
        .get_scenario(scenario_id)
        .ok_or_else(|| anyhow::anyhow!("Scenario not found: {}", scenario_id))?;

    println!("[INFO] Loaded scenario: {}", scenario.metadata.name);
    println!("       Category: {}", scenario.metadata.category);
    println!("       Difficulty: {}", scenario.metadata.difficulty);
    println!();

    // Create runner with optional export directory
    let runner = if let Some(path) = export_json {
        BenchmarkRunner::with_output_dir(path.to_string_lossy().to_string())
    } else {
        BenchmarkRunner::new()
    };

    // Execute benchmark
    println!("[EXECUTING] Running benchmark...");
    let result = runner
        .execute(scenario, tool_name)
        .await
        .context("Failed to execute benchmark")?;

    // Display results
    println!("\n=== RESULTS ===\n");
    println!(
        "  Status: {}",
        if result.passed { "[PASS]" } else { "[FAIL]" }
    );
    println!("  Score: {:.1}/10.0", result.score);
    println!("  Execution Time: {}ms", result.execution_time_ms);
    println!(
        "  Validation: {}/{} tests passed",
        result
            .validation_details
            .test_case_results
            .iter()
            .filter(|t| t.passed)
            .count(),
        result.validation_details.test_case_results.len()
    );

    // Show validation details if any tests failed
    if !result.passed {
        println!("\n  Failed Tests:");
        for test in &result.validation_details.test_case_results {
            if !test.passed {
                println!("    - {}: {}", test.test_name, test.actual);
            }
        }
    }

    // Show export location if specified
    if let Some(path) = export_json {
        println!("\n[EXPORTED] Results saved to: {}", path.display());
    }

    println!();

    Ok(())
}

/// Validate a benchmark scenario file
pub async fn handle_benchmark_validate(scenario_file: &PathBuf) -> Result<()> {
    println!("\n=== VALIDATING SCENARIO FILE ===\n");
    println!("  File: {}\n", scenario_file.display());

    // Load and validate the file
    let content = std::fs::read_to_string(scenario_file)
        .with_context(|| format!("Failed to read file: {}", scenario_file.display()))?;

    let scenario: BenchmarkScenario = toml::from_str(&content)
        .with_context(|| format!("Failed to parse TOML in file: {}", scenario_file.display()))?;

    // Perform validation checks
    let mut warnings = Vec::new();

    if scenario.metadata.id.is_empty() {
        warnings.push("Scenario ID is empty");
    }

    if scenario.metadata.name.is_empty() {
        warnings.push("Scenario name is empty");
    }

    if scenario.prompt.template.is_empty() {
        warnings.push("Prompt template is empty");
    }

    if scenario.scoring.pass_threshold < 0.0 || scenario.scoring.pass_threshold > 1.0 {
        warnings.push("Pass threshold should be between 0.0 and 1.0");
    }

    if !warnings.is_empty() {
        println!("[WARNING] Validation issues found:");
        for warning in warnings {
            println!("  - {}", warning);
        }
        println!();
    }

    println!("[SUCCESS] Scenario file is valid TOML");
    println!("\n  Scenario Details:");
    println!("    ID: {}", scenario.metadata.id);
    println!("    Name: {}", scenario.metadata.name);
    println!("    Category: {}", scenario.metadata.category);
    println!("    Difficulty: {}", scenario.metadata.difficulty);
    println!("    Version: {}", scenario.metadata.version);
    println!("    Points Possible: {}", scenario.scoring.points_possible);
    println!(
        "    Pass Threshold: {:.0}%",
        scenario.scoring.pass_threshold * 100.0
    );
    println!();

    Ok(())
}
