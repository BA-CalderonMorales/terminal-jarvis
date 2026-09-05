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
    let explicit = execute::explicit_capability(words, harnesses);
    let invocation = resolve::run(words, harnesses, home).map_err(execute::resolve_error)?;
    execute::execute(invocation, options, harnesses, home, explicit, None)
}

pub fn direct(
    name: &str,
    extra: &[String],
    options: &Options,
    harnesses: &[Harness],
    home: &Path,
) -> error::Result<(i32, String)> {
    let invocation = resolve::direct(name, extra, harnesses).map_err(execute::resolve_error)?;
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

#[path = "guard_execute.rs"]
mod execute;

#[cfg(test)]
#[path = "../tests/guard_test.rs"]
mod guard_tests;
#[cfg(test)]
#[path = "../tests/guard_narrate.rs"]
mod narrate_tests;

#[cfg(test)]
use execute::{explicit_capability, resolve_error};
