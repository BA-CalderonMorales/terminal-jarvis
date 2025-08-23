//! Terminal Jarvis Remote API Framework
//!
//! **STATUS: FUTURE-USE MODULE** - Represents "positive technical debt"
//!
//! This module implements a comprehensive API framework designed for future remote service
//! integration. Currently marked with #[allow(dead_code)] as it represents a forward
//! investment in the Terminal Jarvis architecture.
//!
//! # PLANNED IMPLEMENTATION ROADMAP
//!
//! ## Phase 1 (v1.0.0): Local API Server
//! - REST API for local Terminal Jarvis management
//! - Tool status monitoring and control endpoints
//! - Configuration management via HTTP
//! - Authentication state tracking and management
//! - WebSocket support for real-time updates
//!
//! ## Phase 2 (v1.1.0): Remote Service Integration  
//! - Connection to Terminal Jarvis cloud services
//! - Centralized tool management across devices
//! - Shared configuration synchronization
//! - Remote tool execution capabilities
//! - Cross-device session management
//!
//! ## Phase 3 (v1.2.0): Advanced Platform Features
//! - Multi-user environments and permissions
//! - Enterprise deployment support
//! - Custom plugin marketplace integration
//! - Advanced analytics and monitoring dashboard
//! - Workflow automation and scripting API
//!
//! # CURRENT IMPLEMENTATION
//!
//! The framework provides three core components:
//!
//! - **[`ApiBase`](api_base::ApiBase)**: Base configuration with timeout and retry logic
//! - **[`ApiClient`](api_client::ApiClient)**: HTTP client abstraction with error handling
//! - **[`ToolApi`](api_tool_operations::ToolApi)**: Tool-specific operations framework
//!
//! # ARCHITECTURE JUSTIFICATION
//!
//! This forward-looking design ensures Terminal Jarvis can evolve into a comprehensive
//! platform without breaking changes. The modular structure allows incremental
//! implementation while maintaining backward compatibility.
//!
//! The API framework follows these principles:
//! - **Extensibility**: Easy to add new endpoints and operations
//! - **Reliability**: Built-in retry logic and error handling
//! - **Performance**: Async-first design with connection pooling
//! - **Security**: Authentication and authorization ready
//!
//! # DEPENDENCIES
//!
//! - `reqwest`: HTTP client with async support
//! - `serde_json`: JSON serialization for API payloads
//! - `anyhow`: Unified error handling
//!
//! # MAINTENANCE NOTES
//!
//! - Keep #[allow(dead_code)] annotations until Phase 1 implementation begins
//! - Update this documentation as implementation progresses
//! - Preserve API contracts to ensure future compatibility
//! - Consider API versioning strategy before Phase 1 implementation

#![allow(unused_imports)]
#![allow(dead_code)]

pub mod api_base;
pub mod api_client;
pub mod api_tool_operations;

// Re-export main types for backward compatibility
pub use api_base::{routes, ApiBase};
pub use api_client::ApiClient;
pub use api_tool_operations::{ToolApi, ToolMetadata};
