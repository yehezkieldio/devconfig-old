use crate::core::config_registry::ConfigProvider;
use crate::error::ConfigError;
use async_trait::async_trait;
use std::fs;
use std::path::Path;
use which::which;

pub struct BiomeProvider;

#[async_trait]
impl ConfigProvider for BiomeProvider {
    fn name(&self) -> &'static str {
        "biome"
    }

    fn description(&self) -> &'static str {
        "Biome configuration for JavaScript/TypeScript projects"
    }

    async fn check_prerequisites(&self) -> Result<(), ConfigError> {
        // Check for npm
        which("bun").map_err(|_| {
            ConfigError::MissingPrerequisite("Bun is required but not found".to_string())
        })?;

        // Check for package.json
        if !Path::new("package.json").exists() {
            return Err(ConfigError::MissingPrerequisite(
                "package.json not found. Run 'bun init' first".to_string(),
            ));
        }

        Ok(())
    }

    async fn install(&self) -> Result<(), ConfigError> {
        let output = tokio::process::Command::new("bun")
            .args(&["install", "--dev", "@biomejs/biome"])
            .output()
            .await
            .map_err(|e| ConfigError::DependencyError(e.to_string()))?;

        if !output.status.success() {
            return Err(ConfigError::DependencyError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    async fn remove(&self) -> Result<(), ConfigError> {
        // Remove config files
        for file in &["biome.json"] {
            if Path::new(file).exists() {
                fs::remove_file(file).map_err(|e| ConfigError::FileWriteError(e.to_string()))?;
            }
        }

        // Uninstall dependencies
        let output = tokio::process::Command::new("bun")
            .args(&["remove", "@biomejs/biome"])
            .output()
            .await
            .map_err(|e| ConfigError::DependencyError(e.to_string()))?;

        if !output.status.success() {
            return Err(ConfigError::DependencyError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }
}
