use crate::evals::benchmarks::results::{TestCaseResult, ValidationResult};
use anyhow::Result;
use regex::Regex;

/// Pattern matching validator for benchmark output validation
///
/// This validator checks if the tool's output matches a set of regular expression patterns.
/// Commonly used for code completion benchmarks where specific code patterns must be present.
///
/// # Phase 2 API
/// This validator is part of the Phase 2 benchmarks framework and is intentionally
/// unused in Phase 1. It will be integrated with BenchmarkScenario validation in Phase 3.
#[allow(dead_code)]
pub struct PatternMatchValidator {
    patterns: Vec<String>,
}

#[allow(dead_code)]
impl PatternMatchValidator {
    /// Create a new pattern match validator with the given regex patterns
    ///
    /// # Arguments
    /// * `patterns` - Vector of regex pattern strings to match against output
    ///
    /// # Example
    /// ```
    /// use terminal_jarvis::evals::benchmarks::validators::PatternMatchValidator;
    ///
    /// let validator = PatternMatchValidator::new(vec![
    ///     r"a \+ b".to_string(),
    ///     r"return".to_string(),
    /// ]);
    /// ```
    pub fn new(patterns: Vec<String>) -> Self {
        Self { patterns }
    }

    /// Validate output against all configured patterns
    ///
    /// # Arguments
    /// * `output` - The tool's output to validate
    ///
    /// # Returns
    /// * `Result<ValidationResult>` - Validation results with score and test case details
    ///
    /// # Scoring
    /// - Score is calculated as (matched_patterns / total_patterns) * 10.0
    /// - All patterns must match for `passed` to be true
    ///
    /// # Example
    /// ```
    /// use terminal_jarvis::evals::benchmarks::validators::PatternMatchValidator;
    ///
    /// let validator = PatternMatchValidator::new(vec![r"return".to_string()]);
    /// let output = "fn add() { return 5; }";
    /// let result = validator.validate(output).unwrap();
    ///
    /// assert!(result.passed);
    /// assert_eq!(result.score, 10.0);
    /// ```
    pub fn validate(&self, output: &str) -> Result<ValidationResult> {
        let mut test_results = Vec::new();
        let mut passed_count = 0;

        for (i, pattern) in self.patterns.iter().enumerate() {
            let regex = Regex::new(pattern)?;
            let matched = regex.is_match(output);

            if matched {
                passed_count += 1;
            }

            test_results.push(TestCaseResult {
                test_name: format!("pattern_{i}"),
                passed: matched,
                expected: pattern.clone(),
                actual: if matched {
                    "Pattern found in output".to_string()
                } else {
                    "Pattern NOT found in output".to_string()
                },
                error: None,
            });
        }

        let total = self.patterns.len();
        let score = if total > 0 {
            (passed_count as f64 / total as f64) * 10.0
        } else {
            0.0
        };

        Ok(ValidationResult {
            passed: passed_count == total,
            score,
            test_case_results: test_results,
            errors: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_patterns_match() {
        let validator =
            PatternMatchValidator::new(vec![r"a \+ b".to_string(), r"return".to_string()]);

        let output = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
        let result = validator.validate(output).unwrap();

        assert!(result.passed);
        assert_eq!(result.score, 10.0);
        assert_eq!(result.test_case_results.len(), 2);
    }

    #[test]
    fn test_partial_match() {
        let validator =
            PatternMatchValidator::new(vec![r"return".to_string(), r"missing_pattern".to_string()]);

        let output = "fn add() { return 5; }";
        let result = validator.validate(output).unwrap();

        assert!(!result.passed);
        assert_eq!(result.score, 5.0); // 1 of 2 patterns
        assert_eq!(result.test_case_results.len(), 2);
    }

    #[test]
    fn test_no_patterns_match() {
        let validator = PatternMatchValidator::new(vec![r"missing".to_string()]);

        let output = "some output";
        let result = validator.validate(output).unwrap();

        assert!(!result.passed);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn test_scenario_patterns_from_toml() {
        // Test with exact patterns from config/benchmarks/scenarios/code-completion/basic-001.toml
        // expected_patterns = ["a \\+ b", "return"]
        let validator =
            PatternMatchValidator::new(vec![r"a \+ b".to_string(), r"return".to_string()]);

        // Case 1: Complete function with explicit return
        let output1 = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
        let result1 = validator.validate(output1).unwrap();
        assert!(result1.passed);
        assert_eq!(result1.score, 10.0);

        // Case 2: Just the expression (partial match - missing return)
        let output2 = "a + b";
        let result2 = validator.validate(output2).unwrap();
        assert!(!result2.passed);
        assert_eq!(result2.score, 5.0); // Only 1 of 2 patterns

        // Case 3: Expression-based return (Rust idiomatic - missing explicit return)
        let output3 = "fn add(a: i32, b: i32) -> i32 { a + b }";
        let result3 = validator.validate(output3).unwrap();
        assert!(!result3.passed); // Missing "return" keyword
        assert_eq!(result3.score, 5.0); // Only has "a + b", missing "return"
    }
}
