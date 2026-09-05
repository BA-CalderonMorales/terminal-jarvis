//! Public face of the runtime domain: the harness execution loop.

#[path = "logic/mod.rs"]
mod logic;

pub use crate::runtime::logic::agent_loop::{next_step, planned_steps};
pub use crate::runtime::logic::live::Running as LiveRunning;
pub use crate::runtime::logic::live::{spawn as spawn_live, Event as LiveEvent, Line as LiveLine};
pub use crate::runtime::logic::runner::{run_command, run_command_text};
pub use crate::runtime::logic::stream::{classify, run as run_streaming, stamp};
