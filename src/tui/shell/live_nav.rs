//! LiveNav: the scroll keys that work while a conversation streams. The
//! offset counts lines hidden below the window -- zero is the newest row,
//! max is the oldest -- and the same numbers drive the prompt session, so
//! turns and prompts scroll one view.

use crate::tui::input::{Key, Move};

/// One navigation key's effect on the scroll offset; true when it moved.
pub fn navigate(key: &Key, offset: &mut usize, body: &[String], rows: usize) -> bool {
    match key {
        Key::Char('j') | Key::Down => {
            *offset = crate::tui::screen::step(*offset, Move::Newer, body.len(), rows);
            true
        }
        Key::Char('k') | Key::Up => {
            *offset = crate::tui::screen::step(*offset, Move::Older, body.len(), rows);
            true
        }
        Key::PageDown => {
            *offset = super::viewport_page::page(body, *offset, true);
            true
        }
        Key::PageUp => {
            *offset = super::viewport_page::page(body, *offset, false);
            true
        }
        Key::Char('g') => {
            *offset = 0;
            true
        }
        Key::Char('G') => {
            *offset = crate::tui::screen::max_offset(body.len(), rows);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "../tests/live_scroll.rs"]
mod live_scroll_tests;
