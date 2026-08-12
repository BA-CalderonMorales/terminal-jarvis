//! Public face of the diagnostics domain: collecting and rendering the
//! environment report.

#[path = "logic/mod.rs"]
mod logic;
#[path = "structs/mod.rs"]
mod structs;

pub use crate::diagnostics::logic::collect::collect;
pub use crate::diagnostics::logic::redact_process::{
    path as redact_process_path, text as redact_process_text,
};
pub use crate::diagnostics::structs::environment::{Environment, ValueState};
pub use crate::diagnostics::structs::input::{
    DiagnosticInput, HarnessInput, PlatformInput, RuntimeInput,
};
pub use crate::diagnostics::structs::model::{Code, Record, Report, Severity};
