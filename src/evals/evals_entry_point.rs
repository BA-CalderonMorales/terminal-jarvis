// Evals Entry Point Domain
// Main API for the evaluation system

use super::evals_data::{ComparisonResult, ToolEvaluation};
use super::evals_export::{ExportFormat, ExportManager};
use super::evals_scoring::{EvaluationStatistics, Recommendation, ScoringEngine};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main evaluation manager - central API for the Evals system
pub struct EvalManager {
    export_manager: ExportManager,
    evaluations: HashMap<String, ToolEvaluation>,
    config_dir: PathBuf,
}

impl Default for EvalManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalManager {
    /// Create a new EvalManager
    pub fn new() -> Self {
        let export_dir = dirs::home_dir()
            .map(|p| p.join(".terminal-jarvis").join("evals_exports"))
            .unwrap_or_else(|| PathBuf::from("./evals_exports"));

        let config_dir = PathBuf::from("./config/evals");

        Self {
            export_manager: ExportManager::with_output_dir(export_dir),
            evaluations: HashMap::new(),
            config_dir,
        }
    }

    /// Get default config directory
    #[allow(dead_code)]
    fn get_default_config_dir() -> PathBuf {
        let possible_paths = vec![
            dirs::config_dir().map(|p| p.join("terminal-jarvis").join("evals")),
            Some(PathBuf::from("./config/evals")),
        ];

        for path in possible_paths.into_iter().flatten() {
            if path.exists() {
                return path;
            }
        }

        // Default to local config if nothing exists
        PathBuf::from("./config/evals")
    }

