#[path = "catalog/index.rs"]
pub mod catalog;
#[path = "cli/index.rs"]
pub mod cli;
#[path = "context/index.rs"]
pub mod context;
#[path = "contracts/index.rs"]
pub mod contracts;
#[path = "diagnostics/index.rs"]
pub mod diagnostics;
#[path = "gates/index.rs"]
pub mod gates;
#[path = "runtime/index.rs"]
pub mod runtime;
#[path = "security/index.rs"]
pub mod security;
#[path = "tui/lib.rs"]
pub mod tui;

#[cfg(test)]
pub static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
