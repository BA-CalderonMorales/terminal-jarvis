//! Public face of the gates domain: per-invocation gate selection and the
//! loader/runner that apply it.

#[path = "logic/mod.rs"]
mod logic;
#[path = "structs/mod.rs"]
mod structs;

pub use crate::gates::logic::interrupt::active_pid as active_child_pid;
pub use crate::gates::logic::loader::{load, Gate};
pub use crate::gates::logic::runner::preflight;
pub use crate::gates::logic::stream::run;
pub use crate::gates::logic::verdict::{interrupted_message, verdict_for, Verdict};
pub use crate::gates::structs::state::{disable, enable, selected, Selection};

#[cfg(test)]
#[path = "tests/interrupt.rs"]
mod interrupt_tests;
#[cfg(test)]
#[path = "tests/narrate.rs"]
mod narrate_tests;
#[cfg(test)]
#[path = "tests/stream.rs"]
mod stream_tests;
#[cfg(test)]
#[path = "tests/util.rs"]
mod tests_util;
