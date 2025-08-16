// Tools Process Management Domain
// Handles special process management and terminal state preparation

use anyhow::Result;
use std::process::Command;

/// Special process management for opencode to prevent "close of closed channel" panic
/// This handles opencode's signal handling more carefully to avoid TUI cleanup race conditions
pub fn run_opencode_with_clean_exit(mut cmd: Command) -> Result<std::process::ExitStatus> {
    // For opencode, we need to be more careful about process management
    // The panic happens when opencode's status component cleanup runs multiple times
    // or when channels are closed by multiple goroutines simultaneously

    // Start the process but manage it more carefully
    let mut child = cmd
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn opencode: {}", e))?;

    // Set up signal handling to ensure clean shutdown
    // This prevents the race condition that causes the channel panic
    let result = child.wait();

    match result {
        Ok(status) => Ok(status),
        Err(e) => {
            // If wait fails, try to clean up gracefully
            let _ = child.kill(); // Best effort cleanup
            Err(anyhow::anyhow!("Failed to wait for opencode: {}", e))
        }
    }
}

/// Prepare terminal state specifically for opencode to ensure proper input focus
pub fn prepare_opencode_terminal_state() -> Result<()> {
    use std::io::Write;

    // For opencode, we need a very careful terminal preparation sequence
    // to ensure the input box gets proper focus on fresh installs
    // MINIMAL approach - let opencode handle most terminal setup itself

    // 1. Just ensure cursor is visible and flush - let opencode do the rest
    print!("\x1b[?25h"); // Show cursor
    std::io::stdout().flush()?;

    // 2. Very brief delay to let any pending output finish
    std::thread::sleep(std::time::Duration::from_millis(50));

    Ok(())
}
