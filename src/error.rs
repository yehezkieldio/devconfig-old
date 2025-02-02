use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[allow(dead_code)]
    #[error("Configuration '{0}' already exists")]
    AlreadyExists(String),

    #[error("Missing prerequisite: {0}")]
    MissingPrerequisite(String),

    #[error("Failed to write configuration file: {0}")]
    FileWriteError(String),

    #[allow(dead_code)]
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),

    #[error("Dependency installation failed: {0}")]
    DependencyError(String),
}
