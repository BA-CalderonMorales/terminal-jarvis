// CLI Logic - Evals Operations Domain
// Handles all user interactions with the Evals framework

use crate::evals::{EvalManager, ExportFormat, IssueSeverity, EVALS_VERSION};
use crate::theme::theme_global_config;
use anyhow::{Context, Result};
use std::io::{self, Write};

/// Display the Evals main menu and handle user choices
pub fn show_evals_menu() -> Result<()> {
    let theme = theme_global_config::current_theme();

    loop {
        println!("\n{}", "=".repeat(70));
        println!(
            "{}",
            theme.primary("  EVALS & COMPARISONS - AI Coding Tools Evaluation")
        );
        println!("{}", "=".repeat(70));
        println!();
        println!("  {}  View All Evaluations", theme.accent("1."));
        println!("  {}  Compare Tools", theme.accent("2."));
        println!("  {}  View Tool Details", theme.accent("3."));
        println!("  {}  Export Evaluations", theme.accent("4."));
        println!("  {}  Statistics & Insights", theme.accent("5."));
        println!("  {}  Coverage Report", theme.accent("6."));
        println!("  {}  Validate Evaluations", theme.accent("7."));
        println!("  {}  About Evals Framework", theme.accent("8."));
        println!("  {}  Return to Main Menu", theme.accent("0."));
        println!();
        print!("  {} ", theme.primary("Choice:"));
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => view_all_evaluations()?,
            "2" => compare_tools_interactive()?,
            "3" => view_tool_details_interactive()?,
            "4" => export_evaluations_interactive()?,
            "5" => show_statistics()?,
            "6" => show_coverage_report()?,
            "7" => validate_evaluations()?,
            "8" => show_about()?,
            "0" => break,
            _ => println!("\n  [!] Invalid choice. Please try again."),
        }

        if input.trim() != "0" {
            println!("\n  Press Enter to continue...");
            let mut pause = String::new();
            io::stdin().read_line(&mut pause)?;
        }
    }

    Ok(())
}

/// View all evaluations summary with contextual insights
fn view_all_evaluations() -> Result<()> {
    let theme = theme_global_config::current_theme();
    println!("\n=== ALL EVALUATIONS ===");

    let mut manager = EvalManager::new();
    manager
        .load_evaluations()
        .context("Failed to load evaluations")?;

    let summary = manager.get_summary();

    if summary.total_evaluations == 0 {
        println!("\n  [!] No evaluations found.");
        println!("  Place evaluation files in: config/evals/evaluations/");
        return Ok(());
    }

    println!(
        "\n  {} tools evaluated",
        theme.primary(&summary.total_evaluations.to_string())
    );
    println!("  Evaluations based on verifiable metrics: GitHub activity, community size, docs freshness, support response\n");

    // Display each evaluation with insights
    let mut evaluations: Vec<_> = manager.get_all_evaluations();
    evaluations.sort_by(|a, b| {
        b.overall_score
            .unwrap_or(0.0)
            .partial_cmp(&a.overall_score.unwrap_or(0.0))
            .unwrap()
    });

    for eval in evaluations {
        println!(
            "{}",
            theme.primary(&format!("━━ {} ━━", eval.tool_display_name))
        );

        // Show real-world metrics if available
        if let Some(metrics) = &eval.metrics {
            // GitHub stats
            if let Some(gh) = &metrics.github {
                println!(
                    "  GitHub: {} stars, {} forks, {} contributors",
                    gh.stars.unwrap_or(0),
                    gh.forks.unwrap_or(0),
                    gh.contributors.unwrap_or(0)
                );
                if let Some(last_commit) = &gh.last_commit_date {
                    println!(
                        "  Last commit: {} ({})",
                        last_commit,
                        gh.commit_frequency.as_deref().unwrap_or("Unknown")
                    );
                }
            }

            // Package stats
            if let Some(pkg) = &metrics.package {
                println!(
                    "  Downloads: {} weekly from {}",
                    pkg.weekly_downloads.unwrap_or(0),
                    pkg.registry
                );
            }

            // Community
            if let Some(community) = &metrics.community {
                if let Some(discord) = community.discord_members {
                    println!("  Community: {} Discord members", discord);
                }
            }

            // Documentation
            if let Some(docs) = &metrics.documentation {
                print!("  Documentation: ");
                let mut features = vec![];
                if docs.has_getting_started {
                    features.push("Getting Started");
                }
                if docs.has_api_reference {
                    features.push("API Ref");
                }
                if docs.has_examples {
                    features.push("Examples");
                }
                println!("{}", features.join(", "));
                if let Some(last_update) = &docs.last_docs_update {
                    println!("    Last updated: {}", last_update);
                }
            }

            // Platform support
            println!("  Platforms: {}", metrics.platform.supported_os.join(", "));

            // Team transparency
            if let Some(team) = &metrics.team {
                println!(
                    "  Organization: {} (Team: {}, Public: {})",
                    team.organization_name,
                    team.team_size.as_deref().unwrap_or("Unknown"),
                    if team.public_team { "Yes" } else { "No" }
                );
                if !team.backed_by.is_empty() {
                    println!("  Backed by: {}", team.backed_by.join(", "));
                }
            }

            // Support metrics
            if let Some(support) = &metrics.support {
                println!(
                    "  Support: Response time {}, channels: {}",
                    support.issue_response_time.as_deref().unwrap_or("Unknown"),
                    support.support_channels.join(", ")
                );
            }
        } else {
            // Fallback to summary for old-style evaluations
            println!("  {}", eval.summary);
        }

        println!();
    }

    println!("  Note: Metrics are collected from public sources (GitHub, NPM, Discord, etc.)");
    println!("  Use 'View Tool Details' for complete evaluation data\n");

    Ok(())
}

