// Error Module - Centralized error handling for Terminal Jarvis
//
// "Friends don't let friends unwrap in prod."
//
// This module provides:
// - Custom error types for each domain
// - Proper error context and propagation
// - User-friendly error messages
// - Graceful degradation strategies

mod jarvis_error;
mod result_ext;

pub use jarvis_error::{ErrorContext, JarvisError, JarvisErrorKind};
pub use result_ext::ResultExt;

/// Type alias for Results using JarvisError
pub type JarvisResult<T> = Result<T, JarvisError>;
