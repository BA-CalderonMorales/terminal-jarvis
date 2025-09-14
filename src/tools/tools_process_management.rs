// Tools Process Management Domain
// Handles special process management and terminal state preparation

use anyhow::Result;
use std::process::Command;

/// Run a tool process with proper signal isolation to prevent parent process termination
/// This ensures that Ctrl+C and other signals sent to child processes don't terminate Terminal Jarvis
pub fn run_tool_with_signal_isolation(mut cmd: Command) -> Result<std::process::ExitStatus> {
    // Start the process as a separate process group to isolate signals
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // Create new process group - this should prevent signal propagation to parent
        cmd.process_group(0);
    }

    // Start the process
    let mut child = cmd
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn process: {}", e))?;

    // Wait for the process to complete
    // Even if the child receives SIGINT/SIGTERM, the parent (Terminal Jarvis) should continue
    let result = child.wait();

    match result {
        Ok(status) => Ok(status),
        Err(e) => {
            // If wait fails, try to clean up gracefully
            let _ = child.kill(); // Best effort cleanup
            Err(anyhow::anyhow!("Failed to wait for child process: {}", e))
        }
    }
}

/// Special process management for opencode to prevent "close of closed channel" panic
/// This handles opencode's signal handling more carefully to avoid TUI cleanup race conditions
pub fn run_opencode_with_clean_exit(cmd: Command) -> Result<std::process::ExitStatus> {
    // Use the same signal isolation approach for opencode
    run_tool_with_signal_isolation(cmd)
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
