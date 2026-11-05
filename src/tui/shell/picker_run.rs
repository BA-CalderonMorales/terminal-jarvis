//! PickerRun: the overlay's paint/read loop, the printed fallback for chat
//! mode, and the one entry point both `list` routes share. The loop is thin on
//! purpose -- every decision it makes is `Picker::step`, so the keys are
//! witnessed by unit tests and only the painting needs a terminal.

use super::super::viewport_chrome::ViewportState;
use super::{Picker, Step, HINT};
use crate::cli::args;
use crate::contracts::Harness;
use crate::tui::input::{read_key, Indicator, Key};
use crate::tui::screen::{frame, parked, size, visible_width};
use std::io::Write;
use std::path::Path;

pub fn filter(rest: &str) -> Option<String> {
    let text = rest.trim_start_matches("list").trim();
    (!text.is_empty()).then(|| text.to_string())
}

/// Selects a harness through the overlay when the frame owns the terminal,
/// and otherwise prints the picker as text. Returns whether the text picker
/// was shown, which the shell hint needs.
pub fn listing(
    out: &mut dyn Write,
    options: &args::Options,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    filter: Option<&str>,
) -> bool {
    match choose(harnesses, catalog_root, state_home, filter) {
        Picked::Action(action) => {
            super::super::dispatch::run(out, action, options, harnesses, catalog_root, state_home);
            false
        }
        Picked::Closed => false,
        Picked::Unavailable => {
            super::text::show(out, harnesses, catalog_root, state_home, filter);
            true
        }
    }
}

/// What the overlay decided: a selection, a dismissal, or a terminal that
/// cannot host it -- the last one falls back to the printed picker instead of
/// leaving the command with nothing on screen.
pub enum Picked {
    Action(args::Action),
    Closed,
    Unavailable,
}

pub fn choose(
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
    filter: Option<&str>,
) -> Picked {
    if !crate::tui::screen::active() {
        return Picked::Unavailable;
    }
    // The prompt's guard was dropped when its line returned; the overlay owns
    // the terminal now, and without raw mode every key would wait for Enter.
    let _raw = match crate::tui::term::enable_raw() {
        Some(guard) => guard,
        None => return Picked::Unavailable,
    };
    let active = super::super::fleet::active(state_home);
    let ready = super::super::fleet::ready(harnesses, catalog_root, state_home);
    let indicator = Indicator {
        active: active.clone().unwrap_or_else(|| "none".to_string()),
        debug: false,
    };
    let state = ViewportState::collect(&indicator, harnesses, catalog_root, state_home);
    let mut picker = Picker::new(harnesses, active, ready, filter);
    loop {
        let size = size();
        let rows = picker.rows(size.inner_cols());
        let mut draft = state.base_draft(HINT, &rows, false);
        draft.offset = picker.offset(size.body_rows());
        draft.prompt = format!("filter> {}", picker.filter);
        let cells = visible_width(&draft.prompt) + 1;
        print!("{}", parked(frame(size, &draft), size, cells));
        std::io::stdout().flush().ok();
        match picker.step(read_key().unwrap_or(Key::Dead)) {
            Step::Select(index) => {
                return match harnesses.get(index) {
                    Some(harness) => Picked::Action(args::Action::Use(harness.name.clone())),
                    None => Picked::Closed,
                }
            }
            Step::Close => return Picked::Closed,
            Step::Continue => {}
        }
    }
}
