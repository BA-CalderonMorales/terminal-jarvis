//! Picker: the fleet as an object, not as output. While the overlay is open it
//! owns the keys -- arrows move a cursor, typing narrows the fleet live, Enter
//! selects, Esc closes -- and a digit still jumps by picker position while the
//! filter is empty. The policy is pure so every key class is witnessed without
//! a terminal; `picker_run` owns the painting and the stdin read.

use crate::contracts::Harness;
use crate::tui::input::Key;
use std::collections::BTreeSet;

#[path = "picker_run.rs"]
mod run;
pub use run::{filter, listing};

#[path = "picker_text.rs"]
mod text;

#[path = "picker_view.rs"]
mod view;

pub const HINT: &str = "↑↓ move · type to filter · enter select · esc close";

/// One key's effect, decided without touching the terminal.
pub enum Step {
    Continue,
    Select(usize),
    Close,
}

pub struct Picker<'a> {
    harnesses: &'a [Harness],
    active: Option<String>,
    ready: BTreeSet<String>,
    pub filter: String,
    pub cursor: usize,
}

impl<'a> Picker<'a> {
    pub fn new(
        harnesses: &'a [Harness],
        active: Option<String>,
        ready: BTreeSet<String>,
        filter: Option<&str>,
    ) -> Self {
        Self {
            harnesses,
            active,
            ready,
            filter: filter.unwrap_or("").to_string(),
            cursor: 0,
        }
    }

    pub fn step(&mut self, key: Key) -> Step {
        match key {
            Key::Escape | Key::Dead | Key::Interrupt => return Step::Close,
            Key::Enter => {
                return match self.selected() {
                    Some(harness) => Step::Select(self.position(&harness.name)),
                    None => Step::Continue,
                }
            }
            Key::Up => self.cursor = self.cursor.saturating_sub(1),
            Key::Down => self.cursor = self.cursor.saturating_add(1),
            Key::PageUp => self.cursor = self.cursor.saturating_sub(10),
            Key::PageDown => self.cursor = self.cursor.saturating_add(10),
            Key::Backspace => {
                self.filter.pop();
                self.cursor = 0;
            }
            Key::ClearLine => {
                self.filter.clear();
                self.cursor = 0;
            }
            Key::Char(digit) if self.filter.is_empty() && digit.is_ascii_digit() => {
                let index = digit.to_digit(10).unwrap_or(0) as usize;
                if index > 0 && index <= self.harnesses.len() {
                    return Step::Select(index - 1);
                }
            }
            Key::Char(character) if !character.is_control() => {
                self.filter.push(character);
                self.cursor = 0;
            }
            _ => {}
        }
        self.clamp();
        Step::Continue
    }
}

#[cfg(test)]
#[path = "../tests/picker_fixture.rs"]
mod fixture;

#[cfg(test)]
#[path = "../tests/picker_overlay.rs"]
mod tests;
