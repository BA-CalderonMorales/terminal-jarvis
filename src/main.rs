mod api;
mod api_base;
mod api_client;
mod auth_manager;
mod cli;
mod cli_logic;
mod config;
mod installation_arguments;
mod progress_utils;
mod services;
mod theme;
mod theme_config;
mod tools;

use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let cli = Cli::new();
  cli.run().await
}
