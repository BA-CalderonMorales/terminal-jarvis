//! ConverseWire: the shell-side application of the conversation state
//! machine. One turn per frame; the loop repaints between every exchange,
//! with a splunk-style marker while the current speaker is mid-response.

use super::converse_live;
use super::status;
use crate::contracts::Harness;
use std::path::Path;
use std::time::{Duration, Instant};

/// Runs at most one conversation turn: paint the current transcript, mark
/// the responding agent, invoke it, then hand the body delta back.
pub fn tick(
    state: &mut super::state::LoopState,
    harnesses: &[Harness],
    catalog_root: &Path,
    state_home: &Path,
) -> Option<Vec<String>> {
    if !crate::tui::screen::active() {
        return None;
    }
    let thinking = state
        .converse
        .as_ref()
        .map(crate::converse::wire::thinking_line);
    if let Some(marker) = &thinking {
        state.body.push(marker.clone());
    }
    let super::state::LoopState {
        body,
        offset,
        converse,
        indicator,
        hint,
        ..
    } = state;
    let mut turn = converse_live::Turn {
        body,
        offset,
        indicator,
        hint,
        harnesses,
        catalog_root,
        state_home,
    };
    turn.push_flush();
    let mut speak = |name: &str, prompt: &str| speak_turn(&mut turn, harnesses, name, prompt);
    let width = crate::tui::screen::size().inner_cols();
    let lines = crate::converse::wire::pending(converse, &mut speak, width);
    if thinking.is_some() {
        state.body.pop();
    }
    let lines = lines?;
    state.hint = crate::converse::wire::hint(&state.converse)
        .unwrap_or_else(|| status::modeline(state_home, false, state.debug));
    Some(lines)
}

/// Runs one streaming headless turn inside the frame: the reply collects
/// from stdout while stderr paints as rows and keys keep scrolling.
fn speak_turn(
    turn: &mut converse_live::Turn<'_>,
    harnesses: &[Harness],
    name: &str,
    prompt: &str,
) -> Result<String, String> {
    let _raw = crate::tui::term::enable_raw();
    // The watcher owns stdin for the rest of the session: turns drain its
    // parked keys live, and every later reader takes from the same queue.
    crate::tui::input::spawn_watcher();
    let mut running = crate::cli::headless_stream(harnesses, name, prompt)?;
    let mut reply = String::new();
    let mut since_paint = Instant::now();
    loop {
        match running.next(Duration::from_millis(180)) {
            crate::runtime::LiveEvent::Line(crate::runtime::LiveLine::Out(line)) => {
                reply.push_str(&line);
                reply.push('\n');
            }
            crate::runtime::LiveEvent::Line(crate::runtime::LiveLine::Err(line)) => {
                for row in line.split('\n') {
                    turn.push_row(row, &mut since_paint);
                }
            }
            crate::runtime::LiveEvent::Idle => turn.repaint(&mut since_paint),
            crate::runtime::LiveEvent::Done => break,
        }
        turn.drain_keys(&mut since_paint);
    }
    turn.push_flush();
    let code = running.wait();
    if code == 0 {
        Ok(reply.trim_end().to_string())
    } else {
        Err(format!("exit {code}"))
    }
}
