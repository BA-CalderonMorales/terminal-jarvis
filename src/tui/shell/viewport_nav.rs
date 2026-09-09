//! ViewportNav: vim-style navigation. Normal mode scrolls; Insert mode types,
//! recalls history, and hands Esc back to Normal. One key, one decision.
use super::viewport_page::page;
use super::viewport_raw::Session;
use crate::tui::input::{Editor, Key, Move};

/// The vim-style prompt modes: Normal navigates, Insert types.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

/// What one key did to the session.
pub enum Flow {
    Continue(usize),
    Submit(String),
    Dead,
}

/// Replaces the editor buffer with a recalled history entry; an empty entry
/// clears the line, the way a shell returns to the live prompt.
pub fn recall(editor: &mut Editor, text: Option<&String>) {
    editor.feed(Key::ClearLine);
    if let Some(text) = text {
        for character in text.chars() {
            editor.feed(Key::Char(character));
        }
    }
}

/// Applies one key to the session; scrolls move the offset, and submit/death
/// end the session.
pub fn key(
    mode: &mut Mode,
    editor: &mut Editor,
    key: Key,
    session: &mut Session<'_>,
    history_at: usize,
) -> Flow {
    let rows = crate::tui::screen::size().body_rows();
    let offset = &mut *session.offset;
    match (*mode, key) {
        (Mode::Normal, Key::Char('j') | Key::Down) => {
            *offset = crate::tui::screen::step(*offset, Move::Newer, session.body.len(), rows);
            Flow::Continue(history_at)
        }
        (Mode::Normal, Key::Char('k') | Key::Up) => {
            *offset = crate::tui::screen::step(*offset, Move::Older, session.body.len(), rows);
            Flow::Continue(history_at)
        }
        (Mode::Normal, Key::PageUp) => {
            *offset = page(session.body, *offset, false);
            Flow::Continue(history_at)
        }
        (Mode::Normal, Key::PageDown) => {
            *offset = page(session.body, *offset, true);
            Flow::Continue(history_at)
        }
        (Mode::Normal, Key::Char('g')) => {
            *offset = 0;
            Flow::Continue(history_at)
        }
        (Mode::Normal, Key::Char('G')) => {
            *offset = crate::tui::screen::max_offset(session.body.len(), rows);
            Flow::Continue(history_at)
        }
        (Mode::Normal, Key::Dead) => Flow::Dead,
        (Mode::Normal, _) => Flow::Continue(history_at),
        (Mode::Insert, Key::Escape) => {
            *mode = Mode::Normal;
            Flow::Continue(history_at)
        }
        (Mode::Insert, Key::Up) => {
            let at = history_at.saturating_sub(1);
            recall(editor, session.history.get(at));
            Flow::Continue(at)
        }
        (Mode::Insert, Key::Down) => {
            let at = (history_at + 1).min(session.history.len());
            recall(editor, session.history.get(at));
            Flow::Continue(at)
        }
        (Mode::Insert, key) => match editor.feed(key) {
            crate::tui::input::Feed::Edited | crate::tui::input::Feed::Idle => {
                Flow::Continue(history_at)
            }
            crate::tui::input::Feed::Moved(toward) => {
                *offset = crate::tui::screen::step(*offset, toward, session.body.len(), rows);
                Flow::Continue(history_at)
            }
            crate::tui::input::Feed::Submit(line) => Flow::Submit(line),
            crate::tui::input::Feed::Dead => Flow::Dead,
        },
    }
}

#[cfg(test)]
#[path = "../tests/viewport_nav.rs"]
mod tests;
