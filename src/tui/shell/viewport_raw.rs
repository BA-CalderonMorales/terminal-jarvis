//! ViewportRaw: one raw-mode prompt session over the frozen chrome. The
//! scroll offset lives in the caller's state (turns and prompts share one).

use super::viewport_chrome::ViewportState;
use super::viewport_nav::{self, Mode};
use crate::tui::input::{read_key, Editor, Key};
use std::io::Write;

/// One raw-mode prompt session over the frozen chrome. The scroll offset lives
/// in the caller's state (turns and prompts share one position).
pub struct Session<'a> {
    pub state: &'a ViewportState,
    pub hint: &'a str,
    pub body: &'a [String],
    pub history: &'a [String],
    pub offset: &'a mut usize,
    /// The harness names Tab completes against.
    pub names: &'a [String],
}

pub fn run(session: &mut Session<'_>, bottom: bool) -> Option<String> {
    let mut editor = Editor::default();
    let mut mode = Mode::Insert;
    let mut history_at = session.history.len();
    loop {
        let size = crate::tui::screen::size();
        let badge = match mode {
            Mode::Normal => " -- NORMAL --",
            Mode::Insert => "",
        };
        let tail = editor.tail_view(session.state.prefix_cells, size.inner_cols());
        // The modeline drops a hint that will not fit beside the badge.
        let hint = match mode {
            Mode::Normal => "esc or i to type",
            Mode::Insert => session.hint,
        };
        let mut draft = session.state.base_draft(hint, session.body, bottom);
        draft.offset = *session.offset;
        draft.prompt = format!("{}{badge}{tail}", session.state.prefix);
        let cells = session.state.prefix_cells
            + crate::tui::screen::visible_width(badge)
            + crate::tui::screen::visible_width(&tail);
        let painted =
            crate::tui::screen::parked(crate::tui::screen::frame(size, &draft), size, cells);
        print!("{painted}");
        std::io::stdout().flush().ok();
        let next = read_key().unwrap_or(Key::Dead);
        // A resize moves the scroll window under the stored offset; re-clamp.
        if next == Key::Resized {
            let rows = crate::tui::screen::size().body_rows();
            *session.offset = crate::tui::screen::clamp(*session.offset, session.body.len(), rows);
        }
        match viewport_nav::key(&mut mode, &mut editor, next, session, history_at) {
            viewport_nav::Flow::Continue(next_at) => history_at = next_at,
            viewport_nav::Flow::Submit(line) => return Some(line),
            viewport_nav::Flow::Dead => return None,
        }
    }
}
