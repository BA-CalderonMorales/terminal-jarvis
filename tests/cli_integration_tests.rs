// CLI Integration Tests
// Testing CLI components and interactions

use terminal_jarvis::presentation::models::{AppState, Tool, ToolAuth, ToolConfig, ViewType};
use terminal_jarvis::presentation::view_models::tool_list_view_model::ToolListViewModel;
use terminal_jarvis::presentation::views::components::menu::Menu;
use terminal_jarvis::presentation::views::components::tool_card::ToolCard;

fn create_test_tool(name: &str, installed: bool) -> Tool {
    let config = ToolConfig {
        homepage: "https://example.com".to_string(),
        documentation: "https://docs.example.com".to_string(),
        features: None,
        auth: ToolAuth {
            env_vars: vec!["API_KEY".to_string()],
            setup_url: "https://setup.example.com".to_string(),
            browser_auth: false,
            auth_instructions: None,
        },
    };

    let mut tool = Tool::new(
        name.to_string(),
        format!("{name} Tool"),
        format!("A test tool for {name}"),
        format!("{name}-cli"),
        config,
        false,
        true, // is_available
    );

    if installed {
        tool.set_installed(true);
    }

    tool
}

#[test]
fn test_tool_card_component() {
    let tool = create_test_tool("test", true);
    let card = ToolCard::new(tool);

    let rendered = card.render();
    assert!(rendered.contains("test Tool"));
    assert!(rendered.contains("Installed"));
    assert!(rendered.contains("test-cli"));

    let compact = card.render_compact();
    assert!(compact.contains("test Tool"));
    assert!(compact.contains("✓"));
}

#[test]
fn test_menu_component() {
    let mut menu = Menu::new("Test Menu".to_string());
    menu.add_item(
        "Option 1".to_string(),
        "opt1".to_string(),
        Some("First choice".to_string()),
    );
    menu.add_item("Option 2".to_string(), "opt2".to_string(), None);

    assert_eq!(menu.len(), 2);

    let rendered = menu.render();
    assert!(rendered.contains("Test Menu"));
    assert!(rendered.contains("1. Option 1"));
    assert!(rendered.contains("First choice"));
}

#[test]
fn test_tool_list_view_model() {
    let tools = vec![
        create_test_tool("tool1", true),
        create_test_tool("tool2", false),
    ];

    let mut view_model = ToolListViewModel::new(tools);

    assert_eq!(view_model.len(), 2);
    assert!(!view_model.is_empty());

    let selected = view_model.selected_tool();
    assert!(selected.is_none());

    view_model.select_tool(0);
    let selected = view_model.selected_tool().unwrap();
    assert_eq!(selected.name, "tool1");
}

#[test]
fn test_app_state_management() {
    let mut state = AppState::new();

    assert_eq!(state.current_view, ViewType::MainMenu);

    state.set_view(ViewType::ToolList);
    assert_eq!(state.current_view, ViewType::ToolList);

    let tools = vec![create_test_tool("tool1", true)];
    state.set_tools(tools);

    state.select_tool(0);
    let selected = state.selected_tool().unwrap();
    assert_eq!(selected.name, "tool1");
    assert!(selected.is_available());
}

#[test]
fn test_component_composition() {
    // Test that components work together
    let tool = create_test_tool("composed", true);
    let card = ToolCard::new(tool.clone());

    let mut menu = Menu::new("Tool Actions".to_string());
    menu.add_item("Install".to_string(), "install".to_string(), None);
    menu.add_item("Info".to_string(), "info".to_string(), None);

    // Both components should render without errors
    let _card_output = card.render();
    let _menu_output = menu.render();

    // Test view model with component
    let tools = vec![tool];
    let view_model = ToolListViewModel::new(tools);

    // Should not panic
    view_model.display_list();
}
