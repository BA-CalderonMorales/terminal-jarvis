//! Screen: the full-viewport command center.

#[path = "structs/mod.rs"]
mod structs;

#[path = "logic/canvas.rs"]
mod canvas;

#[path = "logic/art.rs"]
mod art;

#[path = "logic/paint.rs"]
mod paint;

#[path = "logic/sanitize.rs"]
mod sanitize;

#[path = "index.rs"]
mod index;

pub use art::welcome;
pub use canvas::visible_width;
pub use index::{active, boot, ensure_usable, resume, size, suspend, Session};
pub use paint::{frame, parked, Draft};
pub use sanitize::{is_plain, keep_color};
pub use structs::Size;
