// Audit Logging Module - Immutable Forensic Logging
// Provides comprehensive logging for security analysis and compliance

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    timestamp: DateTime<Utc>,
    level: LogLevel,
    category: LogCategory,
    message: String,
    context: serde_json::Value,
    user_id: Option<String>,
    session_id: String,
    request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogCategory {
    Security,
    ModelAccess,
    CommandExecution,
    InputValidation,
    System,
    Network,
    Error,
}

pub struct AuditLogger {
    log_file: File,
    session_id: String,
}

impl AuditLogger {
    pub fn new() -> Result<Self> {
        let log_dir = PathBuf::from("./logs");
        std::fs::create_dir_all(&log_dir)?;

        let log_file_path = log_dir.join(format!(
            "terminal-jarvis-audit-{}.log",
            chrono::Utc::now().format("%Y%m%d")
        ));

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)?;

        let session_id = generate_session_id();

        Ok(Self {
            log_file,
            session_id,
        })
    }

    pub fn log(&mut self, entry: LogEntry) -> Result<()> {
        let log_line = serde_json::to_string(&entry)?;
        writeln!(self.log_file, "{log_line}")?;
        self.log_file.flush()?;

        // Also log to stderr for immediate visibility
        eprintln!("[AUDIT] {log_line}");

        Ok(())
    }

    pub fn log_security_event(
        &mut self,
        level: LogLevel,
        message: &str,
        context: serde_json::Value,
    ) -> Result<()> {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level,
            category: LogCategory::Security,
            message: message.to_string(),
            context,
            user_id: None, // TODO: Get from auth context
            session_id: self.session_id.clone(),
            request_id: generate_request_id(),
        };

        self.log(entry)
    }

    pub fn log_input_validation(
        &mut self,
        input: &str,
        context: &str,
        is_valid: bool,
        reason: Option<&str>,
    ) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "input".to_string(),
            serde_json::Value::String(input.to_string()),
        );
        context_data.insert(
            "context".to_string(),
            serde_json::Value::String(context.to_string()),
        );
        context_data.insert("is_valid".to_string(), serde_json::Value::Bool(is_valid));

        if let Some(reason) = reason {
            context_data.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
        }

        let level = if is_valid {
            LogLevel::Info
        } else {
            LogLevel::Warning
        };
        let message = if is_valid {
            format!("Input validation passed for context: {context}")
        } else {
            format!("Input validation blocked for context: {context}")
        };

        self.log_security_event(level, &message, serde_json::Value::Object(context_data))
    }

    pub fn log_model_access_attempt(&mut self, model_name: &str) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "model_name".to_string(),
            serde_json::Value::String(model_name.to_string()),
        );

        self.log_security_event(
            LogLevel::Info,
            &format!("Model access attempt: {model_name}"),
            serde_json::Value::Object(context_data),
        )
    }

    pub fn log_model_access_success(&mut self, model_name: &str) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "model_name".to_string(),
            serde_json::Value::String(model_name.to_string()),
        );

        self.log_security_event(
            LogLevel::Info,
            &format!("Model access successful: {model_name}"),
            serde_json::Value::Object(context_data),
        )
    }

    pub fn log_model_access_blocked(&mut self, model_name: &str, reason: &str) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "model_name".to_string(),
            serde_json::Value::String(model_name.to_string()),
        );
        context_data.insert(
            "reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );

        self.log_security_event(
            LogLevel::Warning,
            &format!("Model access blocked: {model_name}"),
            serde_json::Value::Object(context_data),
        )
    }

    pub fn log_command_execution_attempt(&mut self, command: &str, args: &[String]) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "command".to_string(),
            serde_json::Value::String(command.to_string()),
        );
        context_data.insert(
            "args".to_string(),
            serde_json::Value::Array(
                args.iter()
                    .map(|arg| serde_json::Value::String(arg.clone()))
                    .collect(),
            ),
        );

        self.log_security_event(
            LogLevel::Info,
            &format!("Command execution attempt: {command}"),
            serde_json::Value::Object(context_data),
        )
    }

    pub fn log_command_execution_blocked(&mut self, command: &str, args: &[String]) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "command".to_string(),
            serde_json::Value::String(command.to_string()),
        );
        context_data.insert(
            "args".to_string(),
            serde_json::Value::Array(
                args.iter()
                    .map(|arg| serde_json::Value::String(arg.clone()))
                    .collect(),
            ),
        );

        self.log_security_event(
            LogLevel::Warning,
            &format!("Command execution blocked: {command}"),
            serde_json::Value::Object(context_data),
        )
    }

    pub fn log_download_attempt(&mut self, url: &str, destination: &str) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "url".to_string(),
            serde_json::Value::String(url.to_string()),
        );
        context_data.insert(
            "destination".to_string(),
            serde_json::Value::String(destination.to_string()),
        );

        self.log_security_event(
            LogLevel::Info,
            &format!("Download attempt: {url}"),
            serde_json::Value::Object(context_data),
        )
    }

    pub fn log_download_blocked(&mut self, url: &str, reason: &str) -> Result<()> {
        let mut context_data = serde_json::Map::new();
        context_data.insert(
            "url".to_string(),
            serde_json::Value::String(url.to_string()),
        );
        context_data.insert(
            "reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );

        self.log_security_event(
            LogLevel::Warning,
            &format!("Download blocked: {url}"),
            serde_json::Value::Object(context_data),
        )
    }

    pub fn log_suspicious_activity(
        &mut self,
        activity: &str,
        details: serde_json::Value,
    ) -> Result<()> {
        self.log_security_event(
            LogLevel::Critical,
            &format!("Suspicious activity detected: {activity}"),
            details,
        )
    }

    // pub fn get_recent_security_events(&self, _limit: usize) -> Result<Vec<LogEntry>> {
    //     // Read recent log entries and filter for security events
    //     // Implementation depends on log storage format
    //     Ok(vec![]) // TODO: Implement log reading functionality
    // }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            eprintln!("[Warning] Failed to initialize audit logger: {e}");
            // Create a fallback logger that writes to /dev/null (or NUL on Windows)
            #[cfg(unix)]
            let null_path = "/dev/null";
            #[cfg(windows)]
            let null_path = "NUL";

            let log_file = std::fs::OpenOptions::new()
                .write(true)
                .open(null_path)
                .unwrap_or_else(|_| {
                    // Last resort: create a temp file
                    std::fs::File::create(std::env::temp_dir().join("jarvis-audit-fallback.log"))
                        .expect("Cannot create any log file - system may be read-only")
                });

            Self {
                log_file,
                session_id: "fallback".to_string(),
            }
        })
    }
}

fn generate_session_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    std::process::id().hash(&mut hasher);
    chrono::Utc::now()
        .timestamp_nanos_opt()
        .unwrap_or(0)
        .hash(&mut hasher);

    format!("session_{}", hasher.finish())
}

fn generate_request_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    chrono::Utc::now()
        .timestamp_nanos_opt()
        .unwrap_or(0)
        .hash(&mut hasher);
    // Use thread ID and a counter for uniqueness instead of rand crate
    std::thread::current().id().hash(&mut hasher);
    // Add pointer address of a stack variable for additional entropy
    let stack_var: u8 = 0;
    ((&stack_var as *const u8) as usize).hash(&mut hasher);

    format!("req_{}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: LogLevel::Info,
            category: LogCategory::Security,
            message: "Test message".to_string(),
            context: serde_json::json!({"key": "value"}),
            user_id: None,
            session_id: "test_session".to_string(),
            request_id: "test_request".to_string(),
        };

        let serialized = serde_json::to_string(&entry);
        assert!(serialized.is_ok());
    }
}
