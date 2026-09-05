//! ViewportRaw: the raw prompt session chrome. The frozen header/prefix
//! live here; the mode machine and navigation live in viewport_nav.

use super::viewport_nav::{self, Mode};
use crate::contracts::Harness;
use crate::tui::home;
use crate::tui::input::{read_key, Editor, Indicator};
use std::io::Write;
use std::path::Path;

/// The frozen frame chrome for one prompt session: everything that does
/// not change while keys edit the line or scroll the body.
pub struct ViewportState {
    pub header: String,
    pub cwd: String,
    pub tagline: String,
    pub prefix: String,
    pub prefix_cells: usize,
}

impl ViewportState {
    pub fn collect(
        indicator: &Indicator,
        harnesses: &[Harness],
        catalog_root: &Path,
        state_home: &Path,
    ) -> Self {
        let o = home::collect(harnesses, catalog_root, state_home);
        let prefix = indicator.render(true);
        Self {
            header: home::header(&o),
            cwd: o.cwd,
            tagline: crate::tui::screen::tagline(&o.name, o.ready, o.total),
            prefix_cells: crate::tui::screen::visible_width(&prefix) + 1,
            prefix,
        }
    }

    pub fn base_draft(&self, hint: &str, body: &[String]) -> crate::tui::screen::Draft {
        crate::tui::screen::Draft {
            header: self.header.clone(),
            cwd: self.cwd.clone(),
            tagline: self.tagline.clone(),
            body: body.to_vec(),
            prompt: self.prefix.clone(),
            offset: 0,
            hint: hint.to_string(),
        }
    }
}

/// One raw-mode prompt session over the frozen chrome.
pub struct Session<'a> {
    pub state: &'a ViewportState,
    pub hint: &'a str,
    pub body: &'a [String],
    pub history: &'a [String],
}

pub fn run(session: &Session<'_>) -> Option<String> {
    let mut editor = Editor::default();
    let mut offset =
        crate::tui::screen::max_offset(session.body.len(), crate::tui::screen::size().body_rows());
    let mut mode = Mode::Insert;
    let mut history_at = session.history.len();
    loop {
        let size = crate::tui::screen::size();
        let badge = match mode {
            Mode::Normal => " -- NORMAL --",
            Mode::Insert => "",
        };
        let tail = editor.tail_view(session.state.prefix_cells, size.inner_cols());
        let mut draft = session.state.base_draft(session.hint, session.body);
        draft.offset = offset;
        draft.prompt = format!("{}{badge}{tail}", session.state.prefix);
        let cells = session.state.prefix_cells
            + crate::tui::screen::visible_width(badge)
            + crate::tui::screen::visible_width(&tail);
        let painted =
            crate::tui::screen::parked(crate::tui::screen::frame(size, &draft), size, cells);
        print!("{painted}");
        std::io::stdout().flush().ok();
        match viewport_nav::key(
            &mut mode,
            &mut editor,
            read_key()?,
            session,
            &mut offset,
            history_at,
        ) {
            viewport_nav::Flow::Continue(next_at) => history_at = next_at,
            viewport_nav::Flow::Submit(line) => return Some(line),
            viewport_nav::Flow::Dead => return None,
        }
    }
}
