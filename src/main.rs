mod api;
mod api_base;
mod api_client;
mod cli;
mod cli_logic;
mod config;
mod installation_arguments;
mod services;
mod tools;

use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::new();
    cli.run().await
}
