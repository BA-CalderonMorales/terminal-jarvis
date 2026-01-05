// src/progress_utils.rs
//
// Progress indication utilities for Terminal Jarvis operations
//
// Provides spinner and progress bar utilities with theme integration
// to enhance user experience during long-running operations.

use crate::theme::theme_global_config;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::time::sleep;

/// Progress utilities for enhancing user experience during operations
///
/// Provides themed progress indicators, spinners, and progress bars that integrate
/// with Terminal Jarvis's theming system. Designed to make cold starts and long-running
/// operations feel more responsive and professional.
///
/// All progress indicators respect the user's theme configuration and provide
/// consistent visual feedback across different Terminal Jarvis operations.
pub struct ProgressUtils;

impl ProgressUtils {
    /// Creates a themed spinner for indeterminate progress operations
    ///
    /// Displays an animated spinner with the specified message, using Terminal Jarvis
    /// theme colors for consistency. Ideal for operations where progress cannot be
    /// measured (e.g., network requests, tool initialization).
    ///
    /// # Arguments
    ///
    /// * `message` - The message to display alongside the spinner
    ///
    /// # Returns
    ///
    /// A configured ProgressBar in spinner mode
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let spinner = ProgressUtils::spinner("Initializing tool...");
    /// // Perform long operation
    /// spinner.finish_with_message("Tool ready!");
    /// ```
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

    /// Creates a themed progress bar for determinate progress operations
    ///
    /// Displays a progress bar with percentage and ETA, using Terminal Jarvis
    /// theme colors. Ideal for operations where total progress can be measured
    /// (e.g., file downloads, batch processing).
    ///
    /// # Arguments
    ///
    /// * `total` - The total number of steps for the progress bar
    /// * `message` - The message to display alongside the progress bar
    ///
    /// # Returns
    ///
    /// A configured ProgressBar with the specified total and message
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let pb = ProgressUtils::progress_bar(100, "Processing items...");
    /// for i in 0..100 {
    ///     pb.inc(1);
    ///     // Perform work
    /// }
    /// pb.finish();
    /// ```
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

    /// Creates a multi-progress manager for concurrent operations
    ///
    /// Enables multiple progress indicators to run simultaneously without
    /// interfering with each other's display. Useful for batch operations
    /// or parallel task execution.
    ///
    /// # Returns
    ///
    /// A MultiProgress instance for managing multiple progress indicators
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let multi = ProgressUtils::multi_progress();
    /// let pb1 = multi.add(ProgressBar::new(100));
    /// let pb2 = multi.add(ProgressBar::new(50));
    /// // Run concurrent operations
    /// ```
    #[allow(dead_code)]
    pub fn multi_progress() -> MultiProgress {
        MultiProgress::new()
    }

    /// Completes progress indicator with a success message
    ///
    /// Displays a success message and provides visual feedback that the operation
    /// completed successfully. Includes a brief pause to ensure visibility.
    ///
    /// # Arguments
    ///
    /// * `pb` - The ProgressBar to complete
    /// * `message` - Success message to display
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let spinner = ProgressUtils::spinner("Installing tool...");
    /// // Perform installation
    /// ProgressUtils::finish_with_success(&spinner, "Tool installed successfully!");
    /// ```
    pub fn finish_with_success(pb: &ProgressBar, message: &str) {
        pb.finish_with_message(format!("SUCCESS: {message}"));

        // Brief pause to let user see the success message before clearing
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    /// Completes progress indicator with an error message
    ///
    /// Displays an error message and provides visual feedback that the operation
    /// failed. Includes a brief pause to ensure visibility.
    ///
    /// # Arguments
    ///
    /// * `pb` - The ProgressBar to complete
    /// * `message` - Error message to display
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let spinner = ProgressUtils::spinner("Connecting to service...");
    /// // Attempt connection
    /// if connection_failed {
    ///     ProgressUtils::finish_with_error(&spinner, "Connection failed");
    /// }
    /// ```
    pub fn finish_with_error(pb: &ProgressBar, message: &str) {
        pb.finish_with_message(format!("ERROR: {message}"));

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
            pb.set_message(format!("Installing {tool_name}: {step}"));

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
            pb.set_message(format!("Verifying {tool_name}: {step}"));

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
        let theme = theme_global_config::current_theme();

        println!("{} {}", theme.accent("T.JARVIS:"), theme.primary(message));
    }

    /// Create a styled warning message
    pub fn warning_message(message: &str) {
        let theme = theme_global_config::current_theme();

        println!(
            "{} {}",
            theme.secondary("⚠ ADVISORY:"),
            theme.primary(message)
        );
    }

    /// Create a styled error message
    pub fn error_message(message: &str) {
        let theme = theme_global_config::current_theme();

        println!("{} {}", theme.accent("✗ SYSTEM:"), theme.primary(message));
    }

    /// Create a styled success message
    pub fn success_message(message: &str) {
        let theme = theme_global_config::current_theme();

        println!("{} {}", theme.accent("✓ COMPLETE:"), theme.primary(message));
    }
}

/// Progress context for coordinating long-running operations
///
/// Manages multiple related progress indicators and provides a unified interface
/// for complex operations that involve multiple steps or parallel tasks.
///
/// Integrates with Terminal Jarvis theming and provides consistent progress
/// reporting across different operation types.
pub struct ProgressContext {
    pub spinner: ProgressBar,

    #[allow(dead_code)]
    pub operation: String,
}

impl ProgressContext {
    pub fn new(operation: &str) -> Self {
        let spinner = ProgressUtils::spinner(operation);

        Self {
            spinner,
            operation: operation.to_string(),
        }
    }

    pub fn update_message(&self, message: &str) {
        self.spinner.set_message(message.to_string());
    }

    pub fn finish_success(&self, message: &str) {
        ProgressUtils::finish_with_success(&self.spinner, message);

        // Clear the line after showing success message and flush
        print!("\x1b[2K\r");
        std::io::Write::flush(&mut std::io::stdout()).unwrap_or_default();
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
        std::io::Write::flush(&mut std::io::stdout()).unwrap_or_default();
    }
}
