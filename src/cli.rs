use crate::cli_logic;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

/// Terminal Jarvis - A unified interface for AI coding tools
#[derive(Parser)]
#[command(name = "terminal-jarvis")]
#[command(about = "A thin Rust wrapper for managing and running AI coding tools")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    long_about = "Terminal Jarvis provides a unified interface for managing multiple AI coding tools like claude-code, gemini-cli, qwen-code, opencode, aider, amp, goose, ollama, vibe, droid, forge, and many more."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Quick mode: skip all prompts, launch last-used tool immediately
    #[arg(short = 'q', long = "quick", global = true)]
    pub quick: bool,

    /// Headless mode: no interactive prompts, no ANSI escape codes, plain text output.
    /// Also enabled by setting JARVIS_HEADLESS=1 or when stdin is not a TTY.
    #[arg(long, global = true, env = "JARVIS_HEADLESS")]
    pub headless: bool,

    /// Auto-confirm all prompts (implied in headless mode)
    #[arg(short = 'y', long, global = true)]
    pub yes: bool,
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

    /// Check if a string is a valid tool name for direct invocation.
    /// Dynamically reads from config/tools/*.toml (single source of truth).
    fn is_valid_tool(name: &str) -> bool {
        let loader = crate::tools::tools_config::get_tool_config_loader();
        let lower = name.to_lowercase();
        loader
            .get_tool_names()
            .iter()
            .any(|t| t.to_lowercase() == lower)
    }

    /// Find the Go ADK binary for the home screen.
    /// Search strategy:
    /// 0. Respect JARVIS_DISABLE_ADK=1 to force Rust interactive fallback
    /// 1. JARVIS_ADK_BINARY environment override (if it exists)
    /// 2. Same directory as current exe (NPM style)
    /// 3. Walk upward for adk/jarvis (repo style)
    fn find_adk_binary() -> Option<PathBuf> {
        if std::env::var("JARVIS_DISABLE_ADK")
            .ok()
            .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        {
            return None;
        }

        if let Ok(explicit) = std::env::var("JARVIS_ADK_BINARY") {
            let p = PathBuf::from(explicit);
            if p.exists() {
                return Some(p);
            }
        }

        if let Ok(exe) = std::env::current_exe() {
            // Check same directory (NPM style)
            if let Some(dir) = exe.parent() {
                let adk = dir.join("jarvis");
                if adk.exists() {
                    return Some(adk);
                }
            }

            // Check repo-style paths
            let mut dir = exe.parent();
            while let Some(d) = dir {
                let adk = d.join("adk").join("jarvis");
                if adk.exists() {
                    return Some(adk);
                }
                dir = d.parent();
            }
        }

        None
    }

    /// Try to launch ADK binary. Returns true only on successful launch and wait.
    fn try_launch_adk_binary(path: &Path) -> bool {
        match std::process::Command::new(path).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(status) if status.success() => true,
                Ok(status) => {
                    eprintln!(
                        "warning: Go ADK home screen exited with status {} ({}); falling back",
                        status,
                        path.display()
                    );
                    false
                }
                Err(e) => {
                    eprintln!(
                        "warning: failed while waiting for Go ADK home screen ({}): {}",
                        path.display(),
                        e
                    );
                    false
                }
            },
            Err(e) => {
                eprintln!(
                    "warning: failed to launch Go ADK home screen ({}): {}",
                    path.display(),
                    e
                );
                false
            }
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        // Initialize headless detection from CLI flags (and TTY check)
        cli_logic::cli_logic_headless::init(self.headless, self.yes);

        // Handle quick mode: launch last-used tool immediately
        if self.quick && self.command.is_none() {
            return cli_logic::handle_quick_launch().await;
        }

        match self.command {
            Some(Commands::Run { tool, args }) => cli_logic::handle_run_tool(&tool, &args).await,

            Some(Commands::External(args)) => {
                if args.is_empty() {
                    if cli_logic::cli_logic_headless::is_headless() {
                        eprintln!("error: interactive mode is not available in headless mode");
                        eprintln!(
                            "hint: use 'terminal-jarvis list' or 'terminal-jarvis run <tool>'"
                        );
                        std::process::exit(1);
                    }
                    return cli_logic::handle_interactive_mode().await;
                }
                let tool_name = &args[0];
                let tool_args = &args[1..];

                if Self::is_valid_tool(tool_name) {
                    return cli_logic::handle_run_tool(tool_name, tool_args).await;
                }

                // Invalid tool name - show error and exit
                let loader = crate::tools::tools_config::get_tool_config_loader();
                let tool_names = loader.get_tool_names();
                eprintln!("error: '{tool_name}' is not a valid tool or command");
                eprintln!();
                eprintln!("Available tools: {}", tool_names.join(", "));
                eprintln!();
                eprintln!("For more information, try '--help'");
                std::process::exit(1);
            }

            Some(Commands::Install { tool }) => cli_logic::handle_install_tool(&tool).await,

            Some(Commands::Update { package }) => {
                cli_logic::handle_update_packages(package.as_deref()).await
            }

            Some(Commands::List) => cli_logic::handle_list_tools().await,

            Some(Commands::Info { tool }) => cli_logic::handle_tool_info(&tool).await,

            Some(Commands::Auth { action }) => match action {
                AuthCommands::Manage => cli_logic::handle_authentication_menu().await,
                AuthCommands::Help { tool } => cli_logic::handle_auth_help(&tool).await,
                AuthCommands::Set { tool } => {
                    if let Some(t) = tool {
                        cli_logic::handle_auth_set(&t).await
                    } else {
                        cli_logic::handle_authentication_menu().await
                    }
                }
            },

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

            Some(Commands::Db { action }) => match action {
                DbCommands::Import => cli_logic::handle_db_import().await,
                DbCommands::Status => cli_logic::handle_db_status().await,
                DbCommands::Reset { force } => cli_logic::handle_db_reset(force).await,
            },

            Some(Commands::Status) => cli_logic::handle_status_command().await,

            None => {
                if cli_logic::cli_logic_headless::is_headless() {
                    eprintln!("error: interactive mode is not available in headless mode");
                    eprintln!("hint: use 'terminal-jarvis list' or 'terminal-jarvis run <tool>'");
                    std::process::exit(1);
                }

                // Prioritize launching the Go ADK home screen when available.
                if let Some(adk) = Self::find_adk_binary() {
                    if Self::try_launch_adk_binary(&adk) {
                        return Ok(());
                    }
                    // Fall back to Rust interactive mode if ADK fails to launch.
                }

                cli_logic::handle_interactive_mode().await
            }
        }
    }
}

