use terminal_jarvis::evals::EvalManager;

#[test]
fn test_load_all_metrics_files() {
    let mut manager = EvalManager::new();

    match manager.load_evaluations() {
        Ok(_) => {
            let summary = manager.get_summary();
            println!(
                "\n[SUCCESS] Loaded {} evaluations",
                summary.total_evaluations
            );

            let mut tools_with_metrics = 0;
            for tool in &summary.tools {
                if let Some(eval) = manager.get_evaluation(tool) {
                    let has_metrics = eval.metrics.is_some();
                    println!(
                        "  - {} (Metrics: {})",
                        tool,
                        if has_metrics { "YES" } else { "NO" }
                    );
                    if has_metrics {
                        tools_with_metrics += 1;
                    }
                }
            }

            assert!(
                summary.total_evaluations >= 10,
                "Should load at least 10 metrics files"
            );

            // Note: metrics field is optional, so not all tools will have metrics
            // As of this test, 9/10 tools have metrics (qwen does not)
            assert!(
                tools_with_metrics >= 9,
                "Expected at least 9 tools with metrics, found: {tools_with_metrics}"
            );

            println!(
                "\n[VERIFICATION] {} out of {} tools have metrics",
                tools_with_metrics, summary.total_evaluations
            );
            println!("[SUCCESS] All TOML files parsed successfully without errors!");
        }
        Err(e) => {
            panic!("Failed to load evaluations: {e}");
        }
    }
}
