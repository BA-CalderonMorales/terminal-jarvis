// Theme module - Domain-based architecture for theme management
//
// This module provides theme management functionality using a
// domain-based architecture pattern for maintainability and testability.

// Domain modules
mod theme_background_layout;
mod theme_config;
mod theme_definitions;
mod theme_entry_point;
pub mod theme_global_config;
pub mod theme_persistence;
mod theme_text_formatting;
mod theme_utilities; // Export for global theme management

// Re-export main interfaces
pub use theme_entry_point::{Theme, ThemeType};
