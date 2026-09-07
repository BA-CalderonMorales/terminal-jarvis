//! Show: the harness detail surface. Human lines for the rich tui/body
//! (centered+dimmed by the paint pass), key=value for --plain.

use super::super::{output_fields as fields, style};
use crate::cli::logic::output_truth;
use crate::contracts::Harness;

/// Human support counts: every contract state, non-zero only, fixed order.
pub(crate) fn support_counts(harness: &Harness) -> String {
    use crate::contracts::SupportState;
    let order = [
        SupportState::Verified,
        SupportState::Expected,
        SupportState::Manual,
        SupportState::Stub,
        SupportState::Unsupported,
        SupportState::Unknown,
        SupportState::Disabled,
    ];
    let mut counts = order.map(|state| (state, 0));
    for plan in &harness.capabilities {
        for entry in counts.iter_mut() {
            if entry.0 == plan.support {
                entry.1 += 1;
            }
        }
    }
    counts
        .iter()
        .filter(|(_, count)| *count > 0)
        .map(|(state, count)| format!("{count} {}", state.as_str()))
        .collect::<Vec<_>>()
        .join(" · ")
}
pub fn show(harness: &Harness) -> String {
    if style::plain() {
        let mut out = format!(
            "{} ({})\n{}\nbinary: {}\nsetup: {}\nsupport: {}\n",
            harness.display,
            harness.name,
            harness.description,
            harness.binary,
            harness.setup_hint(),
            output_truth::support_summary(harness)
        );
        for plan in &harness.capabilities {
            out.push_str(&output_truth::plain_capability(plan));
        }
        return out;
    }
    // Rich: identity line, wrapped description, then label/value fields.
    let width = fields::width();
    let mut lines = vec![format!("{} ({})", harness.display, harness.name)];
    lines.extend(fields::section(String::new(), &harness.description, width));
    lines.push(String::new());
    lines.extend(fields::field("binary", &harness.binary, width));
    lines.extend(fields::field("setup", &harness.setup_hint(), width));
    lines.extend(fields::field("support", &support_counts(harness), width));
    for plan in &harness.capabilities {
        lines.extend(fields::section(
            format!("  {:<10} ", plan.capability.to_string()),
            &plan.summary,
            width,
        ));
    }
    lines.join("\n")
}
