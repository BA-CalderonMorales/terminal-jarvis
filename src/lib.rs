pub mod catalog;
pub mod cli;
pub mod context;
pub mod contracts;
pub mod diagnostics;
pub mod distribution;
pub mod gates;
pub mod platform;
pub mod runtime;
pub mod security;

#[cfg(test)]
pub static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
