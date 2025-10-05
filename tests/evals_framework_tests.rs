// Evals Framework Integration Tests

use terminal_jarvis::evals::evals_criteria::CriteriaManager;
use terminal_jarvis::evals::evals_data::{
    CategoryEvaluation, EvaluationCriterion, Rating, ToolEvaluation,
};
use terminal_jarvis::evals::evals_scoring::ScoringEngine;
use terminal_jarvis::evals::*;

#[test]
fn test_eval_manager_creation() {
    let manager = EvalManager::new();
    let summary = manager.get_summary();
    assert_eq!(summary.total_evaluations, 0);
}

#[test]
fn test_tool_evaluation_creation() {
    let eval = ToolEvaluation::new(
        "test_tool".to_string(),
        "Test Tool".to_string(),
        "1.0.0".to_string(),
        "Test Evaluator".to_string(),
    );

    assert_eq!(eval.tool_name, "test_tool");
    assert_eq!(eval.tool_display_name, "Test Tool");
    assert_eq!(eval.evaluated_version, "1.0.0");
    assert!(eval.overall_score.is_none());
}

#[test]
fn test_category_evaluation() {
    let mut category = CategoryEvaluation::new("Authentication".to_string());
    category.set_score(8.5);

    assert_eq!(category.score, Some(8.5));
    assert_eq!(category.rating, Rating::Good);

    category.add_strength("Easy to use".to_string());
    category.add_weakness("Limited options".to_string());

    assert_eq!(category.strengths.len(), 1);
    assert_eq!(category.weaknesses.len(), 1);
}

#[test]
fn test_overall_score_calculation() {
    let mut eval = ToolEvaluation::new(
        "test".to_string(),
        "Test".to_string(),
        "1.0".to_string(),
        "Evaluator".to_string(),
    );

    let mut cat1 = CategoryEvaluation::new("Cat1".to_string());
    cat1.set_score(8.0);
    eval.add_category("cat1".to_string(), cat1);

    let mut cat2 = CategoryEvaluation::new("Cat2".to_string());
    cat2.set_score(6.0);
    eval.add_category("cat2".to_string(), cat2);

    let criteria = vec![
        EvaluationCriterion {
            id: "cat1".to_string(),
            name: "Category 1".to_string(),
            description: "Test".to_string(),
            weight: 1.0,
            metrics: vec![],
            is_custom: false,
        },
        EvaluationCriterion {
            id: "cat2".to_string(),
            name: "Category 2".to_string(),
            description: "Test".to_string(),
            weight: 1.0,
            metrics: vec![],
            is_custom: false,
        },
    ];

    eval.calculate_overall_score(&criteria);
    assert_eq!(eval.overall_score, Some(7.0));
}

#[test]
fn test_rating_from_score() {
    assert_eq!(Rating::from_score(9.5), Rating::Excellent);
    assert_eq!(Rating::from_score(7.5), Rating::Good);
    assert_eq!(Rating::from_score(5.5), Rating::Adequate);
    assert_eq!(Rating::from_score(3.5), Rating::Poor);
    assert_eq!(Rating::from_score(1.0), Rating::Inadequate);
}

#[test]
fn test_criteria_manager() {
    let manager = CriteriaManager::new();
    let criteria = manager.get_standard_criteria();

    // Should have 13 standard criteria
    assert_eq!(criteria.len(), 13);

    // Check unique IDs
    let ids: std::collections::HashSet<_> = criteria.iter().map(|c| &c.id).collect();
    assert_eq!(ids.len(), 13);
}

#[test]
fn test_export_format_parsing() {
    assert_eq!(ExportFormat::parse_format("json"), Some(ExportFormat::Json));
    assert_eq!(ExportFormat::parse_format("CSV"), Some(ExportFormat::Csv));
    assert_eq!(
        ExportFormat::parse_format("markdown"),
        Some(ExportFormat::Markdown)
    );
    assert_eq!(ExportFormat::parse_format("invalid"), None);
}

#[test]
fn test_scoring_engine_statistics() {
    let mut eval1 = ToolEvaluation::new(
        "tool1".to_string(),
        "Tool 1".to_string(),
        "1.0".to_string(),
        "Evaluator".to_string(),
    );
    eval1.overall_score = Some(8.0);

    let mut eval2 = ToolEvaluation::new(
        "tool2".to_string(),
        "Tool 2".to_string(),
        "1.0".to_string(),
        "Evaluator".to_string(),
    );
    eval2.overall_score = Some(6.0);

    let stats = ScoringEngine::calculate_statistics(&[eval1, eval2]);

    assert_eq!(stats.count, 2);
    assert_eq!(stats.mean, 7.0);
    assert_eq!(stats.min, 6.0);
    assert_eq!(stats.max, 8.0);
}

