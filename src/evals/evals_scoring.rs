// Evals Scoring Domain
// Handles scoring logic, comparisons, and ranking of tools

use super::evals_data::{ComparisonResult, RankingEntry, Rating, ToolEvaluation};
use anyhow::Result;
use std::collections::HashMap;

/// Scoring and comparison engine
pub struct ScoringEngine;

impl ScoringEngine {
    /// Compare multiple tools and generate rankings
    pub fn compare_tools(evaluations: &[ToolEvaluation]) -> Result<ComparisonResult> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let days = now / 86400;
        let years = 1970 + days / 365;
        let year_days = days % 365;
        let month = (year_days / 30).min(11) + 1;
        let day = (year_days % 30) + 1;
        let hours = (now % 86400) / 3600;
        let minutes = (now % 3600) / 60;
        let seconds = now % 60;
        let comparison_id = format!(
            "comparison_{:04}{:02}{:02}_{:02}{:02}{:02}",
            years, month, day, hours, minutes, seconds
        );
        let comparison_date = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            years, month, day, hours, minutes, seconds
        );

        let tools_compared: Vec<String> = evaluations
            .iter()
            .map(|e| e.tool_display_name.clone())
            .collect();

        // Collect all categories across all evaluations
        let mut all_categories = std::collections::HashSet::new();
        for eval in evaluations {
            for category_id in eval.categories.keys() {
                all_categories.insert(category_id.clone());
            }
        }
        let categories: Vec<String> = all_categories.into_iter().collect();

        // Generate rankings for each category
        let mut rankings: HashMap<String, Vec<RankingEntry>> = HashMap::new();
        for category_id in &categories {
            let category_rankings = Self::rank_by_category(evaluations, category_id);
            rankings.insert(category_id.clone(), category_rankings);
        }

        // Generate overall rankings
        let overall_rankings = Self::rank_overall(evaluations);
        rankings.insert("overall".to_string(), overall_rankings);

        let summary = Self::generate_comparison_summary(evaluations, &rankings);

        Ok(ComparisonResult {
            comparison_id,
            comparison_date,
            tools_compared,
            categories: categories.clone(),
            rankings,
            summary,
        })
    }

    /// Rank tools by a specific category
    fn rank_by_category(evaluations: &[ToolEvaluation], category_id: &str) -> Vec<RankingEntry> {
        let mut entries: Vec<RankingEntry> = evaluations
            .iter()
            .filter_map(|eval| {
                eval.categories.get(category_id).and_then(|category| {
                    category.score.map(|score| {
                        let key_differentiator = if !category.strengths.is_empty() {
                            category.strengths[0].clone()
                        } else if !category.findings.is_empty() {
                            category.findings[0].clone()
                        } else {
                            "No key differentiator".to_string()
                        };

                        RankingEntry {
                            rank: 0, // Will be set after sorting
                            tool_name: eval.tool_display_name.clone(),
                            score,
                            rating: category.rating.clone(),
                            key_differentiator,
                        }
                    })
                })
            })
            .collect();

        // Sort by score descending
        entries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Assign ranks
        for (i, entry) in entries.iter_mut().enumerate() {
            entry.rank = i + 1;
        }

        entries
    }

    /// Rank tools by overall score
    fn rank_overall(evaluations: &[ToolEvaluation]) -> Vec<RankingEntry> {
        let mut entries: Vec<RankingEntry> = evaluations
            .iter()
            .filter_map(|eval| {
                eval.overall_score.map(|score| {
                    let key_differentiator = if !eval.summary.is_empty() {
                        eval.summary.clone()
                    } else {
                        "See detailed evaluation".to_string()
                    };

                    RankingEntry {
                        rank: 0, // Will be set after sorting
                        tool_name: eval.tool_display_name.clone(),
                        score,
                        rating: Rating::from_score(score),
                        key_differentiator,
                    }
                })
            })
            .collect();

        // Sort by score descending
        entries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Assign ranks
        for (i, entry) in entries.iter_mut().enumerate() {
            entry.rank = i + 1;
        }

        entries
    }

    /// Generate a summary of the comparison
    fn generate_comparison_summary(
        evaluations: &[ToolEvaluation],
        rankings: &HashMap<String, Vec<RankingEntry>>,
    ) -> String {
        let mut summary = String::new();

        summary.push_str(&format!(
            "Comparison of {} AI coding tools.\n\n",
            evaluations.len()
        ));

        // Overall winner
        if let Some(overall_rankings) = rankings.get("overall") {
            if let Some(winner) = overall_rankings.first() {
                summary.push_str(&format!(
                    "Overall Winner: {} ({:.2}/10, {})\n\n",
                    winner.tool_name,
                    winner.score,
                    winner.rating.to_string()
                ));
            }
        }

        // Category leaders
        summary.push_str("Category Leaders:\n");
        for (category_id, category_rankings) in rankings {
            if category_id != "overall" {
                if let Some(leader) = category_rankings.first() {
                    summary.push_str(&format!(
                        "- {}: {} ({:.2}/10)\n",
                        category_id, leader.tool_name, leader.score
                    ));
                }
            }
        }

        summary
    }

    /// Calculate statistical insights for a set of evaluations
    pub fn calculate_statistics(evaluations: &[ToolEvaluation]) -> EvaluationStatistics {
        let scores: Vec<f64> = evaluations.iter().filter_map(|e| e.overall_score).collect();

        let mean = if !scores.is_empty() {
            scores.iter().sum::<f64>() / scores.len() as f64
        } else {
            0.0
        };

        let variance = if !scores.is_empty() {
            let squared_diffs: f64 = scores.iter().map(|s| (s - mean).powi(2)).sum();
            squared_diffs / scores.len() as f64
        } else {
            0.0
        };

        let std_dev = variance.sqrt();

        let min = scores.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let median = if !scores.is_empty() {
            let mut sorted = scores.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 {
                (sorted[mid - 1] + sorted[mid]) / 2.0
            } else {
                sorted[mid]
            }
        } else {
            0.0
        };

        EvaluationStatistics {
            mean,
            median,
            std_dev,
            min,
            max,
            count: scores.len(),
        }
    }

    /// Find tools that excel in specific categories
    pub fn find_category_leaders(
        evaluations: &[ToolEvaluation],
    ) -> HashMap<String, Vec<(String, f64)>> {
        let mut leaders: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        // Collect all categories
        let mut all_categories = std::collections::HashSet::new();
        for eval in evaluations {
            for category_id in eval.categories.keys() {
                all_categories.insert(category_id.clone());
            }
        }

        // For each category, find the top tools
        for category_id in all_categories {
            let mut category_scores: Vec<(String, f64)> = evaluations
                .iter()
                .filter_map(|eval| {
                    eval.categories
                        .get(&category_id)
                        .and_then(|c| c.score)
                        .map(|score| (eval.tool_display_name.clone(), score))
                })
                .collect();

            // Sort by score descending
            category_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Take top 3
            let top_tools: Vec<(String, f64)> = category_scores.into_iter().take(3).collect();
            leaders.insert(category_id, top_tools);
        }

        leaders
    }

    /// Calculate similarity between two tool evaluations (Phase 2 API)
    #[allow(dead_code)]
    pub fn calculate_similarity(eval1: &ToolEvaluation, eval2: &ToolEvaluation) -> f64 {
        let mut total_similarity = 0.0;
        let mut category_count = 0;

        // Compare category scores
        for (category_id, cat1) in &eval1.categories {
            if let Some(cat2) = eval2.categories.get(category_id) {
                if let (Some(score1), Some(score2)) = (cat1.score, cat2.score) {
                    // Calculate similarity as 1.0 - normalized difference
                    let difference = (score1 - score2).abs();
                    let similarity = 1.0 - (difference / 10.0); // Assuming 0-10 scale
                    total_similarity += similarity;
                    category_count += 1;
                }
            }
        }

        if category_count > 0 {
            total_similarity / category_count as f64
        } else {
            0.0
        }
    }

    /// Generate recommendations based on evaluation scores
    pub fn generate_recommendations(evaluations: &[ToolEvaluation]) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        // Find overall best
        if let Some(best) = evaluations
            .iter()
            .max_by(|a, b| a.overall_score.partial_cmp(&b.overall_score).unwrap())
        {
            recommendations.push(Recommendation {
                tool_name: best.tool_display_name.clone(),
                use_case: "General purpose AI coding assistant".to_string(),
                reason: format!(
                    "Highest overall score ({:.2}/10) with balanced performance across categories",
                    best.overall_score.unwrap_or(0.0)
                ),
            });
        }

        recommendations
    }
}

