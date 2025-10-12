// Core Security Module - Input Validation and Allowlist Management
// Implements zero-trust validation for all external inputs

use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SecurityError {
    pub message: String,
    pub code: SecurityErrorCode,
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Security Error [{}]: {}", self.code, self.message)
    }
}

impl std::error::Error for SecurityError {}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityErrorCode {
    InvalidInput,
    BlockedKeyword,
    UnallowedCommand,
    PathTraversal,
    InjectionAttempt,
    LengthExceeded,
    FormatInvalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    max_input_length: usize,
    blocked_keywords: Vec<String>,
    allowed_commands: Vec<String>,
    allowed_paths: Vec<String>,
    strict_mode: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_input_length: 1000,
            blocked_keywords: vec![
                // System commands
                "rm".to_string(),
                "sudo".to_string(),
                "su".to_string(),
                "chmod".to_string(),
                "chown".to_string(),
                
                // Network/download commands
                "wget".to_string(),
                "curl".to_string(),
                "nc".to_string(),
                "netcat".to_string(),
                "ssh".to_string(),
                "scp".to_string(),
                "rsync".to_string(),
                "git".to_string(),
                
                // Container/orchestration
                "docker".to_string(),
                "kubectl".to_string(),
                
                // Browser protocols - CRITICAL for ecosystem security
                "chrome://".to_string(),
                "chrome-extension://".to_string(),
                "edge://".to_string(),
                "edge-extension://".to_string(),
                "firefox://".to_string(),
                "about:".to_string(),           // Firefox about: pages
                "resource://".to_string(),      // Firefox resource protocol
                "jar:file://".to_string(),      // Firefox JAR protocol
                "moz-extension://".to_string(), // Firefox extensions
                "safari://".to_string(),
                "webkit://".to_string(),
                "opera://".to_string(),
                "brave://".to_string(),
                "vivaldi://".to_string(),
                "tor://".to_string(),
                "chromium://".to_string(),
                "ms-browser-extension://".to_string(),
                
                // Internal app protocols
                "app://".to_string(),
                "application://".to_string(),
                "file://".to_string(),
                "ftp://".to_string(),
                "mailto:".to_string(),
                "tel:".to_string(),
                "sms:".to_string(),
                
                // System commands
                ";".to_string(),
                "&&".to_string(),
                "||".to_string(),
                "|".to_string(),
                "`".to_string(),
                "$(".to_string(),
                "eval".to_string(),
                "exec".to_string(),
                "system".to_string(),
                "powerShell".to_string(),
                "cmd.exe".to_string(),
                "bash".to_string(),
                "sh".to_string(),
                "zsh".to_string(),
                "fish".to_string(),
                
                // Windows specific
                "powershell://".to_string(),
                "ms-settings:".to_string(),
                "windows:".to_string(),
                "microsoft-edge:".to_string(),
                
                // Browser configuration keywords (attacks you observed)
                "default-search".to_string(),
                "search-engine".to_string(),
                "search-settings".to_string(),
                "browser-default".to_string(),
            ],
            allowed_commands: vec![
                "help".to_string(),
                "status".to_string(),
                "list".to_string(),
                "info".to_string(),
                "version".to_string(),
                "exit".to_string(),
                "quit".to_string(),
            ],
            allowed_paths: vec![
                "/tmp/terminal-jarvis".to_string(),
                "./config".to_string(),
                "./logs".to_string(),
            ],
            strict_mode: true,
        }
    }
}

pub struct SecurityValidator {
    config: SecurityConfig,
    blocked_patterns: Vec<Regex>,
    command_allowlist: HashSet<String>,
}

impl SecurityValidator {
    pub fn new() -> Self {
        Self::with_config(SecurityConfig::default())
    }

    pub fn with_config(config: SecurityConfig) -> Self {
        let blocked_patterns: Vec<Regex> = config.blocked_keywords
            .iter()
            .filter_map(|keyword| Regex::new(&format!(r"(?i){}", regex::escape(keyword))).ok())
            .collect();

        let command_allowlist: HashSet<String> = config.allowed_commands
            .iter()
            .cloned()
            .collect();

        Self {
            config,
            blocked_patterns,
            command_allowlist,
        }
    }

