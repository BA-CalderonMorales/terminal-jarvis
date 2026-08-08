//! Public face of the gates domain: per-invocation gate selection and the
//! loader/runner that apply it.

mod logic;
mod structs;

pub use crate::gates::logic::loader::{load, Gate};
pub use crate::gates::logic::runner::{preflight, run};
pub use crate::gates::structs::state::{disable, enable, selected, Selection};
