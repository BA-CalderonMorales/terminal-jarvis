//! ViewportNav: vim-style navigation. Normal scrolls; Insert types.
use super::viewport_page::page;
use super::viewport_raw::Session;
use crate::tui::input::{Editor, Feed, Key, Move};

/// The vim-style prompt modes.
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

/// Replaces the editor buffer with a recalled history entry.
pub fn recall(editor: &mut Editor, text: Option<&String>) {
    editor.feed(Key::ClearLine);
    if let Some(text) = text {
        for character in text.chars() {
            editor.feed(Key::Char(character));
        }
    }
}

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
        (Mode::Normal, Key::Char('j') | Key::Down | Key::Char('k') | Key::Up) => {
            let toward = if matches!(key, Key::Char('k') | Key::Up) {
                Move::Older
            } else {
                Move::Newer
            };
            *offset = crate::tui::screen::step(*offset, toward, session.body.len(), rows);
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
        (Mode::Normal, Key::Char(mark @ ('g' | 'G'))) => {
            *offset = match mark {
                'G' => crate::tui::screen::max_offset(session.body.len(), rows),
                _ => 0,
            };
            Flow::Continue(history_at)
        }
        (Mode::Normal, Key::Escape | Key::Char('i')) => {
            // The way back: without it Normal is a trap, where nothing typed
            // reaches the line and no key returns to Insert.
            *mode = Mode::Insert;
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
            Feed::Edited | Feed::Idle => Flow::Continue(history_at),
            Feed::Moved(toward) => {
                *offset = crate::tui::screen::step(*offset, toward, session.body.len(), rows);
                Flow::Continue(history_at)
            }
            Feed::Submit(line) => Flow::Submit(line),
            Feed::Dead => Flow::Dead,
        },
    }
}

#[cfg(test)]
#[path = "../tests/viewport_nav.rs"]
mod tests;
