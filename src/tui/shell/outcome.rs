//! Outcome: applies one resolved command to the loop state -- body
//! absorption for the viewport, printing for chat mode, hint and indicator
//! refreshes, and the debug toggle.

use super::{status, viewport, Next};
use crate::{cli::args, contracts::Harness};
use std::path::Path;

/// Applies one command's outcome; false means the loop ends.
#[allow(clippy::too_many_arguments)]
pub fn step(
    next: Next,
    body: &mut Vec<String>,
    sink: Vec<u8>,
    hint: &mut String,
    options: &mut args::Options,
    debug: &mut bool,
    indicator: &mut crate::tui::input::Indicator,
    state_home: &Path,
    harnesses: &[Harness],
    catalog_root: &Path,
) -> bool {
    match next {
        Next::Exit => false,
        Next::Again {
            picker_shown,
            reset,
        } => {
            *hint = status::modeline(state_home, picker_shown, *debug);
            status::refresh_indicator(indicator, state_home, *debug);
            absorb(body, sink, reset, harnesses, catalog_root, state_home);
            true
        }
        Next::Debug(toggle) => {
            *debug = toggle.unwrap_or(!*debug);
            options.narrate = *debug;
            *hint = status::modeline(state_home, false, *debug);
            status::refresh_indicator(indicator, state_home, *debug);
            let line = format!("debug view {}", if *debug { "on" } else { "off" });
            if crate::tui::screen::active() {
                body.push(line);
            } else {
                println!("{line}");
            }
            true
        }
    }
}

/// Viewport absorbs captured output as the next body; chat prints it above
/// the prompt. A reset restores the welcome; an empty capture keeps both.
fn absorb(
    body: &mut Vec<String>,
    sink: Vec<u8>,
    reset: bool,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) {
    let text = String::from_utf8_lossy(&sink).to_string();
    if reset {
        *body = viewport::welcome(harnesses, catalog_root, state_home);
    } else if !text.is_empty() {
        *body = text.lines().map(String::from).collect();
    }
    if !crate::tui::screen::active() {
        print!("{text}");
        println!();
    }
}
