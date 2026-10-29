//! Conversation turns use the ordinary authorization chain before spawn.
use super::{args::Options, guard_authorize, guard_prompt};
use crate::contracts::{Capability, Effect, Harness};
use std::path::Path;

pub fn headless_stream(
    harnesses: &[Harness],
    name: &str,
    prompt: &str,
    home: &Path,
    allow_dangerous: bool,
    paint: &mut dyn FnMut(&str),
) -> Result<crate::runtime::LiveRunning, String> {
    let harness = super::invoke::find(harnesses, name)?;
    let plan = harness
        .plan(Capability::Headless)
        .ok_or_else(|| format!("{name} lacks headless"))?;
    let extra = vec![prompt.to_string()];
    let options = Options {
        allow_dangerous: allow_dangerous && plan.effect != Effect::ReadOnly,
        narrate: false,
        ..Options::default()
    };
    let request = guard_authorize::Request {
        harness,
        plan,
        extra: &extra,
        options: &options,
        home,
        explicit: true,
    };
    guard_authorize::check(request, &mut |lead, token, default_yes| {
        guard_prompt::ask(paint, lead, token, default_yes)
    })
    .map_err(|failure| format!("{}: {}", failure.code, failure.message))?
    .spawn()
    .map_err(|cause| format!("{name} cannot run {}: {cause}", plan.command.command))
}
