//! One authorization chain for ordinary runs and conversation turns.
use super::{args::Options, error, gate_skip, guard_intent, guard_policy};
use crate::contracts::{CapabilityPlan, Harness};
use std::path::Path;

pub type Ask<'a> = dyn FnMut(&str, &str, bool) -> error::Result<()> + 'a;
pub struct Request<'a> {
    pub harness: &'a Harness,
    pub plan: &'a CapabilityPlan,
    pub extra: &'a [String],
    pub options: &'a Options,
    pub home: &'a Path,
    pub explicit: bool,
}

pub struct Authorized<'a> {
    plan: &'a CapabilityPlan,
    extra: Vec<String>,
    preview: bool,
}

pub fn check<'a>(request: Request<'a>, ask: &mut Ask<'_>) -> error::Result<Authorized<'a>> {
    let Request {
        harness,
        plan,
        extra,
        options,
        home,
        explicit,
    } = request;
    guard_policy::check(
        harness,
        plan,
        std::io::IsTerminal::is_terminal(&std::io::stdin()),
    )?;
    guard_intent::check_with(harness, plan, extra, options, explicit, ask)?;
    if !options.dry_run {
        let target = format!("{}:{}", plan.capability, harness.name);
        crate::gates::preflight(home, options.narrate)
            .map_err(|message| {
                error::Failure::safety("gate_blocked", message, "run `terminal-jarvis gate status`")
            })
            .and_then(|verdict| gate_skip::route(options, verdict, &target))?;
    }
    Ok(Authorized {
        plan,
        extra: extra.to_vec(),
        preview: options.dry_run,
    })
}

impl Authorized<'_> {
    pub fn spawn(self) -> std::io::Result<crate::runtime::LiveRunning> {
        if self.preview {
            return Err(std::io::Error::other("a preview cannot spawn an agent"));
        }
        crate::runtime::spawn_live(self.plan, &self.extra)
    }
}

#[cfg(test)]
#[path = "../tests/authorization.rs"]
mod tests;
