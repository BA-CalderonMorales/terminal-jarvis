// Evals Export Domain
// Handles exporting evaluation data to various formats (JSON, CSV, Markdown)

use super::evals_data::{Rating, ToolEvaluation};
use anyhow::{Context, Result};
use serde_json;
use std::fs;
use std::path::PathBuf;

/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
    Markdown,
    Html,
}

impl ExportFormat {
    pub fn extension(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Markdown => "md",
            Self::Html => "html",
        }
    }

    /// Parse export format from string (Phase 2 API)
    #[allow(dead_code)]
    pub fn parse_format(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "csv" => Some(Self::Csv),
            "markdown" | "md" => Some(Self::Markdown),
            "html" => Some(Self::Html),
            _ => None,
        }
    }
}

/// Export manager for evaluation data
pub struct ExportManager {
    output_dir: PathBuf,
}

impl Default for ExportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportManager {
    /// Create new export manager with default output directory
    pub fn new() -> Self {
        let output_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".terminal-jarvis")
            .join("evals_exports");

        Self { output_dir }
    }

    /// Create export manager with custom output directory
    pub fn with_output_dir(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    /// Export a single tool evaluation
    pub fn export_tool_evaluation(
        &self,
        evaluation: &ToolEvaluation,
        format: ExportFormat,
        filename: Option<&str>,
    ) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir).context("Failed to create export output directory")?;

        let default_filename = format!(
            "eval_{}_{}",
            evaluation.tool_name, evaluation.evaluation_date
        );
        let filename = filename.unwrap_or(&default_filename);
        let output_path = self
            .output_dir
            .join(format!("{}.{}", filename, format.extension()));

        let content = match format {
            ExportFormat::Json => self.export_json(evaluation)?,
            ExportFormat::Csv => self.export_csv_single(evaluation)?,
            ExportFormat::Markdown => self.export_markdown_single(evaluation)?,
            ExportFormat::Html => self.export_html_single(evaluation)?,
        };

        fs::write(&output_path, content)
            .context(format!("Failed to write export to {:?}", output_path))?;

        Ok(output_path)
    }

    /// Export multiple tool evaluations for comparison
    pub fn export_comparison(
        &self,
        evaluations: &[ToolEvaluation],
        format: ExportFormat,
        filename: Option<&str>,
    ) -> Result<PathBuf> {
        fs::create_dir_all(&self.output_dir).context("Failed to create export output directory")?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let days = now / 86400;
        let years = 1970 + days / 365;
        let year_days = days % 365;
        let month = (year_days / 30).min(11) + 1;
        let day = (year_days % 30) + 1;
        let default_filename = format!("comparison_{:04}{:02}{:02}", years, month, day);
        let filename = filename.unwrap_or(&default_filename);
        let output_path = self
            .output_dir
            .join(format!("{}.{}", filename, format.extension()));

        let content = match format {
            ExportFormat::Json => self.export_json_multiple(evaluations)?,
            ExportFormat::Csv => self.export_csv_comparison(evaluations)?,
            ExportFormat::Markdown => self.export_markdown_comparison(evaluations)?,
            ExportFormat::Html => self.export_html_comparison(evaluations)?,
        };

        fs::write(&output_path, content).context(format!(
            "Failed to write comparison export to {:?}",
            output_path
        ))?;

        Ok(output_path)
    }

    /// Export to JSON format
    fn export_json(&self, evaluation: &ToolEvaluation) -> Result<String> {
        serde_json::to_string_pretty(evaluation).context("Failed to serialize evaluation to JSON")
    }

    /// Export multiple evaluations to JSON
    fn export_json_multiple(&self, evaluations: &[ToolEvaluation]) -> Result<String> {
        serde_json::to_string_pretty(evaluations).context("Failed to serialize evaluations to JSON")
    }

    /// Export single evaluation to CSV format
    fn export_csv_single(&self, evaluation: &ToolEvaluation) -> Result<String> {
        let mut csv = String::new();

        // Header
        csv.push_str("Tool,Version,Date,Evaluator,Category,Score,Rating,Findings\n");

        // Data rows
        for category in evaluation.categories.values() {
            let score = category
                .score
                .map(|s| s.to_string())
                .unwrap_or_else(|| "N/A".to_string());
            let findings = category.findings.join("; ");

            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                evaluation.tool_name,
                evaluation.evaluated_version,
                evaluation.evaluation_date,
                evaluation.evaluator,
                category.category_name,
                score,
                category.rating.to_string(),
                findings
            ));
        }

        Ok(csv)
    }

    /// Export comparison to CSV format
    fn export_csv_comparison(&self, evaluations: &[ToolEvaluation]) -> Result<String> {
        let mut csv = String::new();

        // Collect all category IDs
        let mut all_categories = std::collections::HashSet::new();
        for eval in evaluations {
            for category_id in eval.categories.keys() {
                all_categories.insert(category_id.clone());
            }
        }
        let mut categories: Vec<_> = all_categories.into_iter().collect();
        categories.sort();

        // Header
        csv.push_str("Tool,Version,Overall Score");
        for category_id in &categories {
            csv.push_str(&format!(",{}", category_id));
        }
        csv.push('\n');

        // Data rows
        for eval in evaluations {
            let overall = eval
                .overall_score
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".to_string());

            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\"",
                eval.tool_name, eval.evaluated_version, overall
            ));

            for category_id in &categories {
                let score = eval
                    .categories
                    .get(category_id)
                    .and_then(|c| c.score)
                    .map(|s| format!("{:.2}", s))
                    .unwrap_or_else(|| "N/A".to_string());
                csv.push_str(&format!(",\"{}\"", score));
            }
            csv.push('\n');
        }

        Ok(csv)
    }

    /// Export single evaluation to Markdown format
    fn export_markdown_single(&self, evaluation: &ToolEvaluation) -> Result<String> {
        let mut md = String::new();

        // Title
        md.push_str(&format!(
            "# Evaluation: {}\n\n",
            evaluation.tool_display_name
        ));

        // Metadata
        md.push_str("## Metadata\n\n");
        md.push_str(&format!("- **Tool**: {}\n", evaluation.tool_name));
        md.push_str(&format!(
            "- **Version**: {}\n",
            evaluation.evaluated_version
        ));
        md.push_str(&format!("- **Date**: {}\n", evaluation.evaluation_date));
        md.push_str(&format!("- **Evaluator**: {}\n", evaluation.evaluator));
        if let Some(score) = evaluation.overall_score {
            md.push_str(&format!("- **Overall Score**: {:.2}/10\n", score));
        }
        md.push('\n');

        // Summary
        if !evaluation.summary.is_empty() {
            md.push_str("## Summary\n\n");
            md.push_str(&evaluation.summary);
            md.push_str("\n\n");
        }

        // Categories
        md.push_str("## Evaluation Categories\n\n");

        let mut categories: Vec<_> = evaluation.categories.iter().collect();
        categories.sort_by_key(|(id, _)| id.as_str());

        for (_category_id, category) in categories {
            md.push_str(&format!("### {}\n\n", category.category_name));

            if let Some(score) = category.score {
                md.push_str(&format!(
                    "**Score**: {:.2}/10 ({})\n\n",
                    score,
                    category.rating.to_string()
                ));
            }

            if !category.strengths.is_empty() {
                md.push_str("**Strengths**:\n");
                for strength in &category.strengths {
                    md.push_str(&format!("- {}\n", strength));
                }
                md.push('\n');
            }

            if !category.weaknesses.is_empty() {
                md.push_str("**Weaknesses**:\n");
                for weakness in &category.weaknesses {
                    md.push_str(&format!("- {}\n", weakness));
                }
                md.push('\n');
            }

            if !category.findings.is_empty() {
                md.push_str("**Findings**:\n");
                for finding in &category.findings {
                    md.push_str(&format!("- {}\n", finding));
                }
                md.push('\n');
            }

            if !category.evidence.is_empty() {
                md.push_str("**Evidence**:\n");
                for evidence in &category.evidence {
                    md.push_str(&format!("- {}\n", evidence));
                }
                md.push('\n');
            }
        }

        // Notes
        if !evaluation.notes.is_empty() {
            md.push_str("## Additional Notes\n\n");
            for note in &evaluation.notes {
                md.push_str(&format!("- {}\n", note));
            }
            md.push('\n');
        }

        Ok(md)
    }

    /// Export comparison to Markdown format
    fn export_markdown_comparison(&self, evaluations: &[ToolEvaluation]) -> Result<String> {
        let mut md = String::new();

        // Title
        md.push_str("# AI Coding Tools Comparison\n\n");
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
        md.push_str(&format!(
            "**Generated**: {:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC\n\n",
            years, month, day, hours, minutes, seconds
        ));

        // Collect all categories
        let mut all_categories = std::collections::HashMap::new();
        for eval in evaluations {
            for (category_id, category) in &eval.categories {
                all_categories
                    .entry(category_id.clone())
                    .or_insert_with(|| category.category_name.clone());
            }
        }
        let mut categories: Vec<_> = all_categories.into_iter().collect();
        categories.sort_by_key(|(id, _)| id.clone());

        // Overall scores table
        md.push_str("## Overall Scores\n\n");
        md.push_str("| Tool | Version | Overall Score | Rating |\n");
        md.push_str("|------|---------|---------------|--------|\n");

        for eval in evaluations {
            let score = eval
                .overall_score
                .map(|s| format!("{:.2}/10", s))
                .unwrap_or_else(|| "N/A".to_string());
            let rating = eval
                .overall_score
                .map(|s| Rating::from_score(s).to_string().to_owned())
                .unwrap_or_else(|| "N/A".to_string());
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                eval.tool_display_name, eval.evaluated_version, score, rating
            ));
        }
        md.push('\n');

        // Category comparison tables
        for (category_id, category_name) in &categories {
            md.push_str(&format!("## {}\n\n", category_name));
            md.push_str("| Tool | Score | Rating | Key Findings |\n");
            md.push_str("|------|-------|--------|---------------|\n");

            for eval in evaluations {
                if let Some(category) = eval.categories.get(category_id) {
                    let score = category
                        .score
                        .map(|s| format!("{:.2}/10", s))
                        .unwrap_or_else(|| "N/A".to_string());
                    let findings = if !category.findings.is_empty() {
                        category.findings[0].clone()
                    } else {
                        "No findings".to_string()
                    };
                    md.push_str(&format!(
                        "| {} | {} | {} | {} |\n",
                        eval.tool_display_name,
                        score,
                        category.rating.to_string(),
                        findings
                    ));
                } else {
                    md.push_str(&format!(
                        "| {} | N/A | N/A | Not evaluated |\n",
                        eval.tool_display_name
                    ));
                }
            }
            md.push('\n');
        }

        Ok(md)
    }

    /// Export single evaluation to HTML format
    fn export_html_single(&self, evaluation: &ToolEvaluation) -> Result<String> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!(
            "<title>Evaluation: {}</title>\n",
            evaluation.tool_display_name
        ));
        html.push_str("<style>\n");
        html.push_str(Self::get_html_styles());
        html.push_str("</style>\n</head>\n<body>\n");

        html.push_str(&format!(
            "<h1>Evaluation: {}</h1>\n",
            evaluation.tool_display_name
        ));

        html.push_str("<div class=\"metadata\">\n");
        html.push_str(&format!(
            "<p><strong>Tool:</strong> {}</p>\n",
            evaluation.tool_name
        ));
        html.push_str(&format!(
            "<p><strong>Version:</strong> {}</p>\n",
            evaluation.evaluated_version
        ));
        html.push_str(&format!(
            "<p><strong>Date:</strong> {}</p>\n",
            evaluation.evaluation_date
        ));
        html.push_str(&format!(
            "<p><strong>Evaluator:</strong> {}</p>\n",
            evaluation.evaluator
        ));
        if let Some(score) = evaluation.overall_score {
            html.push_str(&format!(
                "<p><strong>Overall Score:</strong> {:.2}/10</p>\n",
                score
            ));
        }
        html.push_str("</div>\n");

        if !evaluation.summary.is_empty() {
            html.push_str("<div class=\"summary\">\n<h2>Summary</h2>\n");
            html.push_str(&format!("<p>{}</p>\n", evaluation.summary));
            html.push_str("</div>\n");
        }

        html.push_str("<h2>Evaluation Categories</h2>\n");

        let mut categories: Vec<_> = evaluation.categories.iter().collect();
        categories.sort_by_key(|(id, _)| id.as_str());

        for (_category_id, category) in categories {
            html.push_str("<div class=\"category\">\n");
            html.push_str(&format!("<h3>{}</h3>\n", category.category_name));

            if let Some(score) = category.score {
                let rating_class = match category.rating {
                    Rating::Excellent => "excellent",
                    Rating::Good => "good",
                    Rating::Adequate => "adequate",
                    Rating::Poor => "poor",
                    Rating::Inadequate => "inadequate",
                    Rating::NotApplicable => "na",
                };
                html.push_str(&format!(
                    "<p class=\"score {}\">{:.2}/10 ({})</p>\n",
                    rating_class,
                    score,
                    category.rating.to_string()
                ));
            }

            if !category.strengths.is_empty() {
                html.push_str("<h4>Strengths</h4><ul>\n");
                for strength in &category.strengths {
                    html.push_str(&format!("<li>{}</li>\n", strength));
                }
                html.push_str("</ul>\n");
            }

            if !category.weaknesses.is_empty() {
                html.push_str("<h4>Weaknesses</h4><ul>\n");
                for weakness in &category.weaknesses {
                    html.push_str(&format!("<li>{}</li>\n", weakness));
                }
                html.push_str("</ul>\n");
            }

            html.push_str("</div>\n");
        }

        html.push_str("</body>\n</html>");

        Ok(html)
    }

    /// Export comparison to HTML format
    fn export_html_comparison(&self, evaluations: &[ToolEvaluation]) -> Result<String> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<title>AI Coding Tools Comparison</title>\n");
        html.push_str("<style>\n");
        html.push_str(Self::get_html_styles());
        html.push_str("</style>\n</head>\n<body>\n");

        html.push_str("<h1>AI Coding Tools Comparison</h1>\n");
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
        html.push_str(&format!(
            "<p class=\"date\">Generated: {:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC</p>\n",
            years, month, day, hours, minutes, seconds
        ));

        // Overall scores table
        html.push_str("<h2>Overall Scores</h2>\n");
        html.push_str("<table>\n<thead>\n<tr>\n");
        html.push_str("<th>Tool</th><th>Version</th><th>Overall Score</th><th>Rating</th>\n");
        html.push_str("</tr>\n</thead>\n<tbody>\n");

        for eval in evaluations {
            let score = eval
                .overall_score
                .map(|s| format!("{:.2}/10", s))
                .unwrap_or_else(|| "N/A".to_string());
            let rating = eval
                .overall_score
                .map(|s| Rating::from_score(s).to_string().to_owned())
                .unwrap_or_else(|| "N/A".to_string());
            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                eval.tool_display_name, eval.evaluated_version, score, rating
            ));
        }

        html.push_str("</tbody>\n</table>\n");

        html.push_str("</body>\n</html>");

        Ok(html)
    }

    /// Get CSS styles for HTML export
    fn get_html_styles() -> &'static str {
        r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1, h2, h3, h4 {
            color: #2c3e50;
        }
        .metadata, .summary, .category {
            background-color: white;
            padding: 20px;
            margin: 20px 0;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .score {
            font-size: 1.2em;
            font-weight: bold;
            padding: 10px;
            border-radius: 4px;
        }
        .score.excellent { background-color: #d4edda; color: #155724; }
        .score.good { background-color: #cce5ff; color: #004085; }
        .score.adequate { background-color: #fff3cd; color: #856404; }
        .score.poor { background-color: #f8d7da; color: #721c24; }
        .score.inadequate { background-color: #f5c6cb; color: #721c24; }
        table {
            width: 100%;
            border-collapse: collapse;
            background-color: white;
            margin: 20px 0;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        th, td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }
        th {
            background-color: #2c3e50;
            color: white;
        }
        tr:hover {
            background-color: #f5f5f5;
        }
        .date {
            color: #666;
            font-style: italic;
        }
        "#
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evals::evals_data::{CategoryEvaluation, ToolEvaluation};

    #[test]
    fn test_export_format_from_str() {
        assert_eq!(
            ExportFormat::parse_format("json"),
            Some(ExportFormat::Json)
        );
        assert_eq!(
            ExportFormat::parse_format("CSV"),
            Some(ExportFormat::Csv)
        );
        assert_eq!(
            ExportFormat::parse_format("markdown"),
            Some(ExportFormat::Markdown)
        );
        assert_eq!(
            ExportFormat::parse_format("md"),
            Some(ExportFormat::Markdown)
        );
        assert_eq!(
            ExportFormat::parse_format("html"),
            Some(ExportFormat::Html)
        );
        assert_eq!(ExportFormat::parse_format("invalid"), None);
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Json.extension(), "json");
        assert_eq!(ExportFormat::Csv.extension(), "csv");
        assert_eq!(ExportFormat::Markdown.extension(), "md");
        assert_eq!(ExportFormat::Html.extension(), "html");
    }

    #[test]
    fn test_export_json() {
        let manager = ExportManager::new();
        let eval = ToolEvaluation::new(
            "test".to_string(),
            "Test Tool".to_string(),
            "1.0.0".to_string(),
            "Test Evaluator".to_string(),
        );

        let json = manager.export_json(&eval).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("Test Tool"));
    }

    #[test]
    fn test_export_markdown_single() {
        let manager = ExportManager::new();
        let mut eval = ToolEvaluation::new(
            "test".to_string(),
            "Test Tool".to_string(),
            "1.0.0".to_string(),
            "Test Evaluator".to_string(),
        );

        let mut category = CategoryEvaluation::new("Authentication".to_string());
        category.set_score(8.5);
        category.add_strength("Easy to use".to_string());
        category.add_weakness("Limited options".to_string());
        eval.add_category("auth".to_string(), category);

        let markdown = manager.export_markdown_single(&eval).unwrap();
        assert!(markdown.contains("# Evaluation: Test Tool"));
        assert!(markdown.contains("Authentication"));
        assert!(markdown.contains("Easy to use"));
        assert!(markdown.contains("Limited options"));
    }
}
