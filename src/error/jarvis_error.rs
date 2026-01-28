// JarvisError - The main error type for Terminal Jarvis
//
// Provides structured errors with:
// - Error kind classification
// - Contextual information
// - User-friendly display
// - Backtrace support (in debug builds)

use std::fmt;

/// Classification of error types for appropriate handling
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JarvisErrorKind {
    /// Tool-related errors (not found, not installed, execution failed)
    Tool,
    /// Installation errors (download failed, permissions, dependencies)
    Installation,
    /// Configuration errors (invalid config, missing fields, parse errors)
    Config,
    /// Authentication errors (missing credentials, expired tokens)
    Auth,
    /// Network errors (connection failed, timeout, API errors)
    Network,
    /// Database errors (connection, query, migration)
    Database,
    /// I/O errors (file not found, permissions, disk full)
    Io,
    /// User input errors (cancelled, invalid selection)
    UserInput,
    /// Internal errors (unexpected state, logic errors)
    Internal,
}

impl fmt::Display for JarvisErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JarvisErrorKind::Tool => write!(f, "Tool Error"),
            JarvisErrorKind::Installation => write!(f, "Installation Error"),
            JarvisErrorKind::Config => write!(f, "Configuration Error"),
            JarvisErrorKind::Auth => write!(f, "Authentication Error"),
            JarvisErrorKind::Network => write!(f, "Network Error"),
            JarvisErrorKind::Database => write!(f, "Database Error"),
            JarvisErrorKind::Io => write!(f, "I/O Error"),
            JarvisErrorKind::UserInput => write!(f, "User Input"),
            JarvisErrorKind::Internal => write!(f, "Internal Error"),
        }
    }
}

/// Contextual information for errors
#[derive(Debug, Clone, Default)]
pub struct ErrorContext {
    /// The operation being performed when the error occurred
    pub operation: Option<String>,
    /// The resource involved (tool name, file path, etc.)
    pub resource: Option<String>,
    /// Suggested recovery action
    pub suggestion: Option<String>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn operation(mut self, op: impl Into<String>) -> Self {
        self.operation = Some(op.into());
        self
    }

    pub fn resource(mut self, res: impl Into<String>) -> Self {
        self.resource = Some(res.into());
        self
    }

    pub fn suggestion(mut self, sug: impl Into<String>) -> Self {
        self.suggestion = Some(sug.into());
        self
    }
}

/// The main error type for Terminal Jarvis
#[derive(Debug)]
pub struct JarvisError {
    /// Classification of the error
    pub kind: JarvisErrorKind,
    /// Human-readable error message
    pub message: String,
    /// Additional context
    pub context: ErrorContext,
    /// The underlying cause, if any
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl JarvisError {
    /// Create a new error with the given kind and message
    pub fn new(kind: JarvisErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            context: ErrorContext::default(),
            source: None,
        }
    }

    /// Add context to the error
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Add an operation context
    pub fn during(mut self, operation: impl Into<String>) -> Self {
        self.context.operation = Some(operation.into());
        self
    }

    /// Add a resource context
    pub fn for_resource(mut self, resource: impl Into<String>) -> Self {
        self.context.resource = Some(resource.into());
        self
    }

    /// Add a suggestion for recovery
    pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
        self.context.suggestion = Some(suggestion.into());
        self
    }

    /// Add a source error
    pub fn caused_by<E: std::error::Error + Send + Sync + 'static>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Check if this is a user cancellation (not a real error)
    pub fn is_user_cancelled(&self) -> bool {
        self.kind == JarvisErrorKind::UserInput
    }

    // Convenience constructors for common error types

    /// Tool not found error
    pub fn tool_not_found(tool_name: &str) -> Self {
        Self::new(
            JarvisErrorKind::Tool,
            format!("Tool '{tool_name}' not found"),
        )
        .for_resource(tool_name)
        .suggest("Run 'terminal-jarvis list' to see available tools")
    }

    /// Tool not installed error
    pub fn tool_not_installed(tool_name: &str) -> Self {
        Self::new(
            JarvisErrorKind::Tool,
            format!("Tool '{tool_name}' is not installed"),
        )
        .for_resource(tool_name)
        .suggest(format!(
            "Run 'terminal-jarvis run {tool_name}' to install it"
        ))
    }

    /// Installation failed error
    pub fn installation_failed(tool_name: &str, reason: &str) -> Self {
        Self::new(
            JarvisErrorKind::Installation,
            format!("Failed to install '{tool_name}': {reason}"),
        )
        .for_resource(tool_name)
        .during("tool installation")
    }

    /// Config not found error
    pub fn config_not_found(path: &str) -> Self {
        Self::new(
            JarvisErrorKind::Config,
            format!("Configuration file not found: {path}"),
        )
        .for_resource(path)
    }

    /// User cancelled operation
    pub fn user_cancelled() -> Self {
        Self::new(JarvisErrorKind::UserInput, "Operation cancelled by user")
    }

    /// Internal error (unexpected state)
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(JarvisErrorKind::Internal, message)
            .suggest("This is a bug - please report it at https://github.com/BA-CalderonMorales/terminal-jarvis/issues")
    }
}

impl fmt::Display for JarvisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: [Kind] Message
        write!(f, "[{}] {}", self.kind, self.message)?;

        // Add context if available
        if let Some(ref op) = self.context.operation {
            write!(f, " (during: {op})")?;
        }
        if let Some(ref res) = self.context.resource {
            write!(f, " (resource: {res})")?;
        }

        // Add suggestion on a new line if available
        if let Some(ref sug) = self.context.suggestion {
            write!(f, "\n  Suggestion: {sug}")?;
        }

        Ok(())
    }
}

impl std::error::Error for JarvisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// Conversion from common error types

impl From<std::io::Error> for JarvisError {
    fn from(err: std::io::Error) -> Self {
        Self::new(JarvisErrorKind::Io, err.to_string()).caused_by(err)
    }
}

impl From<anyhow::Error> for JarvisError {
    fn from(err: anyhow::Error) -> Self {
        // Try to extract more specific error types
        Self::new(JarvisErrorKind::Internal, err.to_string())
    }
}

// Note: We don't implement From<JarvisError> for anyhow::Error
// because anyhow already has a blanket impl for any type implementing std::error::Error.
// JarvisError implements std::error::Error, so conversion happens automatically.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_not_found_error() {
        let err = JarvisError::tool_not_found("nonexistent");
        assert_eq!(err.kind, JarvisErrorKind::Tool);
        assert!(err.message.contains("nonexistent"));
        assert!(err.context.suggestion.is_some());
    }

    #[test]
    fn test_error_chaining() {
        let err = JarvisError::new(JarvisErrorKind::Config, "Failed to parse")
            .during("loading configuration")
            .for_resource("config.toml")
            .suggest("Check the config file syntax");

        let display = format!("{err}");
        assert!(display.contains("Failed to parse"));
        assert!(display.contains("loading configuration"));
        assert!(display.contains("config.toml"));
        assert!(display.contains("Check the config file syntax"));
    }

    #[test]
    fn test_user_cancelled() {
        let err = JarvisError::user_cancelled();
        assert!(err.is_user_cancelled());
    }
}
