// CLI Logic Domain Modules
// Each module handles a specific area of CLI business logic

pub mod cli_logic_auth_operations;
pub mod cli_logic_benchmark_operations;
pub mod cli_logic_config_management;
pub mod cli_logic_entry_point;
pub mod cli_logic_evals_operations;
pub mod cli_logic_first_run;
pub mod cli_logic_info_operations;
pub mod cli_logic_interactive;
pub mod cli_logic_list_operations;
pub mod cli_logic_responsive_display;
#[cfg(test)]
mod cli_logic_responsive_display_tests;
pub mod cli_logic_responsive_menu;
pub mod cli_logic_template_operations;
pub mod cli_logic_tool_execution;
pub mod cli_logic_update_operations;
pub mod cli_logic_utilities;
pub mod cli_logic_welcome;
// (auth operations already declared above)

// Re-export main public functions for backward compatibility
pub use cli_logic_auth_operations::*;
pub use cli_logic_benchmark_operations::*;
pub use cli_logic_entry_point::*;
pub use cli_logic_first_run::*;
