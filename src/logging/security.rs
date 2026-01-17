// Security Logging Module - Specialized Security Event Logging
// Provides focused logging for security events and monitoring

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub description: String,
    pub source: String,
    pub context: serde_json::Value,
    pub blocked: bool,
    pub action_taken: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    InputValidationFailed,
    ModelAccessBlocked,
    CommandExecutionBlocked,
    DownloadBlocked,
    SuspiciousActivity,
    UnauthorizedAccess,
    IntegrityFailure,
    SignatureVerificationFailed,
    SecurityConfigChanged,
    ModelAccessAttempt,
    CommandExecutionAttempt,
    DownloadAttempt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct SecurityLogger {
    events: Vec<SecurityEvent>,
    max_events: usize,
    log_to_file: bool,
    log_file_path: Option<std::path::PathBuf>,
}

impl SecurityLogger {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_events: 10000,
            log_to_file: true,
            log_file_path: Some(std::path::PathBuf::from("./logs/security-events.jsonl")),
        }
    }

    pub fn log_event(&mut self, event: SecurityEvent) -> Result<()> {
        // Add to memory
        self.events.push(event.clone());

        // Trim if too many events
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }

        // Log to file if enabled
        if self.log_to_file {
            if let Some(ref path) = self.log_file_path {
                self.write_to_file(&event, path)?;
            }
        }

        // Log to stderr for immediate visibility
        self.log_to_stderr(&event);

        Ok(())
    }

    pub fn log_validation_attempt(&mut self, input: &str, context: &str) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::InputValidationFailed,
            severity: SecuritySeverity::Low,
            description: format!("Input validation attempt for context: {context}"),
            source: "security_validator".to_string(),
            context: serde_json::json!({
                "input": input,
                "context": context,
                "input_length": input.len()
            }),
            blocked: false, // Not blocked yet, just attempt
            action_taken: None,
        };

        let _ = self.log_event(event);
    }

    pub fn log_blocked_input(&mut self, input: &str, context: &str) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::InputValidationFailed,
            severity: SecuritySeverity::Medium,
            description: format!("Blocked input for context: {context}"),
            source: "security_validator".to_string(),
            context: serde_json::json!({
                "input": input,
                "context": context,
                "blocked_at": "validation_stage"
            }),
            blocked: true,
            action_taken: Some("input_blocked".to_string()),
        };

        let _ = self.log_event(event);
    }

    pub fn log_model_access_attempt(&mut self, model_name: &str) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::ModelAccessAttempt,
            severity: SecuritySeverity::Low,
            description: format!("Model access attempt: {model_name}"),
            source: "secure_model_loader".to_string(),
            context: serde_json::json!({
                "model_name": model_name
            }),
            blocked: false,
            action_taken: None,
        };

        let _ = self.log_event(event);
    }

    pub fn log_model_access_success(&mut self, model_name: &str) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::ModelAccessAttempt,
            severity: SecuritySeverity::Low,
            description: format!("Model access successful: {model_name}"),
            source: "secure_model_loader".to_string(),
            context: serde_json::json!({
                "model_name": model_name,
                "result": "success"
            }),
            blocked: false,
            action_taken: Some("model_loaded".to_string()),
        };

        let _ = self.log_event(event);
    }

    pub fn log_model_access_blocked(&mut self, model_name: &str, reason: &str) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::ModelAccessBlocked,
            severity: SecuritySeverity::High,
            description: format!("Model access blocked: {model_name} - {reason}"),
            source: "secure_model_loader".to_string(),
            context: serde_json::json!({
                "model_name": model_name,
                "block_reason": reason
            }),
            blocked: true,
            action_taken: Some("model_access_denied".to_string()),
        };

        let _ = self.log_event(event);
    }

    pub fn log_command_execution_attempt(&mut self, command: &str, args: &[String]) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::CommandExecutionAttempt,
            severity: SecuritySeverity::Low,
            description: format!("Command execution attempt: {command}"),
            source: "security_validator".to_string(),
            context: serde_json::json!({
                "command": command,
                "args": args,
                "arg_count": args.len()
            }),
            blocked: false,
            action_taken: None,
        };

        let _ = self.log_event(event);
    }

    pub fn log_command_execution_blocked(&mut self, command: &str, args: &[String]) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::CommandExecutionBlocked,
            severity: SecuritySeverity::High,
            description: format!("Command execution blocked: {command}"),
            source: "security_validator".to_string(),
            context: serde_json::json!({
                "command": command,
                "args": args,
                "blocked_at": "command_validation"
            }),
            blocked: true,
            action_taken: Some("command_execution_denied".to_string()),
        };

        let _ = self.log_event(event);
    }

    pub fn log_download_attempt(&mut self, url: &str, destination: &str) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::DownloadAttempt,
            severity: SecuritySeverity::Medium,
            description: format!("Download attempt: {url}"),
            source: "supply_chain_security".to_string(),
            context: serde_json::json!({
                "url": url,
                "destination": destination
            }),
            blocked: false,
            action_taken: None,
        };

        let _ = self.log_event(event);
    }

    pub fn log_download_blocked(&mut self, url: &str, reason: &str) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::DownloadBlocked,
            severity: SecuritySeverity::Critical,
            description: format!("Download blocked: {url} - {reason}"),
            source: "supply_chain_security".to_string(),
            context: serde_json::json!({
                "url": url,
                "block_reason": reason
            }),
            blocked: true,
            action_taken: Some("download_denied".to_string()),
        };

        let _ = self.log_event(event);
    }

    pub fn log_suspicious_activity(&mut self, activity: &str, details: serde_json::Value) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::SuspiciousActivity,
            severity: SecuritySeverity::Critical,
            description: format!("Suspicious activity detected: {activity}"),
            source: "security_monitor".to_string(),
            context: details,
            blocked: true,
            action_taken: Some("alert_raised".to_string()),
        };

        let _ = self.log_event(event);
    }

    fn write_to_file(&self, event: &SecurityEvent, path: &std::path::Path) -> Result<()> {
        use std::io::Write;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        let line = serde_json::to_string(event)?;
        writeln!(file, "{line}")?;

        Ok(())
    }

    fn log_to_stderr(&self, event: &SecurityEvent) {
        let severity_symbol = match event.severity {
            SecuritySeverity::Low => "[INFO]",
            SecuritySeverity::Medium => "[WARN]",
            SecuritySeverity::High => "[ERROR]",
            SecuritySeverity::Critical => "[CRITICAL]",
        };

        let status = if event.blocked { "BLOCKED" } else { "ALLOWED" };

        eprintln!(
            "{} [SECURITY] {} {} - {} ({})",
            event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            severity_symbol,
            event.description,
            status,
            event.source
        );
    }

    pub fn get_recent_events(&self, limit: usize) -> &[SecurityEvent] {
        let start = if self.events.len() > limit {
            self.events.len() - limit
        } else {
            0
        };

        &self.events[start..]
    }

    pub fn get_events_by_type(&self, event_type: &SecurityEventType) -> Vec<&SecurityEvent> {
        self.events
            .iter()
            .filter(|event| {
                std::mem::discriminant(&event.event_type) == std::mem::discriminant(event_type)
            })
            .collect()
    }

    pub fn get_security_status(&self) -> serde_json::Value {
        let total_events = self.events.len();
        let blocked_events = self.events.iter().filter(|e| e.blocked).count();

        let mut type_counts = HashMap::new();
        let mut severity_counts = HashMap::new();

        for event in &self.events {
            let type_key = format!("{:?}", event.event_type);
            let severity_key = format!("{:?}", event.severity);

            *type_counts.entry(type_key).or_insert(0) += 1;
            *severity_counts.entry(severity_key).or_insert(0) += 1;
        }

        serde_json::json!({
            "total_events": total_events,
            "blocked_events": blocked_events,
            "allowed_events": total_events - blocked_events,
            "events_by_type": type_counts,
            "events_by_severity": severity_counts,
            "recent_critical_events": self.events.iter()
                .filter(|e| matches!(e.severity, SecuritySeverity::Critical))
                .count(),
            "log_file_path": self.log_file_path
        })
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

impl Default for SecurityLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_event_serialization() {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::InputValidationFailed,
            severity: SecuritySeverity::Medium,
            description: "Test event".to_string(),
            source: "test".to_string(),
            context: serde_json::json!({"test": true}),
            blocked: true,
            action_taken: Some("test_action".to_string()),
        };

        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_security_logger() {
        let mut logger = SecurityLogger::new();

        logger.log_validation_attempt("test_input", "test_context");

        assert_eq!(logger.events.len(), 1);

        let events = logger.get_recent_events(10);
        assert_eq!(events.len(), 1);
    }
}
