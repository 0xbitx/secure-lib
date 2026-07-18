use std::env;
use std::path::PathBuf;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Database connection URL
    pub database_url: String,
    /// Server bind address
    pub bind_address: String,
    /// Maximum number of worker threads
    pub max_workers: usize,
    /// Path to the log directory
    pub log_dir: PathBuf,
    /// Enable debug mode
    pub debug: bool,
}

/// Errors during configuration loading.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required variable: {0}")]
    MissingVariable(String),
    #[error("invalid value for {0}: {1}")]
    InvalidValue(String, String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://localhost:5432/app".into(),
            bind_address: "0.0.0.0:8080".into(),
            max_workers: 4,
            log_dir: PathBuf::from("/var/log/myapp"),
            debug: false,
        }
    }
}

/// Load configuration from environment variables with sensible defaults.
pub fn load_config() -> Result<AppConfig, ConfigError> {
    let mut config = AppConfig::default();

    if let Ok(val) = env::var("DATABASE_URL") {
        config.database_url = val;
    }

    if let Ok(val) = env::var("BIND_ADDRESS") {
        config.bind_address = val;
    }

    if let Ok(val) = env::var("MAX_WORKERS") {
        config.max_workers = val
            .parse()
            .map_err(|_| ConfigError::InvalidValue("MAX_WORKERS".into(), val))?;
    }

    if let Ok(val) = env::var("LOG_DIR") {
        config.log_dir = PathBuf::from(val);
    }

    if let Ok(val) = env::var("DEBUG") {
        config.debug = val == "1" || val.to_lowercase() == "true";
    }

    Ok(config)
}

/// Validate that the configuration is internally consistent.
pub fn validate_config(config: &AppConfig) -> Result<(), ConfigError> {
    if config.database_url.is_empty() {
        return Err(ConfigError::MissingVariable("DATABASE_URL".into()));
    }

    if config.max_workers == 0 {
        return Err(ConfigError::InvalidValue(
            "MAX_WORKERS".into(),
            config.max_workers.to_string(),
        ));
    }

    Ok(())
}

/// Merge two configs, with `override_config` taking precedence.
pub fn merge_configs(base: &AppConfig, override_config: &AppConfig) -> AppConfig {
    AppConfig {
        database_url: if override_config.database_url.is_empty() {
            base.database_url.clone()
        } else {
            override_config.database_url.clone()
        },
        bind_address: if override_config.bind_address.is_empty() {
            base.bind_address.clone()
        } else {
            override_config.bind_address.clone()
        },
        max_workers: if override_config.max_workers == 0 {
            base.max_workers
        } else {
            override_config.max_workers
        },
        log_dir: base.log_dir.clone(),
        debug: base.debug || override_config.debug,
    }
}
