//! Quiet (tui clean) mode swaps the full plan table for a one-line lead;
//! headless narration keeps the complete plan for review.
use super::args::Options;
use crate::cli::logic::output;
use crate::contracts::{CapabilityPlan, Harness};

pub fn confirm_lead(
    options: &Options,
    harness: &Harness,
    plan: &CapabilityPlan,
    extra: &[String],
) -> String {
    if options.narrate {
        return output::plan_with_extra(harness, plan.capability, extra);
    }
    format!(
        "Plan: {} {} — {}\n",
        harness.name,
        plan.capability,
        plan.command.render()
    )
}