    /// Validate any external input with comprehensive checks
    pub fn validate_input(&self, input: &str, context: &str) -> Result<bool, SecurityError> {
        // Length check
        if input.len() > self.config.max_input_length {
            return Err(SecurityError {
                message: format!("Input length {} exceeds maximum {}", input.len(), self.config.max_input_length),
                code: SecurityErrorCode::LengthExceeded,
            });
        }

        // Empty input check
        if input.trim().is_empty() {
            return Err(SecurityError {
                message: "Empty input not allowed".to_string(),
                code: SecurityErrorCode::InvalidInput,
            });
        }

        // Blocked patterns check
        for pattern in &self.blocked_patterns {
            if pattern.is_match(input) {
                return Err(SecurityError {
                    message: format!("Input contains blocked pattern: {}", pattern.as_str()),
                    code: SecurityErrorCode::BlockedKeyword,
                });
            }
        }

        // Context-specific validation
        match context {
            "command" => self.validate_command_input(input),
            "path" => self.validate_path_input(input),
            "model_name" => self.validate_model_name_input(input),
            "url" => self.validate_url_input(input),
            _ => self.validate_generic_input(input),
        }
    }

    /// Validate command execution
    pub fn validate_command(&self, command: &str, args: &[String]) -> Result<bool, SecurityError> {
        // Check if command is in allowlist
        if !self.command_allowlist.contains(command) {
            return Err(SecurityError {
                message: format!("Command '{}' not in allowlist", command),
                code: SecurityErrorCode::UnallowedCommand,
            });
        }

        // Validate each argument
        for arg in args {
            self.validate_input(arg, "command")?;
        }

        // In strict mode, no arguments allowed
        if self.config.strict_mode && !args.is_empty() {
            return Err(SecurityError {
                message: "Arguments not allowed in strict mode".to_string(),
                code: SecurityErrorCode::InvalidInput,
            });
        }

        Ok(true)
    }

    /// Validate file paths to prevent traversal attacks
    pub fn validate_file_path(&self, path: &str) -> Result<bool, SecurityError> {
        // Prevent path traversal
        if path.contains("..") || path.contains("~") {
            return Err(SecurityError {
                message: "Path traversal detected".to_string(),
                code: SecurityErrorCode::PathTraversal,
            });
        }

        // Check if path starts with allowed base paths
        let is_allowed = self.config.allowed_paths
            .iter()
            .any(|allowed_path| path.starts_with(allowed_path));

        if !is_allowed {
            return Err(SecurityError {
                message: format!("Path '{}' not in allowed paths", path),
                code: SecurityErrorCode::PathTraversal,
            });
        }

        Ok(true)
    }

    fn validate_command_input(&self, input: &str) -> Result<bool, SecurityError> {
        // Additional command-specific checks
        let dangerous_patterns = vec![
            // Traditional command injection
            r"(?i)\b(echo|printf)\s*\$",
            r"(?i)\$\(.*\)",
            r"`.*`",
            r"(?i)\b(eval|exec|system)\s*\(",
            
            // Browser and URL hijacking patterns - CRITICAL for all users
            r"(?i)[a-z_-]*://",                    // ANY protocol (block all, only allow http/https in specific context)
            r"(?i)\b(open|start|xdg-open)\s+",     // Cross-platform command to open URLs
            r"(?i)\b(browser|chrome|firefox|edge|safari|opera|brave|vivaldi|tor)\s+", // Browser commands
            r"(?i)\b(window\.open|location\.href|document\.location)", // JavaScript browser APIs
            r"(?i)\b(navigate|redirect|goto|visit)\s+",  // Navigation keywords
            
            // System-level browser automation
            r"(?i)\b(selenium|webdriver|playwright|puppeteer)\s+", // Browser automation tools
            r"(?i)\b(automation|script|macro)\s+",         // General automation
            
            // Cross-platform executable detection
            r"(?i)\.(exe|bat|cmd|ps1|sh|zsh|fish|bash)$", // Executable files
            r"(?i)\b(run|execute|launch|start)\s+",        // Execution verbs
        ];

        for pattern in dangerous_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(input) {
                    return Err(SecurityError {
                        message: format!("Browser/System injection pattern blocked: {}", pattern),
                        code: SecurityErrorCode::InjectionAttempt,
                    });
                }
            }
        }

        Ok(true)
    }

    fn validate_path_input(&self, input: &str) -> Result<bool, SecurityError> {
        self.validate_file_path(input)
    }

    fn validate_model_name_input(&self, input: &str) -> Result<bool, SecurityError> {
        // Only allow alphanumeric, hyphens, underscores, and dots in model names
        let model_pattern = Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap();
        
        if !model_pattern.is_match(input) {
            return Err(SecurityError {
                message: "Model name contains invalid characters".to_string(),
                code: SecurityErrorCode::FormatInvalid,
            });
        }

        Ok(true)
    }

    fn validate_url_input(&self, input: &str) -> Result<bool, SecurityError> {
        // Basic URL validation with security checks
        if !input.starts_with("https://") && !input.starts_with("http://") {
            return Err(SecurityError {
                message: "Only HTTP/HTTPS URLs allowed (all other protocols blocked)".to_string(),
                code: SecurityErrorCode::FormatInvalid,
            });
        }

        // Block ALL internal/special URLs - CRITICAL for ecosystem security
        let blocked_domains = vec![
            "localhost", "127.0.0.1", "0.0.0.0", "::1",       // Local addresses
            "192.168.", "10.", "172.16.", "169.254.",        // Private networks
            "chrome.google.com",                         // Chrome extension store
            "addons.mozilla.org",                        // Firefox extension store
            "microsoftedge.microsoft.com",               // Edge extension store
            "chrome://", "edge://", "firefox://", "about:", // Internal protocols
        ];

        for blocked in &blocked_domains {
            if input.contains(blocked) {
                return Err(SecurityError {
                    message: format!("Blocked URL contains dangerous domain/protocol: {}", blocked),
                    code: SecurityErrorCode::InvalidInput,
                });
            }
        }

        // Block browser extension/hijacking domains
        let suspicious_patterns = vec![
            r"(?i)(extension|addon|theme|plugin).*\.(com|org|net)",
            r"(?i)(malware|phishing|exploit|inject).*\.(com|org|net)",
            r"(?i)(bit\.ly|tinyurl|t\.co|short\.io)",            // URL shorteners (often malicious)
            r"(?i)(pastebin|gist).*\.(com|io)",                  // Code sharing (potential malware delivery)
            r"(?i)\.tk$|\.ml$|\.ga$",                           // Suspicious TLDs
        ];

        for pattern in suspicious_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(input) {
                    return Err(SecurityError {
                        message: format!("Suspicious URL pattern blocked: {}", pattern),
                        code: SecurityErrorCode::InvalidInput,
                    });
                }
            }
        }

        Ok(true)
    }

    fn validate_generic_input(&self, _input: &str) -> Result<bool, SecurityError> {
        // Generic validation for other contexts
        Ok(true)
    }
}

