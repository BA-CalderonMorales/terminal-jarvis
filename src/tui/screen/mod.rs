//! Screen: the full-viewport command center.

#[path = "structs/mod.rs"]
mod structs;

#[path = "logic/canvas.rs"]
mod canvas;

#[path = "logic/art.rs"]
mod art;

#[path = "logic/layout.rs"]
mod layout;

#[path = "logic/modeline.rs"]
mod modeline;

#[path = "logic/paint.rs"]
mod paint;

#[path = "logic/palettes.rs"]
mod palettes;

#[path = "logic/theme.rs"]
mod theme;

#[path = "logic/sanitize.rs"]
mod sanitize;

#[path = "logic/scroll.rs"]
mod scroll;

#[path = "index.rs"]
mod index;

pub use art::{tagline, welcome};
pub use canvas::visible_width;
pub use index::{
    accent, accent2, active, active_theme, apply_theme, boot, cycle_theme, dim, ensure_usable,
    resume, size, suspend, theme_names, Session,
};
pub use paint::{frame, parked, Draft};
pub use sanitize::{is_plain, keep_color};
pub use scroll::{badge, clamp, max_offset, step, window};
pub use structs::Size;
