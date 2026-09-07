//! ConverseLive: the conversation's live surface. One streaming turn:
//! stderr arrives as classified rows in the frame, stdout accumulates as
//! the reply, and navigation keys keep scrolling while the child works --
//! silence included, on the same ticker the rows repaint with.

use crate::contracts::Harness;
use crate::tui::input::Indicator;
use std::path::Path;
use std::time::{Duration, Instant};

/// The frame pieces one turn paints and scrolls.
pub struct Turn<'a> {
    pub body: &'a mut Vec<String>,
    pub offset: &'a mut usize,
    pub indicator: &'a Indicator,
    pub hint: &'a str,
    pub harnesses: &'a [Harness],
    pub catalog_root: &'a Path,
    pub state_home: &'a Path,
}

impl Turn<'_> {
    /// One full frame at the current scroll offset.
    pub fn push_flush(&mut self) {
        super::viewport::paint(
            self.indicator,
            self.hint,
            self.harnesses,
            self.catalog_root,
            self.state_home,
            self.body,
            *self.offset,
        );
    }

    /// Pushes one child line as a body row, following the bottom unless
    /// the user scrolled away, then repaints on the row cadence.
    pub fn push_row(&mut self, row: &str, since_paint: &mut Instant) {
        let rows = crate::tui::screen::size().body_rows();
        let pin = *self.offset == crate::tui::screen::max_offset(self.body.len(), rows);
        self.body.push(row.to_string());
        if pin {
            *self.offset = crate::tui::screen::max_offset(self.body.len(), rows);
        }
        self.repaint(since_paint);
    }

    /// Applies every parked navigation key, repainting on movement.
    pub fn drain_keys(&mut self, since_paint: &mut Instant) {
        let rows = crate::tui::screen::size().body_rows();
        let moved = crate::tui::input::poll::drained()
            .into_iter()
            .any(|key| super::live_nav::navigate(&key, self.offset, self.body, rows));
        if moved {
            self.push_flush();
            *since_paint = Instant::now();
        }
    }

    /// Repaints the frame at the current offset, throttled by the caller.
    pub fn repaint(&mut self, since_paint: &mut Instant) {
        if since_paint.elapsed() > Duration::from_millis(120) {
            self.push_flush();
            *since_paint = Instant::now();
        }
    }
}
