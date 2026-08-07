use crate::contracts::{Capability, CapabilityPlan, Harness};

pub fn planned_steps(harness: &Harness) -> Vec<&CapabilityPlan> {
    Capability::ALL
        .iter()
        .filter_map(|capability| harness.plan(*capability))
        .collect()
}

pub fn next_step<'a>(harness: &'a Harness, completed: &[Capability]) -> Option<&'a CapabilityPlan> {
    Capability::ALL
        .iter()
        .find(|capability| !completed.contains(capability))
        .and_then(|capability| harness.plan(*capability))
}
