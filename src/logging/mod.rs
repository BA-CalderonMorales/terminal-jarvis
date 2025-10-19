// Logging Module - Comprehensive Audit and Security Logging
// Provides immutable logging for forensic analysis and monitoring

pub mod audit;
pub mod security;

pub use audit::{AuditLogger, LogCategory, LogEntry, LogLevel};
pub use security::{SecurityEvent, SecurityLogger};
