// Evals Data Structures Domain
// Handles evaluation data models and schemas for tool assessments

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete evaluation for a single tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvaluation {
    pub tool_name: String,
    pub tool_display_name: String,
    pub evaluated_version: String,
    pub evaluation_date: String,
    pub evaluator: String,
    pub categories: HashMap<String, CategoryEvaluation>,
    pub overall_score: Option<f64>,
    pub summary: String,
    pub notes: Vec<String>,
}

/// Evaluation data for a single category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryEvaluation {
    pub category_name: String,
    pub score: Option<f64>, // 0.0-10.0 scale, None if not applicable
    pub rating: Rating,
    pub findings: Vec<String>,
    pub metrics: HashMap<String, String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub evidence: Vec<String>, // Citations, references, examples
}

/// Rating scale for qualitative assessments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rating {
    Excellent,   // 9-10
    Good,        // 7-8
    Adequate,    // 5-6
    Poor,        // 3-4
    Inadequate,  // 0-2
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
    Boolean,           // Yes/No
    Numeric,           // Quantitative value
    Scale,             // 1-10 scale
    Categorical,       // Multiple choice
    Qualitative,       // Free text assessment
    Evidence,          // Requires citation/proof
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

impl ToolEvaluation {
    /// Create a new empty evaluation
    pub fn new(
        tool_name: String,
        tool_display_name: String,
        version: String,
        evaluator: String,
    ) -> Self {
        Self {
            tool_name,
            tool_display_name,
            evaluated_version: version,
            evaluation_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            evaluator,
            categories: HashMap::new(),
            overall_score: None,
            summary: String::new(),
            notes: Vec::new(),
        }
    }

    /// Add a category evaluation
    pub fn add_category(&mut self, category_id: String, evaluation: CategoryEvaluation) {
        self.categories.insert(category_id, evaluation);
    }

    /// Calculate overall score from category scores
    pub fn calculate_overall_score(&mut self, criteria: &[EvaluationCriterion]) {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for criterion in criteria {
            if let Some(category_eval) = self.categories.get(&criterion.id) {
                if let Some(score) = category_eval.score {
                    weighted_sum += score * criterion.weight;
                    total_weight += criterion.weight;
                }
            }
        }

        self.overall_score = if total_weight > 0.0 {
            Some(weighted_sum / total_weight)
        } else {
            None
        };
    }
}

impl CategoryEvaluation {
    /// Create a new category evaluation
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

    /// Set score and automatically derive rating
    pub fn set_score(&mut self, score: f64) {
        self.score = Some(score);
        self.rating = Rating::from_score(score);
    }

    /// Add a metric result
    pub fn add_metric(&mut self, metric_id: String, value: String) {
        self.metrics.insert(metric_id, value);
    }

    /// Add finding
    pub fn add_finding(&mut self, finding: String) {
        self.findings.push(finding);
    }

    /// Add strength
    pub fn add_strength(&mut self, strength: String) {
        self.strengths.push(strength);
    }

    /// Add weakness
    pub fn add_weakness(&mut self, weakness: String) {
        self.weaknesses.push(weakness);
    }

    /// Add evidence
    pub fn add_evidence(&mut self, evidence: String) {
        self.evidence.push(evidence);
    }
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
