use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::time::sleep;

/// Progress utilities for making cold starts more seamless
pub struct ProgressUtils;

impl ProgressUtils {
    /// Create a spinner for indeterminate progress
    pub fn spinner(message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    /// Create a progress bar for determinate progress
    #[allow(dead_code)]
    pub fn progress_bar(total: u64, message: &str) -> ProgressBar {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {eta}")
                .unwrap()
                .progress_chars("██░"),
        );
        pb.set_message(message.to_string());
        pb
    }

    /// Create a multi-progress container for multiple concurrent operations
    #[allow(dead_code)]
    pub fn multi_progress() -> MultiProgress {
        MultiProgress::new()
    }

    /// Finish progress with success message
    pub fn finish_with_success(pb: &ProgressBar, message: &str) {
        pb.finish_with_message(format!("✅ {message}"));
        // Brief pause to let user see the success message before clearing
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    /// Finish progress with error message
    pub fn finish_with_error(pb: &ProgressBar, message: &str) {
        pb.finish_with_message(format!("❌ {message}"));
        // Brief pause to let user see the error message before clearing
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    /// Simulate installation progress with realistic steps
    pub async fn simulate_installation_progress(pb: &ProgressBar, tool_name: &str) {
        let steps = vec![
            ("Checking NPM registry", 200),
            ("Downloading package metadata", 300),
            ("Resolving dependencies", 400),
            ("Downloading packages", 800),
            ("Installing dependencies", 600),
            ("Building native modules", 400),
            ("Finalizing installation", 200),
        ];

        for (step, duration) in steps {
            pb.set_message(format!("📦 Installing {tool_name}: {step}"));
            sleep(Duration::from_millis(duration)).await;
        }
    }

    /// Simulate tool verification progress
    pub async fn simulate_verification_progress(pb: &ProgressBar, tool_name: &str) {
        let steps = vec![
            ("Locating binary", 150),
            ("Checking PATH", 100),
            ("Verifying version", 200),
            ("Testing functionality", 250),
        ];

        for (step, duration) in steps {
            pb.set_message(format!("🔍 Verifying {tool_name}: {step}"));
            sleep(Duration::from_millis(duration)).await;
        }
    }

    /// Show a quick loading animation for brief operations
    #[allow(dead_code)]
    pub async fn quick_load(message: &str, duration_ms: u64) -> ProgressBar {
        let pb = Self::spinner(message);
        sleep(Duration::from_millis(duration_ms)).await;
        pb
    }

    /// Create a styled info message
    pub fn info_message(message: &str) {
        println!("ℹ️  {message}");
    }

    /// Create a styled warning message
    pub fn warning_message(message: &str) {
        println!("⚠️  {message}");
    }

    /// Create a styled error message
    pub fn error_message(message: &str) {
        println!("❌ {message}");
    }

    /// Create a styled success message
    pub fn success_message(message: &str) {
        println!("✅ {message}");
    }
}

/// Progress context for long-running operations
pub struct ProgressContext {
    pub spinner: ProgressBar,
    #[allow(dead_code)]
    pub operation: String,
}

impl ProgressContext {
    pub fn new(operation: &str) -> Self {
        let spinner = ProgressUtils::spinner(&format!("⚡ {operation}"));
        Self {
            spinner,
            operation: operation.to_string(),
        }
    }

    pub fn update_message(&self, message: &str) {
        self.spinner.set_message(format!("⚡ {message}"));
    }

    pub fn finish_success(&self, message: &str) {
        ProgressUtils::finish_with_success(&self.spinner, message);
        // Clear the line after showing success message
        print!("\x1b[2K\r");
    }

    pub fn finish_error(&self, message: &str) {
        ProgressUtils::finish_with_error(&self.spinner, message);
        // Keep error messages visible longer
    }
}

impl Drop for ProgressContext {
    fn drop(&mut self) {
        if !self.spinner.is_finished() {
            self.spinner.finish_and_clear();
        }
        // Ensure cursor is visible and terminal is clean
        print!("\x1b[?25h"); // Show cursor
        print!("\x1b[2K\r"); // Clear current line
    }
}
