mod commands;
mod core;
mod error;
mod providers;

use clap::Parser;
use commands::Commands;
use core::config_registry::ConfigRegistry;
use providers::biome::BiomeProvider;

#[derive(Parser)]
#[command(name = "amaris")]
#[command(author = "elizielx")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Automate opinionated development configurations.", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: CLI = CLI::parse();

    let mut registry = ConfigRegistry::new();
    registry.register(BiomeProvider);

    match cli.command.execute(&registry).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