/// Terminal Jarvis - A unified interface for AI coding tools
#[derive(Subcommand)]
pub enum Commands {
    /// Run a specific AI coding tool
    Run {
        /// The tool to run (claude, gemini, qwen, opencode, llxprt, codex, aider, amp, goose, crush, ollama, vibe, droid, forge...)
        tool: String,
        /// Arguments to pass to the tool
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    #[command(external_subcommand)]
    External(Vec<String>),

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

    /// Authentication management commands
    Auth {
        #[command(subcommand)]
        action: AuthCommands,
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

    /// Database management commands
    Db {
        #[command(subcommand)]
        action: DbCommands,
    },

    /// Show tool health status dashboard
    Status,
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

#[derive(Subcommand)]
#[command(disable_help_subcommand = true)]
pub enum AuthCommands {
    /// Open interactive authentication menu
    Manage,

    /// Show auth help for a tool
    Help {
        /// Tool name (e.g., claude, gemini, goose)
        tool: String,
    },

    /// Set and save credentials for a tool (guided)
    Set {
        /// Optional tool name; if omitted, opens interactive menu
        #[arg(long)]
        tool: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum DbCommands {
    /// Import tool configurations from TOML files into database
    Import,

    /// Show database status and statistics
    Status,

    /// Reset database (WARNING: deletes all data)
    Reset {
        /// Confirm reset without prompting
        #[arg(long)]
        force: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use std::path::{Path, PathBuf};

    #[test]
    fn launch_adk_failure_returns_false_instead_of_panicking() {
        let missing = PathBuf::from("/tmp/terminal-jarvis-does-not-exist/jarvis");
        assert!(!Cli::try_launch_adk_binary(&missing));
    }

    #[test]
    fn disable_adk_env_skips_discovery() {
        std::env::set_var("JARVIS_DISABLE_ADK", "1");
        assert!(Cli::find_adk_binary().is_none());
        std::env::remove_var("JARVIS_DISABLE_ADK");
    }

    #[test]
    fn invalid_adk_override_path_is_ignored() {
        std::env::remove_var("JARVIS_DISABLE_ADK");
        std::env::set_var("JARVIS_ADK_BINARY", "/tmp/does-not-exist/jarvis");
        let discovered = Cli::find_adk_binary();
        std::env::remove_var("JARVIS_ADK_BINARY");
        if let Some(path) = discovered {
            assert!(
                path.exists(),
                "discovered ADK path must exist: {}",
                path.display()
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn adk_nonzero_exit_triggers_fallback() {
        use std::os::unix::fs::PermissionsExt;

        let dir =
            std::env::temp_dir().join(format!("terminal-jarvis-adk-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let script = dir.join("fake-adk.sh");
        std::fs::write(&script, "#!/usr/bin/env sh\nexit 7\n").expect("write script");
        let mut perms = std::fs::metadata(&script).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script, perms).expect("chmod");

        assert!(!Cli::try_launch_adk_binary(Path::new(&script)));

        let _ = std::fs::remove_file(script);
        let _ = std::fs::remove_dir_all(dir);
    }
}
