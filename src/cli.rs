use crate::cli_logic;
use clap::{Parser, Subcommand};

/// Terminal Jarvis - A unified interface for AI coding tools
#[derive(Parser)]
#[command(name = "terminal-jarvis")]
#[command(about = "A thin Rust wrapper for managing and running AI coding tools")]
#[command(
    long_about = "Terminal Jarvis provides a unified interface for managing multiple AI coding tools like claude-code, gemini-cli, qwen-code, and opencode."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a specific AI coding tool
    Run {
        /// The tool to run (claude-code, gemini-cli, qwen-code, opencode)
        tool: String,
        /// Arguments to pass to the tool
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Update packages
    Update {
        /// Specific package to update (optional - updates all if not specified)
        package: Option<String>,
    },
    /// List available tools
    List,
    /// Show information about a specific tool
    Info {
        /// The tool to show information about
        tool: String,
    },
    /// Template management commands
    Templates {
        #[command(subcommand)]
        action: TemplateCommands,
    },
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    /// Initialize template repository (requires gh CLI)
    Init,
    /// Create a new template
    Create {
        /// Name of the template
        name: String,
    },
    /// List available templates
    List,
    /// Apply a template
    Apply {
        /// Name of the template to apply
        name: String,
    },
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Run { tool, args } => cli_logic::handle_run_tool(&tool, &args).await,
            Commands::Update { package } => {
                cli_logic::handle_update_packages(package.as_deref()).await
            }
            Commands::List => cli_logic::handle_list_tools().await,
            Commands::Info { tool } => cli_logic::handle_tool_info(&tool).await,
            Commands::Templates { action } => match action {
                TemplateCommands::Init => cli_logic::handle_templates_init().await,
                TemplateCommands::Create { name } => {
                    cli_logic::handle_templates_create(&name).await
                }
                TemplateCommands::List => cli_logic::handle_templates_list().await,
                TemplateCommands::Apply { name } => cli_logic::handle_templates_apply(&name).await,
            },
        }
    }
}
