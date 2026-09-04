//! ViewportRaw: the interactive prompt session. Raw mode is already on;
//! every key repaints the frame -- editor tail on the prompt row, scroll
//! window on the body -- and Enter hands the committed line back so child
//! runs own the real terminal again.

use crate::contracts::Harness;
use crate::tui::home;
use crate::tui::input::{read_key, Editor, Feed, Indicator};
use crate::tui::screen::{self, Draft};
use std::io::Write;
use std::path::Path;

/// The frozen frame chrome for one prompt session: everything that does not
/// change while keys edit the line or scroll the body.
pub struct ViewportState {
    pub title: String,
    pub status: String,
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
            title: format!("TERMINAL JARVIS v{}", env!("CARGO_PKG_VERSION")),
            status: home::styled(&o),
            prefix_cells: screen::visible_width(&prefix) + 2,
            prefix,
        }
    }

    pub fn base_draft(&self, hint: &str, body: &[String]) -> Draft {
        Draft {
            title: self.title.clone(),
            status: self.status.clone(),
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
}

/// Runs until Enter submits, Ctrl-D ends the session, or EOF arrives.
/// Returns the committed command line, or None when the session is over.
pub fn run(session: &Session) -> Option<String> {
    let mut editor = Editor::default();
    let mut offset = 0;
    loop {
        let size = screen::size();
        let tail = editor.tail_view(session.state.prefix_cells, size.inner_cols());
        let draft = Draft {
            offset,
            prompt: format!("{}{tail}", session.state.prefix),
            ..session.state.base_draft(session.hint, session.body)
        };
        let cells = session.state.prefix_cells + screen::visible_width(&tail);
        let painted = screen::parked(screen::frame(size, &draft), size, cells);
        print!("{painted}");
        std::io::stdout().flush().ok();
        match editor.feed(read_key()?) {
            Feed::Edited | Feed::Idle => {}
            Feed::Moved(toward) => {
                offset = screen::step(offset, toward, session.body.len(), size.body_rows());
            }
            Feed::Submit(line) => return Some(line),
            Feed::Dead => return None,
        }
    }
}
