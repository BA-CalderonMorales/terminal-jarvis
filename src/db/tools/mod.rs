// Tools Domain Module
//
// Tool configuration storage and retrieval.
// Manages tool metadata, installation info, and authentication.

mod entities;
mod repository;

pub use entities::{Tool, ToolAuth, ToolInstall};
pub use repository::ToolsRepository;
