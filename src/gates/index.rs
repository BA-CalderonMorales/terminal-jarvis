//! Public face of the gates domain: per-invocation gate selection and the
//! loader/runner that apply it.

#[path = "logic/mod.rs"]
mod logic;
#[path = "structs/mod.rs"]
mod structs;

pub use crate::gates::logic::loader::{load, Gate};
pub use crate::gates::logic::runner::{preflight, run};
pub use crate::gates::structs::state::{disable, enable, selected, Selection};

#[cfg(test)]
#[path = "tests/narrate.rs"]
mod narrate_tests;
