mod cli;
mod cli_logic;
mod config;
mod services;
mod api;
mod api_base;
mod api_client;

use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::new();
    cli.run().await
}
