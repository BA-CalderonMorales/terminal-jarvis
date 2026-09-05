//! Key: the decoded input event shape shared by the editor, the navigation
//! table, and the consent gate.

/// One decoded input event from the raw terminal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    Char(char),
    Escape,
    Enter,
    Backspace,
    ClearLine,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Ignored,
    Dead,
}
