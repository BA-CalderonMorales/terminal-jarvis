//! Tool List View Model
//!
//! Handles presentation logic for displaying lists of AI coding tools.
//! Provides methods for tool selection, filtering, and display formatting.

use crate::presentation::models::Tool;

/// View model for managing tool lists and selection
#[derive(Debug, Clone)]
pub struct ToolListViewModel {
    tools: Vec<Tool>,
    selected_index: Option<usize>,
}

impl ToolListViewModel {
    pub fn new(tools: Vec<Tool>) -> Self {
        Self {
            tools,
            selected_index: None,
        }
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    pub fn selected_tool(&self) -> Option<&Tool> {
        self.selected_index
            .and_then(|index| self.tools.get(index))
    }

    pub fn select_tool(&mut self, index: usize) {
        if index < self.tools.len() {
            self.selected_index = Some(index);
        }
    }

    pub fn display_list(&self) {
        if self.tools.is_empty() {
            println!("No tools available.");
            return;
        }

        println!("Available AI Coding Tools:\n");

        for (index, tool) in self.tools.iter().enumerate() {
            let status = if tool.is_installed {
                "[INSTALLED]"
            } else {
                "[AVAILABLE]"
            };

            let selected_marker = if Some(index) == self.selected_index {
                "► "
            } else {
                "  "
            };

            println!(
                "{}{}. {} {} - {}",
                selected_marker,
                index + 1,
                tool.display_name,
                status,
                tool.description
            );
        }

        println!("\nUse 'terminal-jarvis info <tool>' for detailed information.");
        println!("Use 'terminal-jarvis install <tool>' to install a tool.");
    }
}