// Benchmarks Module - Automated Evaluation Framework
//
// This module provides community-driven automated benchmarks for AI coding tools.
// It enables executable, reproducible testing of tool capabilities.
//
// Architecture:
// - BenchmarkTestEnvironment: Isolated test execution (src/evals/benchmarks/test_environment.rs)
// - Scenario: TOML-based benchmark definitions (src/evals/benchmarks/scenario.rs)
// - Results: JSON-exportable results for validation (src/evals/benchmarks/results.rs)
// - Validators: Multiple validation strategies (src/evals/benchmarks/validators/)
//
// Integration Points:
// - CLI: `terminal-jarvis benchmark run/results/compare`
// - TypeScript E2E: npm/terminal-jarvis/tests/benchmarks/
// - JSON Bridge: Rust exports → TypeScript validates
//
// Usage Example:
// ```rust
// use terminal_jarvis::evals::benchmarks::BenchmarkTestEnvironment;
//
// let env = BenchmarkTestEnvironment::new()?;
// // Run benchmark in isolated environment
// ```

pub mod registry;
pub mod results;
pub mod runner;
pub mod scenario;
pub mod test_environment;
pub mod validators;

// Re-export main types (Phase 2 API - intentionally unused in Phase 1)
#[allow(unused_imports)]
pub use registry::BenchmarkRegistry;
#[allow(unused_imports)]
pub use results::{BenchmarkResult, TestCaseResult, ValidationResult};
#[allow(unused_imports)]
pub use runner::BenchmarkRunner;
#[allow(unused_imports)]
pub use scenario::{
    BenchmarkScenario, PromptConfig, ScenarioMetadata, ScoringConfig, ValidationConfig,
};
#[allow(unused_imports)]
pub use test_environment::BenchmarkTestEnvironment;

/// Version of the Benchmarks framework (Phase 2 - not yet used)
#[allow(dead_code)]
pub const BENCHMARKS_VERSION: &str = "0.1.0";
