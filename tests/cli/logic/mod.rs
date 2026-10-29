pub mod assert;
pub mod child;
pub mod cli_driver;
#[cfg(unix)]
pub mod frames;
pub mod golden;
pub mod matrix;
pub mod platform;
#[cfg(unix)]
pub mod pty;
// pty_io is the PTY reader owned by `pty`; both are unix-only, so the helper
// is scoped the same way rather than left dead on Windows.
#[cfg(unix)]
mod pty_child;
#[cfg(unix)]
mod pty_io;
#[cfg(unix)]
mod pty_tree;
pub mod screen;
pub mod tui;
pub mod width;
