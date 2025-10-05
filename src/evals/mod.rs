// Evals Module - Evaluation and Comparison Framework
//
// This module provides a comprehensive evaluation system for AI coding tools integrated
// with Terminal Jarvis. It enables structured, side-by-side comparison of tools across
// 13 standard criteria plus customizable X-factor categories.
//
// Architecture:
// - Domain-based modular design following Terminal Jarvis patterns
// - Extensible configuration via TOML files
// - Multiple export formats (JSON, CSV, Markdown, HTML)
// - Community-driven X-factor custom categories
//
// Key Components:
// - evals_entry_point: Main EvalManager API
// - evals_criteria: Evaluation category definitions and management
// - evals_data: Core data structures and models
// - evals_export: Export functionality for multiple formats
// - evals_scoring: Scoring, ranking, and comparison logic
//
// Usage Example:
// ```rust
// use terminal_jarvis::evals::EvalManager;
//
// let mut manager = EvalManager::new();
// manager.load_evaluations()?;
//
// // Compare tools
// let comparison = manager.compare_tools(&["claude", "gemini", "qwen"])?;
//
// // Export results
// manager.export_comparison(&["claude", "gemini"], ExportFormat::Markdown, None)?;
// ```

pub mod benchmarks;
pub mod evals_criteria;
pub mod evals_data;
pub mod evals_entry_point;
pub mod evals_export;
pub mod evals_metrics;
pub mod evals_scoring;

// Re-export only the types that are actually used
pub use evals_entry_point::{EvalManager, IssueSeverity};
pub use evals_export::ExportFormat;

/// Version of the Evals framework
pub const EVALS_VERSION: &str = "1.0.0";
