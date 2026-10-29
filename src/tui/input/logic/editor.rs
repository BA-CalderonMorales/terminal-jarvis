//! Editor: the one-line buffer behind the viewport prompt. Pure mapping
//! from decoded keys to text edits, scroll intents, submit, or session end.

use super::structs::Key;

/// A scroll intent produced by the navigation keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Move {
    Older,
    Newer,
    Top,
    Bottom,
    PageOlder,
    PageNewer,
}

/// What one key did to the editor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Feed {
    Edited,
    Idle,
    Moved(Move),
    Submit(String),
    Dead,
}

/// The draft command line; chars are pushed and popped whole so unicode never
/// splits mid-glyph.
#[derive(Default)]
pub struct Editor {
    buf: String,
}

impl Editor {
    pub fn feed(&mut self, key: Key) -> Feed {
        match key {
            crate::tui::input::Key::Escape => Feed::Idle,
            Key::Char(c) => {
                self.buf.push(c);
                Feed::Edited
            }
            Key::Backspace => {
                self.buf.pop();
                Feed::Edited
            }
            Key::ClearLine => {
                self.buf.clear();
                Feed::Edited
            }
            Key::Enter => Feed::Submit(std::mem::take(&mut self.buf)),
            Key::Up => Feed::Moved(Move::Older),
            Key::Down => Feed::Moved(Move::Newer),
            Key::PageUp => Feed::Moved(Move::PageOlder),
            Key::PageDown => Feed::Moved(Move::PageNewer),
            Key::Home => Feed::Moved(Move::Top),
            Key::End => Feed::Moved(Move::Bottom),
            Key::Interrupt => Feed::Idle,
            Key::Resized => Feed::Idle,
            Key::Ignored => Feed::Idle,
            Key::Dead => Feed::Dead,
        }
    }

    /// The whole draft line, as completion sees it.
    pub fn line(&self) -> &str {
        &self.buf
    }

    /// Replaces the line wholesale (completion); whole strings, no split glyph.
    pub fn rewrite(&mut self, text: &str) {
        self.buf = text.to_string();
    }

    /// Replaces the line with a recalled history entry.
    pub fn recall(&mut self, text: Option<&String>) {
        self.feed(Key::ClearLine);
        if let Some(text) = text {
            for character in text.chars() {
                self.feed(Key::Char(character));
            }
        }
    }

    /// The visible tail that fits beside a prefix of
    /// `prefix_cells` within `inner` -- cursor-at-end editing.
    pub fn tail_view(&self, prefix_cells: usize, inner: usize) -> String {
        let budget = inner.saturating_sub(prefix_cells);
        let mut width = 0;
        let mut view = String::new();
        for c in self.buf.chars().rev() {
            let cells = crate::cli::char_cells(c);
            if width + cells > budget {
                break;
            }
            width += cells;
            view.insert(0, c);
        }
        view
    }
}
