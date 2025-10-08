// Voice Command Recognition and Parsing
// Maps transcribed voice input to application commands

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Recognized voice command with intent and parameters
#[derive(Debug, Clone, PartialEq)]
pub enum VoiceCommand {
    /// Navigate to AI tools menu
    OpenAITools,

    /// Navigate to authentication menu
    OpenAuthentication,

    /// Navigate to settings menu
    OpenSettings,

    /// Navigate to evals menu
    OpenEvals,

    /// Navigate to important links
    OpenLinks,

    /// Exit the application
    Exit,

    /// Install a specific tool
    InstallTool { tool_name: String },

    /// Update a specific tool
    UpdateTool { tool_name: String },

    /// Check status of a tool
    CheckStatus { tool_name: String },

    /// List all available tools
    ListTools,

    /// Show help information
    ShowHelp,

    /// Run a benchmark
    RunBenchmark { benchmark_name: Option<String> },

    /// Unrecognized command - contains the raw text
    Unknown { raw_text: String },
}

/// Voice command pattern definition for configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommandPattern {
    /// The command variant this pattern matches
    pub command: String,

    /// List of phrases that trigger this command
    pub patterns: Vec<String>,

    /// Whether partial matches are allowed
    pub allow_partial: bool,

    /// Minimum confidence required (0.0 to 1.0)
    pub min_confidence: f32,
}

/// Parser for converting transcribed text to commands
pub struct VoiceCommandParser {
    /// Command patterns loaded from configuration
    patterns: Vec<VoiceCommandPattern>,

    /// Whether to use fuzzy matching
    fuzzy_matching: bool,
}