/// Statistical metrics for evaluations
#[derive(Debug, Clone)]
pub struct EvaluationStatistics {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
}

/// Tool recommendation
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub tool_name: String,
    pub use_case: String,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evals::evals_data::CategoryEvaluation;

    #[test]
    fn test_rank_overall() {
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

        let rankings = ScoringEngine::rank_overall(&[eval1, eval2]);

        assert_eq!(rankings.len(), 2);
        assert_eq!(rankings[0].rank, 1);
        assert_eq!(rankings[0].tool_name, "Tool 1");
        assert_eq!(rankings[1].rank, 2);
        assert_eq!(rankings[1].tool_name, "Tool 2");
    }

    #[test]
    fn test_calculate_statistics() {
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

        assert_eq!(stats.mean, 7.0);
        assert_eq!(stats.min, 6.0);
        assert_eq!(stats.max, 8.0);
        assert_eq!(stats.count, 2);
    }

    #[test]
    fn test_calculate_similarity() {
        let mut eval1 = ToolEvaluation::new(
            "tool1".to_string(),
            "Tool 1".to_string(),
            "1.0".to_string(),
            "Evaluator".to_string(),
        );
        let mut cat1 = CategoryEvaluation::new("auth".to_string());
        cat1.set_score(8.0);
        eval1.add_category("auth".to_string(), cat1);

        let mut eval2 = ToolEvaluation::new(
            "tool2".to_string(),
            "Tool 2".to_string(),
            "1.0".to_string(),
            "Evaluator".to_string(),
        );
        let mut cat2 = CategoryEvaluation::new("auth".to_string());
        cat2.set_score(8.0);
        eval2.add_category("auth".to_string(), cat2);

        let similarity = ScoringEngine::calculate_similarity(&eval1, &eval2);
        assert_eq!(similarity, 1.0); // Identical scores
    }
}
