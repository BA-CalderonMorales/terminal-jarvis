//! Integration tests for the cli domain.
//!
//! Each file under `tests/` witnesses one behavior surface of `src/cli/`;
//! `logic/` holds the disposable fixtures that drive the binary in a hermetic
//! environment, and `structs/` the fixture data shapes they write.

mod logic;
mod structs;
mod tests;
