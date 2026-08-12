//! Public face of the context domain: environment and session state.
//!
//! Consumers call `context::default_home()`, `context::Session`, or the
//! `context::{platform, distribution}` helpers; internals stay behind
//! `logic/` and `structs/`.

#[path = "constants/mod.rs"]
mod constants;
#[path = "logic/mod.rs"]
mod logic;
#[path = "structs/mod.rs"]
mod structs;

pub use crate::context::logic::distribution;
pub use crate::context::logic::gates::gates_root;
pub use crate::context::logic::platform;
pub use crate::context::logic::session::{catalog_root, default_home, load, save};
pub use crate::context::structs::session::Session;
