//! API Layer - HTTP client and service abstractions
//!
//! This module provides a complete API framework for Terminal Jarvis, including:
//! - Base configuration and routing
//! - HTTP client abstraction with retry logic
//! - Tool-specific API operations
//!
//! The API layer is designed for future extensibility when Terminal Jarvis
//! integrates with remote services for tool discovery, updates, and metadata.

#![allow(unused_imports)]

pub mod api_base;
pub mod api_client;
pub mod api_tool_operations;

// Re-export main types for backward compatibility
pub use api_base::{routes, ApiBase};
pub use api_client::ApiClient;
pub use api_tool_operations::{ToolApi, ToolMetadata};
