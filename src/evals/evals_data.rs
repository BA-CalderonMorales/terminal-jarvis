// Evals Data Structures Domain
// Handles evaluation data models and schemas for tool assessments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete evaluation for a single tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvaluation {
    pub tool_name: String,
    pub tool_display_name: String,

    // Optional fields for full evaluations
    #[serde(default)]
    pub evaluated_version: String,
    #[serde(default)]
    pub evaluation_date: String,
    #[serde(default)]
    pub evaluator: String,

    // Categories are optional (for metrics-only evaluations)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub categories: HashMap<String, CategoryEvaluation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_score: Option<f64>,

    #[serde(default)]
    pub summary: String,

    #[serde(default)]
    pub notes: Vec<String>,

    // Real-world verifiable metrics (optional for backward compatibility)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<crate::evals::evals_metrics::ToolMetrics>,
}

impl ToolEvaluation {
    /// Create a new ToolEvaluation (Phase 2 API)
    #[allow(dead_code)]
    pub fn new(
        tool_name: String,
        tool_display_name: String,
        evaluated_version: String,
        evaluator: String,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let days = now / 86400;
        let years = 1970 + days / 365;
        let year_days = days % 365;
        let month = (year_days / 30).min(11) + 1;
        let day = (year_days % 30) + 1;
        let evaluation_date = format!("{years:04}-{month:02}-{day:02}");

        Self {
            tool_name,
            tool_display_name,
            evaluated_version,
            evaluation_date,
            evaluator,
            categories: HashMap::new(),
            overall_score: None,
            summary: String::new(),
            notes: Vec::new(),
            metrics: None,
        }
    }

    /// Add a category evaluation (Phase 2 API)
    #[allow(dead_code)]
    pub fn add_category(&mut self, category_id: String, category: CategoryEvaluation) {
        self.categories.insert(category_id, category);
    }

    /// Calculate overall score from category scores (Phase 2 API)
    #[allow(dead_code)]
    pub fn calculate_overall_score(&mut self, criteria: &[EvaluationCriterion]) {
        let mut total_weighted_score = 0.0;
        let mut total_weight = 0.0;

        for criterion in criteria {
            if let Some(category) = self.categories.get(&criterion.id) {
                if let Some(score) = category.score {
                    total_weighted_score += score * criterion.weight;
                    total_weight += criterion.weight;
                }
            }
        }

        if total_weight > 0.0 {
            self.overall_score = Some(total_weighted_score / total_weight);
        }
    }
}

/// Evaluation data for a single category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryEvaluation {
    pub category_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>, // 0.0-10.0 scale, None if not applicable
    pub rating: Rating,
    #[serde(default)]
    pub findings: Vec<String>,
    #[serde(default)]
    pub metrics: HashMap<String, String>,
    #[serde(default)]
    pub strengths: Vec<String>,
    #[serde(default)]
    pub weaknesses: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<String>, // Citations, references, examples
}

impl CategoryEvaluation {
    /// Create a new CategoryEvaluation (Phase 2 API)
    #[allow(dead_code)]
    pub fn new(category_name: String) -> Self {
        Self {
            category_name,
            score: None,
            rating: Rating::NotApplicable,
            findings: Vec::new(),
            metrics: HashMap::new(),
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            evidence: Vec::new(),
        }
    }

    /// Set the score and update the rating (Phase 2 API)
    #[allow(dead_code)]
    pub fn set_score(&mut self, score: f64) {
        self.score = Some(score);
        self.rating = Rating::from_score(score);
    }

    /// Add a strength to the evaluation (Phase 2 API)
    #[allow(dead_code)]
    pub fn add_strength(&mut self, strength: String) {
        self.strengths.push(strength);
    }

    /// Add a weakness to the evaluation (Phase 2 API)
    #[allow(dead_code)]
    pub fn add_weakness(&mut self, weakness: String) {
        self.weaknesses.push(weakness);
    }
}

/// Rating scale for qualitative assessments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rating {
    Excellent,  // 9-10
    Good,       // 7-8
    Adequate,   // 5-6
    Poor,       // 3-4
    Inadequate, // 0-2
    NotApplicable,
}

impl Rating {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 9.0 => Rating::Excellent,
            s if s >= 7.0 => Rating::Good,
            s if s >= 5.0 => Rating::Adequate,
            s if s >= 3.0 => Rating::Poor,
            _ => Rating::Inadequate,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Rating::Excellent => "Excellent",
            Rating::Good => "Good",
            Rating::Adequate => "Adequate",
            Rating::Poor => "Poor",
            Rating::Inadequate => "Inadequate",
            Rating::NotApplicable => "N/A",
        }
    }
}

/// Evaluation criterion definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCriterion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: f64, // For weighted scoring
    pub metrics: Vec<MetricDefinition>,
    pub is_custom: bool, // X-factor custom criterion
}

/// Individual metric within a criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metric_type: MetricType,
    pub evaluation_guide: String,
}

/// Types of metrics that can be evaluated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Boolean,     // Yes/No
    Numeric,     // Quantitative value
    Scale,       // 1-10 scale
    Categorical, // Multiple choice
    Qualitative, // Free text assessment
    Evidence,    // Requires citation/proof
}

/// Comparison result between multiple tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub comparison_id: String,
    pub comparison_date: String,
    pub tools_compared: Vec<String>,
    pub categories: Vec<String>,
    pub rankings: HashMap<String, Vec<RankingEntry>>, // Category -> Rankings
    pub summary: String,
}

/// Individual ranking entry for a tool in a category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingEntry {
    pub rank: usize,
    pub tool_name: String,
    pub score: f64,
    pub rating: Rating,
    pub key_differentiator: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_from_score() {
        assert_eq!(Rating::from_score(9.5), Rating::Excellent);
        assert_eq!(Rating::from_score(7.5), Rating::Good);
        assert_eq!(Rating::from_score(5.5), Rating::Adequate);
        assert_eq!(Rating::from_score(3.5), Rating::Poor);
        assert_eq!(Rating::from_score(1.0), Rating::Inadequate);
    }

    #[test]
    fn test_tool_evaluation_creation() {
        let eval = ToolEvaluation::new(
            "claude".to_string(),
            "Claude".to_string(),
            "1.0.0".to_string(),
            "Test Evaluator".to_string(),
        );

        assert_eq!(eval.tool_name, "claude");
        assert_eq!(eval.tool_display_name, "Claude");
        assert_eq!(eval.categories.len(), 0);
        assert_eq!(eval.overall_score, None);
    }

    #[test]
    fn test_category_evaluation_score_setting() {
        let mut category = CategoryEvaluation::new("Authentication".to_string());
        category.set_score(8.5);

        assert_eq!(category.score, Some(8.5));
        assert_eq!(category.rating, Rating::Good);
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
}
