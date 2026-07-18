use super::{input, sandbox::Sandbox, Report, Row};
use std::collections::BTreeSet;
use terminal_jarvis::catalog;

pub fn walk_catalog() -> Report {
    let root = input::catalog_root();
    let harnesses = catalog::load(&root).expect("validated catalog loads");
    assert!(catalog::validate(&harnesses).is_empty());
    let mut sandbox = Sandbox::new();
    let mut keys = BTreeSet::new();
    let mut probes = Vec::new();
    let mut states = BTreeSet::new();
    let mut rows = Vec::new();
    for harness in &harnesses {
        for plan in &harness.capabilities {
            let key = format!("{}:{}", harness.name, plan.capability);
            assert!(keys.insert(key), "catalog row was visited twice");
            sandbox.add_fake(&plan.command.command);
            states.insert(plan.support);
            probes.push((harness.name.clone(), plan.capability, plan.support));
            rows.push(Row::from_plan(harness, plan));
        }
    }
    assert_eq!(keys.len(), 225, "catalog denominator changed");
    assert_eq!(states.len(), 3, "support-state denominator changed");
    sandbox.verify_guards(&probes);
    sandbox.assert_zero_effects();
    Report::new(rows)
}
