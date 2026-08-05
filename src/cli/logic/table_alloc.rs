use super::super::width::display_width as width;

pub const MAX_FLOOR: usize = 24;

pub fn widths(headers: &[&str], rows: &[Vec<String>]) -> Vec<usize> {
    let mut widths = headers
        .iter()
        .map(|header| width(header))
        .collect::<Vec<_>>();
    for row in rows {
        for (index, value) in row.iter().enumerate().take(widths.len()) {
            widths[index] = widths[index].max(value.lines().map(width).max().unwrap_or(0));
        }
    }
    let budget = terminal_width().saturating_sub(headers.len() * 3 + 1);
    let mut floors = floors(headers, rows);
    if floors.iter().sum::<usize>() > budget {
        floors = vec![MIN_COLUMN; headers.len()];
    }
    fit(&mut widths, budget, &floors);
    widths
}

pub(super) fn floors(headers: &[&str], rows: &[Vec<String>]) -> Vec<usize> {
    let mut floors = headers
        .iter()
        .map(|header| longest_token(header))
        .collect::<Vec<_>>();
    for row in rows {
        for (index, value) in row.iter().enumerate().take(floors.len()) {
            floors[index] = floors[index].max(longest_token(value));
        }
    }
    floors
}

fn longest_token(value: &str) -> usize {
    value
        .split_whitespace()
        .flat_map(super::text::segments)
        .map(|token| width(&token).min(MAX_FLOOR))
        .max()
        .unwrap_or(0)
        .max(MIN_COLUMN)
}

fn fit(widths: &mut [usize], budget: usize, floors: &[usize]) {
    let total = widths.iter().sum::<usize>();
    if total <= budget {
        return;
    }
    let demand = total - budget;
    let original = widths.to_vec();
    for (index, width) in widths.iter_mut().enumerate() {
        *width = original[index]
            .saturating_sub(original[index] * demand / total)
            .max(floors[index]);
    }
    let mut remaining = widths.iter().sum::<usize>().saturating_sub(budget);
    while remaining > 0 {
        let Some(index) = widest_above_floor(widths, floors) else {
            break;
        };
        widths[index] -= 1;
        remaining -= 1;
    }
}

fn widest_above_floor(widths: &[usize], floors: &[usize]) -> Option<usize> {
    let mut candidate = None;
    let mut best = 0usize;
    for (index, width) in widths.iter().enumerate() {
        if *width > best && *width > floors[index] {
            best = *width;
            candidate = Some(index);
        }
    }
    candidate
}

use super::{terminal_width, MIN_COLUMN};
