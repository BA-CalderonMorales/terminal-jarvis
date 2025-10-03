// CLI Logic Domain Modules
// Each module handles a specific area of CLI business logic

pub mod cli_logic_auth_operations;
pub mod cli_logic_config_management;
pub mod cli_logic_entry_point;
pub mod cli_logic_info_operations;
pub mod cli_logic_interactive;
pub mod cli_logic_list_operations;
pub mod cli_logic_responsive_display;
pub mod cli_logic_responsive_menu;
pub mod cli_logic_template_operations;
pub mod cli_logic_tool_execution;
pub mod cli_logic_update_operations;
pub mod cli_logic_utilities;
// (auth operations already declared above)

// Re-export main public functions for backward compatibility
pub use cli_logic_auth_operations::*;
pub use cli_logic_entry_point::*;
