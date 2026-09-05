//! InvokeLive: the headless one-shot that streams. Same policy gate and
//! plan resolution as `headless_one_shot`, but the caller drives the wait
//! loop, so the tui can paint stderr rows and keep scrolling while the
//! child thinks. Stdout accumulates as the reply.

use crate::contracts::{Capability, Harness};

/// Policy-checked streaming headless run: guards first, then a piped child
/// whose tagged lines arrive via [`crate::runtime::LiveRunning`]. The
/// caller pumps `next` (draining keys and repainting as it goes) and then
/// waits; exit 0 means the collected stdout is the reply.
pub fn headless_stream(
    harnesses: &[Harness],
    name: &str,
    prompt: &str,
) -> Result<crate::runtime::LiveRunning, String> {
    let selected = super::invoke::find(harnesses, name)?;
    let plan = selected
        .plan(Capability::Headless)
        .ok_or_else(|| format!("{name} lacks headless"))?;
    if let Err(failure) = super::guard_policy::check(selected, plan, true) {
        return Err(format!("{}: {}", failure.code, failure.message));
    }
    crate::runtime::spawn_live(plan, &[prompt.to_string()])
        .map_err(|cause| format!("{name} cannot run {}: {cause}", plan.command.command))
}
