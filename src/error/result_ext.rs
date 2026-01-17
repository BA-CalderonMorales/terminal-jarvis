// ResultExt - Extension traits for Result types
//
// Provides ergonomic methods for error handling that avoid panics.

#![allow(dead_code)] // These extension traits are part of the public API for future use

use super::{ErrorContext, JarvisError, JarvisErrorKind};

/// Extension trait for Option types to convert to JarvisError
pub trait ResultExt<T> {
    /// Convert None to a JarvisError with context
    fn ok_or_jarvis(
        self,
        kind: JarvisErrorKind,
        message: impl Into<String>,
    ) -> Result<T, JarvisError>;

    /// Convert None to a "not found" error for a resource
    fn ok_or_not_found(self, resource_type: &str, resource_name: &str) -> Result<T, JarvisError>;

    /// Convert None to a tool not found error
    fn ok_or_tool_not_found(self, tool_name: &str) -> Result<T, JarvisError>;
}

impl<T> ResultExt<T> for Option<T> {
    fn ok_or_jarvis(
        self,
        kind: JarvisErrorKind,
        message: impl Into<String>,
    ) -> Result<T, JarvisError> {
        self.ok_or_else(|| JarvisError::new(kind, message))
    }

    fn ok_or_not_found(self, resource_type: &str, resource_name: &str) -> Result<T, JarvisError> {
        self.ok_or_else(|| {
            JarvisError::new(
                JarvisErrorKind::Internal,
                format!("{resource_type} '{resource_name}' not found"),
            )
            .for_resource(resource_name)
        })
    }

    fn ok_or_tool_not_found(self, tool_name: &str) -> Result<T, JarvisError> {
        self.ok_or_else(|| JarvisError::tool_not_found(tool_name))
    }
}

/// Extension trait for Result types to add context
pub trait ResultContextExt<T, E> {
    /// Add operation context to an error
    fn during(self, operation: impl Into<String>) -> Result<T, JarvisError>
    where
        E: std::error::Error + Send + Sync + 'static;

    /// Add resource context to an error
    fn for_resource(self, resource: impl Into<String>) -> Result<T, JarvisError>
    where
        E: std::error::Error + Send + Sync + 'static;

    /// Convert to JarvisError with full context
    fn with_jarvis_context(
        self,
        kind: JarvisErrorKind,
        context: ErrorContext,
    ) -> Result<T, JarvisError>
    where
        E: std::error::Error + Send + Sync + 'static;
}

impl<T, E> ResultContextExt<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn during(self, operation: impl Into<String>) -> Result<T, JarvisError> {
        self.map_err(|e| {
            JarvisError::new(JarvisErrorKind::Internal, e.to_string())
                .during(operation)
                .caused_by(e)
        })
    }

    fn for_resource(self, resource: impl Into<String>) -> Result<T, JarvisError> {
        self.map_err(|e| {
            JarvisError::new(JarvisErrorKind::Internal, e.to_string())
                .for_resource(resource)
                .caused_by(e)
        })
    }

    fn with_jarvis_context(
        self,
        kind: JarvisErrorKind,
        context: ErrorContext,
    ) -> Result<T, JarvisError> {
        self.map_err(|e| {
            JarvisError::new(kind, e.to_string())
                .with_context(context)
                .caused_by(e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_ok_or_tool_not_found() {
        let result: Result<&str, JarvisError> = None.ok_or_tool_not_found("claude");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, JarvisErrorKind::Tool);
        assert!(err.message.contains("claude"));
    }

    #[test]
    fn test_option_ok_or_not_found() {
        let result: Result<i32, JarvisError> = None.ok_or_not_found("Config", "api.toml");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Config"));
        assert!(err.context.resource == Some("api.toml".to_string()));
    }

    #[test]
    fn test_result_during() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let result: Result<(), std::io::Error> = Err(io_error);
        let jarvis_result = result.during("reading config file");

        assert!(jarvis_result.is_err());
        let err = jarvis_result.unwrap_err();
        assert!(err.context.operation == Some("reading config file".to_string()));
    }
}
