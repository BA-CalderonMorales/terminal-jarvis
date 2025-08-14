use crate::cli_logic;
use clap::{Parser, Subcommand};

/// Terminal Jarvis - A unified interface for AI coding tools
#[derive(Parser)]
#[command(name = "terminal-jarvis")]
#[command(about = "A thin Rust wrapper for managing and running AI coding tools")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    long_about = "Terminal Jarvis provides a unified interface for managing multiple AI coding tools like claude-code, gemini-cli, qwen-code, and opencode."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a specific AI coding tool
    Run {
        /// The tool to run (claude, gemini, qwen, opencode, llxprt, codex)
        tool: String,
        /// Arguments to pass to the tool
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Install a specific AI coding tool
    Install {
        /// The tool to install
        tool: String,
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
    /// Configuration management commands
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// Version cache management commands
    Cache {
        #[command(subcommand)]
        action: CacheCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Reset configuration to defaults
    Reset,
    /// Show current configuration
    Show,
    /// Show configuration file path
    Path,
}

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Clear the version cache
    Clear,
    /// Show cache status
    Status,
    /// Refresh cache with latest version
    Refresh {
        /// Cache TTL in seconds (default: 3600)
        #[arg(long, default_value = "3600")]
        ttl: u64,
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
            Some(Commands::Run { tool, args }) => cli_logic::handle_run_tool(&tool, &args).await,
            Some(Commands::Install { tool }) => cli_logic::handle_install_tool(&tool).await,
            Some(Commands::Update { package }) => {
                cli_logic::handle_update_packages(package.as_deref()).await
            }
            Some(Commands::List) => cli_logic::handle_list_tools().await,
            Some(Commands::Info { tool }) => cli_logic::handle_tool_info(&tool).await,
            Some(Commands::Templates { action }) => match action {
                TemplateCommands::Init => cli_logic::handle_templates_init().await,
                TemplateCommands::Create { name } => {
                    cli_logic::handle_templates_create(&name).await
                }
                TemplateCommands::List => cli_logic::handle_templates_list().await,
                TemplateCommands::Apply { name } => cli_logic::handle_templates_apply(&name).await,
            },
            Some(Commands::Config { action }) => match action {
                ConfigCommands::Reset => cli_logic::handle_config_reset().await,
                ConfigCommands::Show => cli_logic::handle_config_show().await,
                ConfigCommands::Path => cli_logic::handle_config_path().await,
            },
            Some(Commands::Cache { action }) => match action {
                CacheCommands::Clear => cli_logic::handle_cache_clear().await,
                CacheCommands::Status => cli_logic::handle_cache_status().await,
                CacheCommands::Refresh { ttl } => cli_logic::handle_cache_refresh(ttl).await,
            },
            None => cli_logic::handle_interactive_mode().await,
        }
    }
}
