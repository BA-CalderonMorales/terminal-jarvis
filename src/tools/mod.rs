// Tools Domain Modules
// Each module handles a specific area of tool management

pub mod tools_command_mapping;
pub mod tools_detection;
pub mod tools_entry_point;
pub mod tools_execution_engine;
pub mod tools_process_management;
pub mod tools_startup_guidance;

// Re-export main public functions for backward compatibility
pub use tools_entry_point::*;
