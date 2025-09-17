// Tools Process Management Domain
// Handles special process management and terminal state preparation

use anyhow::Result;
use std::process::{Command, Stdio};

// NOTE: We intentionally avoid spawning tools in separate process groups, because
// that can detach them from the foreground TTY and cause input hangs (SIGTTIN).
// Instead, we keep tools in the same process group and intercept SIGINT explicitly.

/// Special process management for opencode to prevent "close of closed channel" panic
/// This handles opencode's signal handling more carefully to avoid TUI cleanup race conditions
pub fn run_opencode_with_clean_exit(cmd: Command) -> Result<std::process::ExitStatus> {
    // IMPORTANT: Do NOT place opencode in a separate process group.
    // Doing so can detach it from the foreground TTY, leading to SIGTTIN on reads
    // and an apparent hang where the UI never accepts input. Instead, keep it in
    // the same foreground process group and intercept SIGINT to terminate only the child.
    run_tool_intercepting_sigint(cmd)
}

/// Run a tool while intercepting Ctrl+C (SIGINT) so it stops the child instead of Terminal Jarvis
/// This ensures that pressing Ctrl+C during OAuth flows (like in aider) terminates only the child
/// and always returns control back to Terminal Jarvis for post-tool menu handling.
pub fn run_tool_intercepting_sigint(mut cmd: Command) -> Result<std::process::ExitStatus> {
    // Inherit stdio so child is properly interactive
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    // IMPORTANT: Keep child in the SAME foreground process group so it can read from the TTY.
    // If the child is in a different process group that's not foreground, reads from the terminal
    // can result in SIGTTIN and appear as a hang. We'll still handle Ctrl+C by killing the child
    // process explicitly instead of signaling a process group.

    // Spawn child
    let mut child = cmd
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn process: {}", e))?;

    // Set up Ctrl+C (SIGINT) handler to forward termination to child and not kill parent
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    let child_id = child.id();
    let sigint_flag = Arc::new(AtomicBool::new(false));
    let _flag_handle =
        signal_hook::flag::register(signal_hook::consts::SIGINT, sigint_flag.clone())
            .map_err(|e| anyhow::anyhow!("Failed to register SIGINT handler: {}", e))?;

    // Wait loop: either child exits, or we get Ctrl+C signal
    loop {
        // Try non-blocking check if we received Ctrl+C
        if sigint_flag.load(Ordering::Relaxed) {
            // Received Ctrl+C: terminate the child process only (keep parent alive)
            #[cfg(unix)]
            unsafe {
                let _ = libc::kill(child_id as i32, libc::SIGTERM);
            }
            // Fallback: ensure child is killed if still running
            let _ = child.kill();
            let status = child
                .wait()
                .map_err(|e| anyhow::anyhow!("Failed to wait after SIGINT: {}", e))?;
            return Ok(status);
        }

        // Check if child has exited
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) => {
                // Sleep briefly to avoid busy loop
                std::thread::sleep(std::time::Duration::from_millis(30));
            }
            Err(e) => {
                // On error, ensure child is terminated and return error
                let _ = child.kill();
                return Err(anyhow::anyhow!("Failed to wait on child: {}", e));
            }
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