impl std::fmt::Display for SecurityErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityErrorCode::InvalidInput => write!(f, "InvalidInput"),
            SecurityErrorCode::BlockedKeyword => write!(f, "BlockedKeyword"),
            SecurityErrorCode::UnallowedCommand => write!(f, "UnallowedCommand"),
            SecurityErrorCode::PathTraversal => write!(f, "PathTraversal"),
            SecurityErrorCode::InjectionAttempt => write!(f, "InjectionAttempt"),
            SecurityErrorCode::LengthExceeded => write!(f, "LengthExceeded"),
            SecurityErrorCode::FormatInvalid => write!(f, "FormatInvalid"),
        }
    }
}

impl Default for SecurityValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod browser_protection_tests {
    use super::*;

    #[test]
    fn test_chrome_attacks_blocked() {
        let validator = SecurityValidator::new();
        
        // The exact attacks you observed
        let attacks = vec![
            "chrome://settings/searchEngines",
            "chrome://extensions",
            "edge://settings", 
            "firefox://about:config",
            "default-search",
        ];

        for attack in &attacks {
            println!("Testing attack: {}", attack);
            let result = validator.validate_input(attack, "command");
            assert!(result.is_err(), "Attack should be blocked: {}", attack);
            println!("✅ BLOCKED: {}", attack);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocked_keywords() {
        let validator = SecurityValidator::new();
        
        // Should block dangerous commands
        assert!(validator.validate_input("rm -rf /", "command").is_err());
        assert!(validator.validate_input("sudo ls", "command").is_err());
        assert!(validator.validate_input("curl http://evil.com", "command").is_err());
    }

    #[test]
    fn test_allowed_commands() {
        let validator = SecurityValidator::new();
        
        // Should allow safe commands
        assert!(validator.validate_command("help", &[]).is_ok());
        assert!(validator.validate_command("status", &[]).is_ok());
        
        // Should block unallowed commands
        assert!(validator.validate_command("rm", &[]).is_err());
    }

    #[test]
    fn test_path_validation() {
        let validator = SecurityValidator::new();
        
        // Should block path traversal
        assert!(validator.validate_input("../../../etc/passwd", "path").is_err());
        assert!(validator.validate_input("~/.ssh/id_rsa", "path").is_err());
    }

    #[test]
    fn test_model_name_validation() {
        let validator = SecurityValidator::new();
        
        // Should allow valid model names
        assert!(validator.validate_input("model-v1.0", "model_name").is_ok());
        assert!(validator.validate_input("whisper_tiny", "model_name").is_ok());
        
        // Should block invalid model names
        assert!(validator.validate_input("model;rm -rf /", "model_name").is_err());
        assert!(validator.validate_input("../../../malicious", "model_name").is_err());
    }
}
