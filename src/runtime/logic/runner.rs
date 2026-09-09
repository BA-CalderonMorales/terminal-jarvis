use crate::contracts::CapabilityPlan;
use crate::security;
use std::io;
use std::process::{Command, Stdio};

pub fn run_command(plan: &CapabilityPlan, extra: &[String]) -> io::Result<i32> {
    run_command_text(plan, extra).map(|(code, _)| code)
}

/// Like [`run_command`], but inside the tui the child's stream is captured
/// and returned, so the frame can render it as console log lines instead of
/// letting it paint over the alt-screen. Headless keeps inherited stdio.
pub fn run_command_text(plan: &CapabilityPlan, extra: &[String]) -> io::Result<(i32, String)> {
    let mut command = Command::new(security::resolved(&plan.command.command).as_ref());
    command.args(&plan.command.args).args(extra);
    let tui_capture = crate::tui::screen::active();
    if tui_capture {
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
    } else {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }
    #[cfg(unix)]
    reset_sigint_in_child(&mut command);
    if !tui_capture {
        return command
            .status()
            .map(|status| (status_code(status), String::new()));
    }
    let output = command.output()?;
    let mut text = String::from_utf8_lossy(&output.stdout).to_string();
    let err = String::from_utf8_lossy(&output.stderr);
    if !err.trim().is_empty() {
        if !text.trim().is_empty() {
            text.push('\n');
        }
        text.push_str(err.trim());
    }
    Ok((status_code(output.status), text))
}

#[cfg(unix)]
pub(crate) fn reset_sigint_in_child(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    unsafe {
        command.pre_exec(|| {
            signal(SIGINT, SIG_DFL);
            Ok(())
        });
    }
}

#[cfg(unix)]
extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

#[cfg(unix)]
const SIGINT: i32 = 2;
#[cfg(all(unix, test))]
const SIG_IGN: usize = 1;
#[cfg(unix)]
const SIG_DFL: usize = 0;

fn status_code(status: std::process::ExitStatus) -> i32 {
    status.code().unwrap_or_else(|| signal_code(&status))
}

#[cfg(unix)]
fn signal_code(status: &std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    status.signal().map_or(1, |signal| 128 + signal)
}

#[cfg(not(unix))]
fn signal_code(_status: &std::process::ExitStatus) -> i32 {
    1
}

// Exercises `sh`/POSIX signal semantics (`kill -TERM $$`, the 128+signal
// exit-code convention `signal_code` only computes on `#[cfg(unix)]`) that
// have no Windows equivalent, so the whole module is Unix-only.
#[cfg(all(test, unix))]
#[path = "../tests/runner_test.rs"]
mod tests;
