use clap::Subcommand;
use inquire::Select;
use tracing::info;

use crate::core::config_registry::ConfigRegistry;

#[derive(Subcommand)]
pub enum Commands {
    Install {
        #[arg(short, long)]
        config: Option<String>,
    },
    List,
    Remove {
        #[arg(short, long)]
        config: String,
    },
    Doctor,
}

impl Commands {
    pub async fn execute(&self, registry: &ConfigRegistry) -> anyhow::Result<()> {
        match self {
            Commands::Install { config } => {
                let config_name = match config {
                    Some(name) => name.clone(),
                    None => {
                        let configs = registry.available_configs();
                        let options: Vec<_> = configs.iter().map(|(_, desc)| *desc).collect();
                        let selection =
                            Select::new("Select configuration to install:", options).prompt()?;
                        configs
                            .iter()
                            .find(|(_, desc)| *desc == selection)
                            .map(|(name, _)| name.to_string())
                            .unwrap()
                    }
                };

                if let Some(provider) = registry.get_provider(&config_name) {
                    info!("Installing {} configuration...", config_name);
                    provider.check_prerequisites().await?;
                    provider.install().await?;
                    info!("Installation complete!");
                }
            }
            Commands::List => {
                println!("Available configurations:");
                for (name, description) in registry.available_configs() {
                    println!("- {}: {}", name, description);
                }
            }
            Commands::Remove { config } => {
                if let Some(provider) = registry.get_provider(config) {
                    info!("Removing {} configuration...", config);
                    provider.remove().await?;
                    info!("Removal complete!");
                }
            }
            Commands::Doctor => {
                info!("Running diagnostics...");
                for (name, _) in registry.available_configs() {
                    if let Some(provider) = registry.get_provider(&name) {
                        match provider.check_prerequisites().await {
                            Ok(_) => println!("✅ {}: All prerequisites met", name),
                            Err(e) => println!("❌ {}: {}", name, e),
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
