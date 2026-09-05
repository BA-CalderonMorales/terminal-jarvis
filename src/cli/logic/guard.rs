//! Guard: the consent layer every harness invocation passes through --
//! policy (support/freshness/platform), intent, the trivy gate preflight,
//! and the package advisory -- before the runner executes. Streaming runs
//! share the identical chain; only the last hop differs.

use super::{args::Options, error, resolve};
use crate::contracts::{Capability, Harness};
use std::path::Path;

pub fn run(
    words: &[String],
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
) -> error::Result<(i32, String)> {
    let explicit = explicit_capability(words, harnesses);
    let invocation = resolve::run(words, harnesses, home).map_err(resolve_error)?;
    execute::execute(invocation, options, harnesses, home, explicit, None)
}

pub fn direct(
    name: &str,
    extra: &[String],
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
) -> error::Result<(i32, String)> {
    let invocation = resolve::direct(name, extra, harnesses).map_err(resolve_error)?;
    execute::execute(invocation, options, harnesses, home, false, None)
}

pub fn capability(
    harnesses: &[Harness],
    name: &str,
    capability: Capability,
    options: &Options,
    home: &Path,
) -> error::Result<(i32, String)> {
    let invocation = resolve::Invocation {
        harness: name.to_string(),
        capability,
        extra: Vec::new(),
    };
    execute::execute(invocation, options, harnesses, home, true, None)
}

/// Streams one invocation line-by-line through the same guard chain the
/// blocking run uses; the tui paints each line as a splunk row.
pub fn stream_invocation(
    invocation: resolve::Invocation,
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
    on_line: &mut dyn FnMut(&str),
) -> error::Result<i32> {
    execute::execute(invocation, options, harnesses, home, true, Some(on_line))
        .map(|(code, _)| code)
}

/// True when the words name a known harness plus a parsable capability, so
/// the intent gate can tell an explicit ask from an ambient default.
pub fn explicit_capability(words: &[String], harnesses: &[Harness]) -> bool {
    words.len() >= 2
        && harnesses.iter().any(|harness| harness.name == words[0])
        && Capability::parse(&words[1]).is_some()
}

/// Maps a resolve failure onto the exit contract: active-harness state
/// problems guide back to `use`, everything else to the fleet list.
pub fn resolve_error(message: String) -> error::Failure {
    if message.contains("no active harness") || message.contains("active harness") {
        return error::Failure::state(
            "active_harness_invalid",
            message,
            "run `terminal-jarvis use <harness>` or pass a harness",
        );
    }
    error::Failure::unavailable("harness_unknown", message, "run `terminal-jarvis list`")
}

#[path = "guard_execute.rs"]
mod execute;

#[cfg(test)]
#[path = "../tests/guard_test.rs"]
mod guard_tests;
#[cfg(test)]
#[path = "../tests/guard_narrate.rs"]
mod narrate_tests;
