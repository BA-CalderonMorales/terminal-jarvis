use terminal_jarvis::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::new();

    cli.run().await
}
