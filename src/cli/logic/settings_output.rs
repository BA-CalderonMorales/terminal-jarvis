//! Settings output: the flag list, grouped, and the set/reset receipts.
//! Plain mode is stable `key=value` lines; rich mode is the grouped list.

use super::style;
use crate::flags::{Resolved, Stage};

const EMPTY: &str = "No optional features yet.\n";

fn state(on: bool) -> &'static str {
    if on {
        "on"
    } else {
        "off"
    }
}

pub fn list(all: &[Resolved]) -> String {
    if all.is_empty() {
        return EMPTY.to_string();
    }
    let mut ordered: Vec<&Resolved> = all.iter().collect();
    ordered.sort_by_key(|row| (rank(row.flag.stage), row.flag.name.clone()));
    if style::plain() {
        return plain(&ordered);
    }
    let width = all.iter().map(|row| row.flag.name.len()).max().unwrap_or(0);
    let (labs, features): (Vec<&Resolved>, Vec<&Resolved>) = ordered
        .into_iter()
        .partition(|row| row.flag.stage == Stage::Labs);
    let mut lines = Vec::new();
    if !features.is_empty() {
        lines.push(style::heading("Features"));
        lines.extend(features.iter().map(|row| row_line(row, width)));
    }
    if !labs.is_empty() {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.push(style::heading("Labs"));
        lines.push(style::dim("  experimental, off by default"));
        lines.extend(labs.iter().map(|row| row_line(row, width)));
    }
    format!("{}\n", lines.join("\n"))
}

fn row_line(row: &Resolved, width: usize) -> String {
    let chip = format!("[{:<3}]", state(row.enabled));
    let chip = if row.enabled {
        style::success(&chip)
    } else {
        style::dim(&chip)
    };
    format!(
        "  {chip}  {:<width$}  {}  ({})",
        row.flag.name,
        row.flag.description,
        row.source.as_str()
    )
}

/// Stable first, then preview, then labs.
fn rank(stage: Stage) -> u8 {
    match stage {
        Stage::Stable => 0,
        Stage::Preview => 1,
        Stage::Labs => 2,
    }
}

fn plain(ordered: &[&Resolved]) -> String {
    ordered
        .iter()
        .map(|row| {
            format!(
                "flag={} state={} stage={} source={} description={}\n",
                row.flag.name,
                state(row.enabled),
                row.flag.stage.as_str(),
                row.source.as_str(),
                row.flag.description
            )
        })
        .collect()
}

pub fn receipt(name: &str, value: Option<bool>, path: &str, dry_run: bool) -> String {
    let change = match value {
        Some(on) => format!("set {name} {}", state(on)),
        None => format!("reset {name} to its default"),
    };
    if dry_run {
        return format!("dry run: would {change} in {path}; nothing was written\n");
    }
    if style::plain() {
        let change = change.replace(' ', "_");
        return format!("flag={name} change={change} path={path}\n");
    }
    format!("{} ({path})\n", style::success(&change))
}
