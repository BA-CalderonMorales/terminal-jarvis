//! Public face of the catalog domain: loading, truth, and validation of the
//! harness catalog.

#[path = "logic/mod.rs"]
mod logic;
#[path = "structs/mod.rs"]
mod structs;

pub use crate::catalog::logic::freshness::status as freshness_status;
pub use crate::catalog::logic::loader::load;
pub(crate) use crate::catalog::logic::parser;
pub use crate::catalog::logic::validate::validate;
