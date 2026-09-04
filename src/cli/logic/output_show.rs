//! Show: the harness detail surface. Human lines for the rich tui/body
//! (centered+dimmed by the paint pass), key=value for --plain.

use super::{style, table};
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
    // Rich: label/value fields + one line per capability, width-wrapped.
    let width = table::terminal_width();
    let rows = |pad: String, text: &str| -> Vec<String> {
        wrap(text, width.saturating_sub(pad.chars().count()))
            .split('\n')
            .enumerate()
            .map(|(step, line)| {
                if step == 0 {
                    format!("{pad}{line}")
                } else {
                    format!("{}{line}", " ".repeat(pad.chars().count()))
                }
            })
            .collect()
    };
    let field = |label: &str, value: &str| rows(format!("  {:<10} ", label), value);

    let mut lines: Vec<String> = wrap(&harness.description, width)
        .split('\n')
        .map(String::from)
        .collect();
    lines.push(String::new());
    lines.extend(field("binary", &harness.binary));
    lines.extend(field("setup", &harness.setup_hint()));
    lines.extend(field("support", &support_counts(harness)));
    for plan in &harness.capabilities {
        lines.extend(rows(
            format!("  {:<10} ", plan.capability.to_string()),
            &plan.summary,
        ));
    }
    lines.join("\n")
}

/// Wraps by display cells (wide glyphs count two), never by char count.
pub(crate) fn wrap(text: &str, width: usize) -> String {
    let cells = crate::cli::logic::table::char_cells;
    let mut out = String::new();
    let mut line_cells = 0;
    for character in text.chars() {
        let glyph = cells(character);
        if line_cells + glyph > width && line_cells > 0 {
            out.push('\n');
            line_cells = 0;
        }
        out.push(character);
        line_cells += glyph;
    }
    out
}