#[test]
fn test_tool_comparison() {
    let mut eval1 = ToolEvaluation::new(
        "tool1".to_string(),
        "Tool 1".to_string(),
        "1.0".to_string(),
        "Evaluator".to_string(),
    );
    eval1.overall_score = Some(8.5);

    let mut eval2 = ToolEvaluation::new(
        "tool2".to_string(),
        "Tool 2".to_string(),
        "1.0".to_string(),
        "Evaluator".to_string(),
    );
    eval2.overall_score = Some(7.0);

    let comparison = ScoringEngine::compare_tools(&[eval1, eval2]).unwrap();

    assert_eq!(comparison.tools_compared.len(), 2);
    assert!(comparison.rankings.contains_key("overall"));

    let overall_rankings = &comparison.rankings["overall"];
    assert_eq!(overall_rankings.len(), 2);
    assert_eq!(overall_rankings[0].rank, 1);
    assert_eq!(overall_rankings[0].tool_name, "Tool 1");
}

#[test]
fn test_coverage_report() {
    let mut manager = EvalManager::new();

    let eval = ToolEvaluation::new(
        "claude".to_string(),
        "Claude".to_string(),
        "1.0".to_string(),
        "Evaluator".to_string(),
    );
    manager.add_evaluation(eval);

    let integrated_tools = vec!["claude".to_string(), "gemini".to_string()];
    let coverage = manager.check_coverage(&integrated_tools);

    assert_eq!(coverage.total_tools, 2);
    assert_eq!(coverage.evaluated_tools, 1);
    assert_eq!(coverage.missing_evaluations.len(), 1);
}

#[test]
fn test_validation() {
    let mut manager = EvalManager::new();

    // Add evaluation with missing data
    let eval = ToolEvaluation::new(
        "incomplete".to_string(),
        "Incomplete".to_string(),
        "1.0".to_string(),
        "Evaluator".to_string(),
    );
    manager.add_evaluation(eval);

    let issues = manager.validate_evaluations();
    assert!(!issues.is_empty());
}

#[test]
fn test_all_metrics_files_load_successfully() {
    // This test verifies that all 10 evaluation TOML files can be parsed
    let evaluation_files = vec![
        "config/evals/evaluations/claude-metrics.toml",
        "config/evals/evaluations/gemini-metrics.toml",
        "config/evals/evaluations/qwen-metrics.toml",
        "config/evals/evaluations/opencode-metrics.toml",
        "config/evals/evaluations/aider-metrics.toml",
        "config/evals/evaluations/amp-metrics.toml",
        "config/evals/evaluations/goose-metrics.toml",
        "config/evals/evaluations/llxprt-metrics.toml",
        "config/evals/evaluations/codex-metrics.toml",
        "config/evals/evaluations/crush-metrics.toml",
    ];

    for file_path in evaluation_files {
        let toml_content = std::fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", file_path));

        let result: Result<ToolEvaluation, toml::de::Error> = toml::from_str(&toml_content);

        assert!(
            result.is_ok(),
            "Failed to parse {}: {:?}",
            file_path,
            result.err()
        );

        let evaluation = result.unwrap();
        // Files can have either metrics or categories or both
        assert!(
            evaluation.metrics.is_some() || !evaluation.categories.is_empty(),
            "Evaluation should have either metrics or categories in {}",
            file_path
        );
    }
}

#[test]
fn test_metrics_only_evaluations_are_valid() {
    // Test that metrics-only evaluations (no categories, no overall_score) are considered valid
    let mut manager = EvalManager::new();

    // Load the actual metrics files to test real-world scenarios
    manager.load_evaluations().expect("Failed to load evaluations");

    let issues = manager.validate_evaluations();

    // Filter for errors only (metrics-only evaluations should not trigger errors)
    let errors: Vec<_> = issues.iter().filter(|i| matches!(i.severity, terminal_jarvis::evals::IssueSeverity::Error)).collect();

    // There should be no errors for metrics-only evaluations
    assert!(
        errors.is_empty(),
        "Metrics-only evaluations should not trigger validation errors. Found: {:?}",
        errors
    );
}
