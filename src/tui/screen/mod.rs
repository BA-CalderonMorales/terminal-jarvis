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

#[path = "logic/roles.rs"]
mod roles;

#[path = "logic/theme.rs"]
mod theme;

#[path = "logic/view.rs"]
mod view;

#[path = "logic/sanitize.rs"]
mod sanitize;

#[path = "logic/tagline.rs"]
mod tagline;

#[path = "logic/scroll.rs"]
mod scroll;

#[path = "index.rs"]
mod index;

pub use art::welcome;
pub use canvas::{clip_line, fit, visible_width};
pub use index::{
    accent, accent2, active, active_theme, apply_theme, boot, cycle_theme, dim, ensure_usable,
    resume, size, suspend, theme_names, Session,
};
pub use paint::{frame, parked, Draft};
pub use roles::Role;
pub use sanitize::{is_plain, keep_color};
pub use scroll::{badge, clamp, max_offset, step, window};
pub use structs::Size;
pub use tagline::tagline;
pub use theme::paint;
pub use view::{block_width, chip, pair, section};