/// Compare multiple tools
fn compare_tools_interactive() -> Result<()> {
    println!("\n=== COMPARE TOOLS ===");

    let mut manager = EvalManager::new();
    manager
        .load_evaluations()
        .context("Failed to load evaluations")?;

    let summary = manager.get_summary();

    if summary.total_evaluations < 2 {
        println!("\n  [!] At least 2 evaluations needed for comparison.");
        return Ok(());
    }

    // Show available tools
    println!("\n  Available tools:");
    for (i, tool) in summary.tools.iter().enumerate() {
        println!("    {}. {}", i + 1, tool);
    }

    println!("\n  Enter tool numbers to compare (comma-separated, e.g., 1,2,3):");
    print!("  > ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let indices: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .filter(|&i| i > 0 && i <= summary.tools.len())
        .map(|i| i - 1)
        .collect();

    if indices.len() < 2 {
        println!("\n  [!] Please select at least 2 tools.");
        return Ok(());
    }

    let tool_names: Vec<String> = indices.iter().map(|&i| summary.tools[i].clone()).collect();

    println!("\n  Comparing: {}", tool_names.join(", "));

    let comparison = manager
        .compare_tools(&tool_names)
        .context("Failed to generate comparison")?;

    // Display comparison results
    println!("\n=== COMPARISON RESULTS ===");
    println!("\n{}", comparison.summary);

    // Show rankings
    if let Some(overall_rankings) = comparison.rankings.get("overall") {
        println!("\n  Overall Rankings:");
        for entry in overall_rankings {
            let rank_display = format!("{}.", entry.rank);
            let score_display = format!("{:.2}/10", entry.score);

            // Note: Color-coding removed - all scores display the same way
            println!(
                "    {} {:<20} {} ({})",
                rank_display,
                entry.tool_name,
                score_display,
                entry.rating.to_string()
            );
        }
    }

    Ok(())
}

/// View detailed evaluation for a specific tool
fn view_tool_details_interactive() -> Result<()> {
    println!("\n=== TOOL DETAILS ===");

    let mut manager = EvalManager::new();
    manager
        .load_evaluations()
        .context("Failed to load evaluations")?;

    let summary = manager.get_summary();

    if summary.total_evaluations == 0 {
        println!("\n  [!] No evaluations available.");
        return Ok(());
    }

    // Show available tools
    println!("\n  Available tools:");
    for (i, tool) in summary.tools.iter().enumerate() {
        println!("    {}. {}", i + 1, tool);
    }

    print!("\n  Select tool number: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let index: usize = input.trim().parse().unwrap_or(0);

    if index == 0 || index > summary.tools.len() {
        println!("\n  [!] Invalid selection.");
        return Ok(());
    }

    let tool_name = &summary.tools[index - 1];
    let evaluation = manager
        .get_evaluation(tool_name)
        .context("Evaluation not found")?;

    // Display detailed evaluation
    println!("\n{}", "=".repeat(70));
    println!("  {}", evaluation.tool_display_name);
    println!("{}", "=".repeat(70));
    println!();
    println!("  Version: {}", evaluation.evaluated_version);
    println!("  Evaluated: {}", evaluation.evaluation_date);
    println!("  Evaluator: {}", evaluation.evaluator);

    if let Some(score) = evaluation.overall_score {
        let score_str = format!("{:.2}/10", score);
        println!("  Overall Score: {}", score_str);
    }

    println!();
    println!("  Summary:");
    println!("  {}", evaluation.summary);

    println!();
    println!("  Category Scores:");
    println!("  {}", "-".repeat(60));

    let mut categories: Vec<_> = evaluation.categories.iter().collect();
    categories.sort_by_key(|(id, _)| id.as_str());

    for (_category_id, category) in categories {
        let score_str = if let Some(score) = category.score {
            format!("{:.1}/10", score)
        } else {
            "N/A".to_string()
        };

        println!(
            "  {:<35} {} ({})",
            category.category_name,
            score_str,
            category.rating.to_string()
        );
    }

    Ok(())
}

/// Export evaluations to various formats
fn export_evaluations_interactive() -> Result<()> {
    println!("\n=== EXPORT EVALUATIONS ===");

    let mut manager = EvalManager::new();
    manager
        .load_evaluations()
        .context("Failed to load evaluations")?;

    println!("\n  Export Format:");
    println!("    1. JSON");
    println!("    2. CSV");
    println!("    3. Markdown");
    println!("    4. HTML");

    print!("\n  Select format: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let format = match input.trim() {
        "1" => ExportFormat::Json,
        "2" => ExportFormat::Csv,
        "3" => ExportFormat::Markdown,
        "4" => ExportFormat::Html,
        _ => {
            println!("\n  [!] Invalid format.");
            return Ok(());
        }
    };

    println!("\n  Export Type:");
    println!("    1. Single tool evaluation");
    println!("    2. Comparison (multiple tools)");

    print!("\n  Select type: ");
    io::stdout().flush()?;

    let mut type_input = String::new();
    io::stdin().read_line(&mut type_input)?;

    let summary = manager.get_summary();

    match type_input.trim() {
        "1" => {
            // Single tool export
            if summary.total_evaluations == 0 {
                println!("\n  [!] No evaluations available.");
                return Ok(());
            }

            println!("\n  Available tools:");
            for (i, tool) in summary.tools.iter().enumerate() {
                println!("    {}. {}", i + 1, tool);
            }

            print!("\n  Select tool number: ");
            io::stdout().flush()?;

            let mut tool_input = String::new();
            io::stdin().read_line(&mut tool_input)?;

            let index: usize = tool_input.trim().parse().unwrap_or(0);

            if index == 0 || index > summary.tools.len() {
                println!("\n  [!] Invalid selection.");
                return Ok(());
            }

            let tool_name = &summary.tools[index - 1];
            let path = manager.export_evaluation(tool_name, format, None)?;

            println!("\n  [DONE] Evaluation exported successfully!");
            println!("  Location: {}", path.display());
        }
        "2" => {
            // Comparison export
            if summary.total_evaluations < 2 {
                println!("\n  [!] At least 2 evaluations needed.");
                return Ok(());
            }

            let tool_names = summary.tools.clone();
            let path = manager.export_comparison(&tool_names, format, None)?;

            println!("\n  [DONE] Comparison exported successfully!");
            println!("  Location: {}", path.display());
        }
        _ => {
            println!("\n  [!] Invalid type.");
        }
    }

    Ok(())
}

/// Show statistics and insights
fn show_statistics() -> Result<()> {
    println!("\n=== STATISTICS & INSIGHTS ===");

    let mut manager = EvalManager::new();
    manager
        .load_evaluations()
        .context("Failed to load evaluations")?;

    let stats = manager.calculate_statistics();

    if stats.count == 0 {
        println!("\n  [!] No evaluations with scores available.");
        return Ok(());
    }

    println!("\n  Statistical Overview:");
    println!("  {}", "-".repeat(60));
    println!("  Total Evaluations: {}", stats.count);
    println!("  Mean Score: {:.2}/10", stats.mean);
    println!("  Median Score: {:.2}/10", stats.median);
    println!("  Std Deviation: {:.2}", stats.std_dev);
    println!("  Min Score: {:.2}/10", stats.min);
    println!("  Max Score: {:.2}/10", stats.max);

    // Category leaders
    let leaders = manager.find_category_leaders();

    if !leaders.is_empty() {
        println!("\n  Category Leaders:");
        println!("  {}", "-".repeat(60));

        for (category_id, top_tools) in leaders.iter().take(5) {
            println!("\n  {}:", category_id);
            for (i, (tool_name, score)) in top_tools.iter().enumerate() {
                println!("    {}. {:<20} {:.2}/10", i + 1, tool_name, score);
            }
        }
    }

    // Recommendations
    let recommendations = manager.generate_recommendations();

    if !recommendations.is_empty() {
        println!("\n  Recommendations:");
        println!("  {}", "-".repeat(60));

        for rec in recommendations {
            println!("\n  Tool: {}", rec.tool_name);
            println!("  Use Case: {}", rec.use_case);
            println!("  Reason: {}", rec.reason);
        }
    }

    Ok(())
}

/// Show coverage report for integrated tools
fn show_coverage_report() -> Result<()> {
    println!("\n=== COVERAGE REPORT ===");

    let mut manager = EvalManager::new();
    manager
        .load_evaluations()
        .context("Failed to load evaluations")?;

    // Get integrated tools from Terminal Jarvis
    let integrated_tools = vec![
        "claude".to_string(),
        "gemini".to_string(),
        "qwen".to_string(),
        "opencode".to_string(),
        "aider".to_string(),
        "amp".to_string(),
        "goose".to_string(),
        "llxprt".to_string(),
        "codex".to_string(),
        "crush".to_string(),
    ];

    let coverage = manager.check_coverage(&integrated_tools);

    println!("\n  Total Integrated Tools: {}", coverage.total_tools);
    println!("  Evaluated Tools: {}", coverage.evaluated_tools);
    println!("  Coverage: {:.1}%", coverage.coverage_percentage);

    if !coverage.missing_evaluations.is_empty() {
        println!("\n  Missing Evaluations:");
        for tool in &coverage.missing_evaluations {
            println!("    - {}", tool);
        }
    } else {
        println!("\n  [COMPLETE] All integrated tools have been evaluated!");
    }

    Ok(())
}

/// Validate all evaluations
fn validate_evaluations() -> Result<()> {
    println!("\n=== VALIDATE EVALUATIONS ===");

    let mut manager = EvalManager::new();
    manager
        .load_evaluations()
        .context("Failed to load evaluations")?;

    let issues = manager.validate_evaluations();

    if issues.is_empty() {
        println!("\n  [OK] All evaluations are valid!");
        return Ok(());
    }

    println!("\n  Found {} validation issues:", issues.len());
    println!("  {}", "-".repeat(60));

    let mut errors = 0;
    let mut warnings = 0;
    let mut info = 0;

    for issue in &issues {
        let severity_str = match issue.severity {
            IssueSeverity::Error => {
                errors += 1;
                "[ERROR]".to_string()
            }
            IssueSeverity::Warning => {
                warnings += 1;
                "[WARNING]".to_string()
            }
            IssueSeverity::Info => {
                info += 1;
                "[INFO]".to_string()
            }
        };

        println!("  {} {}: {}", severity_str, issue.tool_name, issue.message);
    }

    println!(
        "\n  Summary: {} errors, {} warnings, {} info",
        errors, warnings, info
    );

    Ok(())
}

/// Show information about the Evals framework
fn show_about() -> Result<()> {
    println!("\n=== ABOUT EVALS FRAMEWORK ===");

    println!("\n  Overview:");
    println!("  Terminal Jarvis Evals Framework provides comprehensive, structured");
    println!("  evaluation of AI coding tools across 13 standard criteria plus");
    println!("  customizable X-factor categories.");

    println!("\n  Standard Evaluation Criteria:");
    println!("   1. Authentication & Setup");
    println!("   2. Invocation Interface");
    println!("   3. Model/Provider Support");
    println!("   4. Extensibility");
    println!("   5. User Experience");
    println!("   6. Privacy & Security");
    println!("   7. Documentation Quality");
    println!("   8. Community & Support");
    println!("   9. Licensing");
    println!("  10. Performance");
    println!("  11. Integration Capabilities");
    println!("  12. Unique Differentiators");
    println!("  13. Cost Structure");

    println!("\n  Export Formats:");
    println!("  - JSON (structured data for programmatic use)");
    println!("  - CSV (spreadsheet compatibility)");
    println!("  - Markdown (documentation and reports)");
    println!("  - HTML (web-ready comparisons)");

    println!("\n  Configuration:");
    println!("  - Standard criteria: config/evals/criteria.toml");
    println!("  - Custom X-factor: config/evals/x-factor.toml");
    println!("  - Evaluations: config/evals/evaluations/*.toml");

    println!("\n  Version:");
    println!("  Terminal Jarvis v{}", env!("CARGO_PKG_VERSION"));
    println!("  Evals Framework v{}", EVALS_VERSION);

    println!("\n  New in v0.0.70:");
    println!("  - Real-world verifiable metrics (GitHub stars, forks, community size)");
    println!("  - Objective decision factors (docs freshness, response times, team transparency)");
    println!("  - Platform support matrix (macOS, Linux, Windows, architectures)");
    println!("  - Future-ready data fetching scaffold for online statistics");

    Ok(())
}
