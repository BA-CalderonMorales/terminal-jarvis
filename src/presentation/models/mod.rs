//! Core data models for Terminal Jarvis
//!
//! This module contains the fundamental data structures used throughout
//! the application, following an MVVM-inspired architecture.

pub mod tool;

pub use tool::*;

/// Application state management
#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub current_view: ViewType,
    pub tools: Vec<Tool>,
    pub selected_tool_index: Option<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: ViewType::MainMenu,
            tools: Vec::new(),
            selected_tool_index: None,
        }
    }

    pub fn set_view(&mut self, view: ViewType) {
        self.current_view = view;
    }

    pub fn set_tools(&mut self, tools: Vec<Tool>) {
        self.tools = tools;
    }

    pub fn select_tool(&mut self, index: usize) {
        if index < self.tools.len() {
            self.selected_tool_index = Some(index);
        }
    }

    pub fn selected_tool(&self) -> Option<&Tool> {
        self.selected_tool_index
            .and_then(|index| self.tools.get(index))
    }
}

/// View types for navigation
#[derive(Debug, Clone, PartialEq)]
pub enum ViewType {
    MainMenu,
    ToolList,
    ToolDetails,
    Settings,
}