    /// Load all evaluations from the configuration directory
    pub fn load_evaluations(&mut self) -> Result<()> {
        let eval_dir = self.config_dir.join("evaluations");

        if !eval_dir.exists() {
            return Ok(()); // No evaluations to load
        }

        for entry in std::fs::read_dir(&eval_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                match self.load_evaluation_file(&path) {
                    Ok(eval) => {
                        self.evaluations.insert(eval.tool_name.clone(), eval);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to load evaluation from {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Load a single evaluation from a TOML file
    fn load_evaluation_file(&self, path: &Path) -> Result<ToolEvaluation> {
        let content = std::fs::read_to_string(path)
            .context(format!("Failed to read evaluation file: {:?}", path))?;

        let eval: ToolEvaluation = toml::from_str(&content)
            .context(format!("Failed to parse evaluation TOML: {:?}", path))?;

        Ok(eval)
    }

    /// Get an evaluation for a specific tool
    pub fn get_evaluation(&self, tool_name: &str) -> Option<&ToolEvaluation> {
        self.evaluations.get(tool_name)
    }

    /// Get all loaded evaluations
    pub fn get_all_evaluations(&self) -> Vec<&ToolEvaluation> {
        self.evaluations.values().collect()
    }

    /// Add an evaluation to the manager (Phase 2 API)
    #[allow(dead_code)]
    pub fn add_evaluation(&mut self, evaluation: ToolEvaluation) {
        self.evaluations
            .insert(evaluation.tool_name.clone(), evaluation);
    }

    /// Compare multiple tools and generate a comparison result
    pub fn compare_tools(&self, tool_names: &[String]) -> Result<ComparisonResult> {
        let evaluations: Vec<&ToolEvaluation> = tool_names
            .iter()
            .filter_map(|name| self.evaluations.get(name))
            .collect();

        if evaluations.is_empty() {
            anyhow::bail!("No evaluations found for the specified tools");
        }

        let owned_evaluations: Vec<ToolEvaluation> =
            evaluations.iter().map(|&e| e.clone()).collect();
        ScoringEngine::compare_tools(&owned_evaluations)
    }

    /// Export an evaluation to a specific format
    pub fn export_evaluation(
        &self,
        tool_name: &str,
        format: ExportFormat,
        filename: Option<&str>,
    ) -> Result<PathBuf> {
        let evaluation = self
            .evaluations
            .get(tool_name)
            .context(format!("No evaluation found for tool: {}", tool_name))?;

        self.export_manager
            .export_tool_evaluation(evaluation, format, filename)
    }

    /// Export a comparison of multiple tools
    pub fn export_comparison(
        &self,
        tool_names: &[String],
        format: ExportFormat,
        filename: Option<&str>,
    ) -> Result<PathBuf> {
        let evaluations: Vec<&ToolEvaluation> = tool_names
            .iter()
            .filter_map(|name| self.evaluations.get(name))
            .collect();

        if evaluations.is_empty() {
            anyhow::bail!("No evaluations found for the specified tools");
        }

        let owned_evaluations: Vec<ToolEvaluation> =
            evaluations.iter().map(|&e| e.clone()).collect();
        self.export_manager
            .export_comparison(&owned_evaluations, format, filename)
    }

    /// Calculate statistics for all loaded evaluations
    pub fn calculate_statistics(&self) -> EvaluationStatistics {
        let evaluations: Vec<ToolEvaluation> = self.evaluations.values().cloned().collect();
        ScoringEngine::calculate_statistics(&evaluations)
    }

    /// Generate tool recommendations
    pub fn generate_recommendations(&self) -> Vec<Recommendation> {
        let evaluations: Vec<ToolEvaluation> = self.evaluations.values().cloned().collect();
        ScoringEngine::generate_recommendations(&evaluations)
    }

    /// Find category leaders across all tools
    pub fn find_category_leaders(&self) -> HashMap<String, Vec<(String, f64)>> {
        let evaluations: Vec<ToolEvaluation> = self.evaluations.values().cloned().collect();
        ScoringEngine::find_category_leaders(&evaluations)
    }

    /// Get a summary of all evaluations
    pub fn get_summary(&self) -> EvaluationSummary {
        let total_evaluations = self.evaluations.len();
        let tools: Vec<String> = self.evaluations.keys().cloned().collect();

        EvaluationSummary {
            total_evaluations,
            tools,
        }
    }

    /// Check if evaluations exist for all integrated tools
    pub fn check_coverage(&self, integrated_tools: &[String]) -> CoverageReport {
        let evaluated_tools: Vec<String> = integrated_tools
            .iter()
            .filter(|tool| self.evaluations.contains_key(*tool))
            .cloned()
            .collect();

        let missing_tools: Vec<String> = integrated_tools
            .iter()
            .filter(|tool| !self.evaluations.contains_key(*tool))
            .cloned()
            .collect();

        let coverage_percentage = if !integrated_tools.is_empty() {
            (evaluated_tools.len() as f64 / integrated_tools.len() as f64) * 100.0
        } else {
            0.0
        };

        CoverageReport {
            total_tools: integrated_tools.len(),
            evaluated_tools: evaluated_tools.len(),
            missing_evaluations: missing_tools,
            coverage_percentage,
        }
    }

    /// Validate that all evaluations have complete data
    pub fn validate_evaluations(&self) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for (tool_name, evaluation) in &self.evaluations {
            // Determine if this is a metrics-only evaluation
            let is_metrics_only = evaluation.metrics.is_some() && evaluation.categories.is_empty();

            // Check for missing overall score (only for traditional evaluations)
            if evaluation.overall_score.is_none() && !is_metrics_only {
                issues.push(ValidationIssue {
                    tool_name: tool_name.clone(),
                    severity: IssueSeverity::Warning,
                    message: "Missing overall score".to_string(),
                });
            }

            // Check for empty categories (only error if no metrics either)
            if evaluation.categories.is_empty() && evaluation.metrics.is_none() {
                issues.push(ValidationIssue {
                    tool_name: tool_name.clone(),
                    severity: IssueSeverity::Error,
                    message: "No categories evaluated and no metrics provided".to_string(),
                });
            }

            // Check for missing summary
            if evaluation.summary.is_empty() {
                issues.push(ValidationIssue {
                    tool_name: tool_name.clone(),
                    severity: IssueSeverity::Info,
                    message: "Missing evaluation summary".to_string(),
                });
            }

            // Check for categories without scores
            for (category_id, category) in &evaluation.categories {
                if category.score.is_none() {
                    issues.push(ValidationIssue {
                        tool_name: tool_name.clone(),
                        severity: IssueSeverity::Warning,
                        message: format!("Category '{}' has no score", category_id),
                    });
                }
            }
        }

        issues
    }
}

/// Summary of all evaluations
#[derive(Debug, Clone)]
pub struct EvaluationSummary {
    pub total_evaluations: usize,
    pub tools: Vec<String>,
}

/// Coverage report for tool evaluations
#[derive(Debug, Clone)]
pub struct CoverageReport {
    pub total_tools: usize,
    pub evaluated_tools: usize,
    pub missing_evaluations: Vec<String>,
    pub coverage_percentage: f64,
}

/// Validation issue found in evaluations
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub tool_name: String,
    pub severity: IssueSeverity,
    pub message: String,
}

/// Severity level for validation issues
#[derive(Debug, Clone, PartialEq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Info => write!(f, "INFO"),
            IssueSeverity::Warning => write!(f, "WARNING"),
            IssueSeverity::Error => write!(f, "ERROR"),
        }
    }
}
