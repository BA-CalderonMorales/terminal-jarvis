use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;

mod goose_handler;

pub use goose_handler::GooseHandler;

pub trait ToolHandler {
    fn pre_execution(&self, cmd: &mut Command) -> Result<()> {
        self.prepare_env(cmd)?;
        self.validate_auth(cmd)
    }

    fn validate_auth(&self, cmd: &mut Command) -> Result<()>;

    fn prepare_env(&self, cmd: &mut Command) -> Result<()>;

    fn uses_host_auth_environment(&self) -> bool {
        false
    }
}

pub fn tool_registry() -> HashMap<&'static str, Box<dyn ToolHandler>> {
    let mut registry: HashMap<&'static str, Box<dyn ToolHandler>> = HashMap::new();
    registry.insert("goose", Box::new(GooseHandler));
    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_goose_handler() {
        let registry = tool_registry();
        assert!(registry.contains_key("goose"));
    }
}