impl Default for VoiceCommandParser {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceCommandParser {
    /// Create a new parser with default command patterns
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
            fuzzy_matching: true,
        }
    }

    /// Create parser with custom patterns from configuration
    #[allow(dead_code)]
    pub fn with_patterns(patterns: Vec<VoiceCommandPattern>) -> Self {
        Self {
            patterns,
            fuzzy_matching: true,
        }
    }

    /// Default command patterns for common actions
    fn default_patterns() -> Vec<VoiceCommandPattern> {
        vec![
            VoiceCommandPattern {
                command: "OpenAITools".to_string(),
                patterns: vec![
                    "open ai tools".to_string(),
                    "show ai tools".to_string(),
                    "ai tools menu".to_string(),
                    "tools".to_string(),
                ],
                allow_partial: true,
                min_confidence: 0.7,
            },
            VoiceCommandPattern {
                command: "OpenAuthentication".to_string(),
                patterns: vec![
                    "open authentication".to_string(),
                    "show auth".to_string(),
                    "authentication".to_string(),
                    "login".to_string(),
                ],
                allow_partial: true,
                min_confidence: 0.7,
            },
            VoiceCommandPattern {
                command: "OpenSettings".to_string(),
                patterns: vec![
                    "open settings".to_string(),
                    "show settings".to_string(),
                    "settings".to_string(),
                    "preferences".to_string(),
                ],
                allow_partial: true,
                min_confidence: 0.7,
            },
            VoiceCommandPattern {
                command: "Exit".to_string(),
                patterns: vec![
                    "exit".to_string(),
                    "quit".to_string(),
                    "close".to_string(),
                    "goodbye".to_string(),
                ],
                allow_partial: false,
                min_confidence: 0.8,
            },
            VoiceCommandPattern {
                command: "ListTools".to_string(),
                patterns: vec![
                    "list tools".to_string(),
                    "show all tools".to_string(),
                    "what tools are available".to_string(),
                ],
                allow_partial: true,
                min_confidence: 0.7,
            },
            VoiceCommandPattern {
                command: "ShowHelp".to_string(),
                patterns: vec![
                    "help".to_string(),
                    "show help".to_string(),
                    "what can i do".to_string(),
                    "commands".to_string(),
                ],
                allow_partial: true,
                min_confidence: 0.7,
            },
        ]
    }

    /// Parse transcribed text into a command
    ///
    /// This method:
    /// 1. Normalizes the input text
    /// 2. Matches against known patterns
    /// 3. Extracts parameters for parameterized commands
    /// 4. Returns the best matching command or Unknown
    pub fn parse(&self, text: &str, confidence: f32) -> Result<VoiceCommand> {
        let normalized = self.normalize_text(text);

        // Try to match against known patterns
        for pattern in &self.patterns {
            if confidence < pattern.min_confidence {
                continue;
            }

            for phrase in &pattern.patterns {
                if self.matches_pattern(&normalized, phrase, pattern.allow_partial) {
                    return self.create_command(&pattern.command, &normalized, text);
                }
            }
        }

        // Check for parameterized commands
        if let Some(cmd) = self.try_parse_install_command(&normalized) {
            return Ok(cmd);
        }

        if let Some(cmd) = self.try_parse_update_command(&normalized) {
            return Ok(cmd);
        }

        if let Some(cmd) = self.try_parse_status_command(&normalized) {
            return Ok(cmd);
        }

        // No match found - return unknown
        Ok(VoiceCommand::Unknown {
            raw_text: text.to_string(),
        })
    }

    /// Normalize text for matching (lowercase, trim, etc.)
    fn normalize_text(&self, text: &str) -> String {
        text.to_lowercase()
            .trim()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect()
    }

    /// Check if text matches a pattern
    fn matches_pattern(&self, text: &str, pattern: &str, allow_partial: bool) -> bool {
        if allow_partial {
            if self.fuzzy_matching {
                // Allow for small variations (simple fuzzy matching)
                text.contains(pattern) || pattern.contains(text)
            } else {
                text.contains(pattern)
            }
        } else {
            text == pattern
        }
    }

    /// Create a command from a pattern match
    fn create_command(&self, command_name: &str, _normalized: &str, _raw: &str) -> Result<VoiceCommand> {
        match command_name {
            "OpenAITools" => Ok(VoiceCommand::OpenAITools),
            "OpenAuthentication" => Ok(VoiceCommand::OpenAuthentication),
            "OpenSettings" => Ok(VoiceCommand::OpenSettings),
            "Exit" => Ok(VoiceCommand::Exit),
            "ListTools" => Ok(VoiceCommand::ListTools),
            "ShowHelp" => Ok(VoiceCommand::ShowHelp),
            _ => Err(anyhow!("Unknown command: {}", command_name)),
        }
    }

    /// Try to parse "install <tool>" command
    fn try_parse_install_command(&self, text: &str) -> Option<VoiceCommand> {
        let install_patterns = ["install ", "add "];

        for pattern in &install_patterns {
            if let Some(tool_name) = text.strip_prefix(pattern) {
                let tool_name = tool_name.trim().to_string();
                if !tool_name.is_empty() {
                    return Some(VoiceCommand::InstallTool { tool_name });
                }
            }
        }

        None
    }

    /// Try to parse "update <tool>" command
    fn try_parse_update_command(&self, text: &str) -> Option<VoiceCommand> {
        let update_patterns = ["update ", "upgrade "];

        for pattern in &update_patterns {
            if let Some(tool_name) = text.strip_prefix(pattern) {
                let tool_name = tool_name.trim().to_string();
                if !tool_name.is_empty() {
                    return Some(VoiceCommand::UpdateTool { tool_name });
                }
            }
        }

        None
    }

    /// Try to parse "status <tool>" or "check <tool>" command
    fn try_parse_status_command(&self, text: &str) -> Option<VoiceCommand> {
        let status_patterns = ["status ", "check ", "is ", "show status of "];

        for pattern in &status_patterns {
            if let Some(tool_name) = text.strip_prefix(pattern) {
                let tool_name = tool_name.trim().to_string();
                if !tool_name.is_empty() {
                    return Some(VoiceCommand::CheckStatus { tool_name });
                }
            }
        }

        None
    }

    /// Get all available command patterns for display/help
    #[allow(dead_code)]
    pub fn available_commands(&self) -> Vec<String> {
        self.patterns
            .iter()
            .flat_map(|p| p.patterns.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_open_tools_command() {
        let parser = VoiceCommandParser::new();
        let result = parser.parse("open ai tools", 0.9).unwrap();
        assert_eq!(result, VoiceCommand::OpenAITools);
    }

    #[test]
    fn test_parse_exit_command() {
        let parser = VoiceCommandParser::new();
        let result = parser.parse("exit", 0.9).unwrap();
        assert_eq!(result, VoiceCommand::Exit);
    }

    #[test]
    fn test_parse_install_command() {
        let parser = VoiceCommandParser::new();
        let result = parser.parse("install aider", 0.9).unwrap();
        assert_eq!(
            result,
            VoiceCommand::InstallTool {
                tool_name: "aider".to_string()
            }
        );
    }

    #[test]
    fn test_parse_unknown_command() {
        let parser = VoiceCommandParser::new();
        let result = parser.parse("do something random", 0.9).unwrap();
        assert!(matches!(result, VoiceCommand::Unknown { .. }));
    }

    #[test]
    fn test_low_confidence_rejection() {
        let parser = VoiceCommandParser::new();
        let result = parser.parse("exit", 0.5).unwrap();
        // Should return Unknown due to low confidence
        assert!(matches!(result, VoiceCommand::Unknown { .. }));
    }

    #[test]
    fn test_normalize_text() {
        let parser = VoiceCommandParser::new();
        let normalized = parser.normalize_text("  OPEN AI Tools!  ");
        assert_eq!(normalized, "open ai tools");
    }
}
