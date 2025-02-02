mod commands;
mod core;
mod error;
mod providers;

use core::config_registry::ConfigRegistry;

use clap::Parser;
use commands::Commands;
use providers::biome::BiomeProvider;
use tracing::{Level, error, info};

#[derive(Parser)]
#[command(name = "amaris")]
#[command(author = "elizielx")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Automate opinionated development configurations.", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "info")]
    log_level: Level,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: CLI = CLI::parse();

    tracing_subscriber::fmt()
        .with_env_filter(cli.log_level.to_string())
        .init();

    let mut registry = ConfigRegistry::new();
    registry.register(BiomeProvider);

    match cli.command.execute(&registry).await {
        Ok(_) => {
            info!("Command completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Command failed: {}", e);
            Err(e)
        }
    }
}
