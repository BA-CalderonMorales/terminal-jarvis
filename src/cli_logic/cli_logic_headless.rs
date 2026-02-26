// Headless mode detection and helpers
//
// Provides utilities for determining if the CLI is running in headless/agentic
// mode and adjusting behavior accordingly (no ANSI codes, no interactive prompts).

use std::sync::OnceLock;

static HEADLESS: OnceLock<bool> = OnceLock::new();
static AUTO_YES: OnceLock<bool> = OnceLock::new();

/// Initialize headless state from CLI flags.
/// Must be called once at startup before any `is_headless()` checks.
pub fn init(headless_flag: bool, yes_flag: bool) {
    use is_terminal::IsTerminal;
    let headless = headless_flag || !std::io::stdin().is_terminal();
    let _ = HEADLESS.set(headless);
    // In headless mode, --yes is always implied
    let _ = AUTO_YES.set(yes_flag || headless);
}

/// Returns true when running in headless/agentic mode.
///
/// Headless mode is active when any of these conditions are met:
/// - `--headless` CLI flag was passed
/// - `JARVIS_HEADLESS=1` environment variable is set (handled by clap)
/// - stdin is not a TTY (piped input)
pub fn is_headless() -> bool {
    *HEADLESS.get().unwrap_or(&false)
}

/// Returns true when all confirmation prompts should be auto-accepted.
///
/// True when `--yes` flag was passed or when in headless mode.
pub fn is_auto_yes() -> bool {
    *AUTO_YES.get().unwrap_or(&false)
}

/// Print a line without ANSI escape codes when headless.
/// In interactive mode, this is a no-op (caller handles themed output).
pub fn plain_info(msg: &str) {
    if is_headless() {
        println!("[INFO] {msg}");
    }
}

pub fn plain_ok(msg: &str) {
    if is_headless() {
        println!("[OK] {msg}");
    }
}

pub fn plain_error(msg: &str) {
    if is_headless() {
        eprintln!("[ERROR] {msg}");
    }
}
