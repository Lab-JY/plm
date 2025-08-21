//! Core traits for the plugin system

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Plugin error types
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Installation failed: {0}")]
    InstallationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Plugin error: {0}")]
    PluginError(String),
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Plugin homepage
    pub homepage: Option<String>,
    /// Plugin repository
    pub repository: Option<String>,
    /// Supported platforms
    pub supported_platforms: Vec<String>,
    /// Plugin tags
    pub tags: Vec<String>,
    /// Plugin dependencies
    pub dependencies: Vec<String>,
    /// Minimum PLM version
    pub min_plm_version: Option<String>,
}

/// Plugin status
#[derive(Debug, Clone, PartialEq)]
pub enum PluginStatus {
    /// Plugin is active and ready
    Active,
    /// Plugin is inactive
    Inactive,
    /// Plugin is loading
    Loading,
    /// Plugin has an error
    Error(String),
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Version string
    pub version: String,
    /// Target platform
    pub platform: String,
    /// Download URL
    pub download_url: String,
    /// File checksum
    pub checksum: Option<String>,
    /// Release date
    pub release_date: Option<String>,
    /// Pre-release flag
    pub prerelease: bool,
}

/// Installation options
#[derive(Debug, Clone)]
pub struct InstallOptions {
    /// Force installation
    pub force: bool,
    /// Enable debug mode
    pub debug: bool,
    /// Skip confirmation prompts
    pub yes: bool,
    /// Quiet mode (minimal output)
    pub quiet: bool,
    /// Custom installation directory
    pub install_dir: Option<String>,
    /// Additional environment variables
    pub env_vars: HashMap<String, String>,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            force: false,
            debug: false,
            yes: false,
            quiet: false,
            install_dir: None,
            env_vars: HashMap::new(),
        }
    }
}

/// Main plugin trait
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Get plugin status
    fn status(&self) -> PluginStatus;

    /// Initialize plugin
    async fn initialize(&mut self) -> Result<(), PluginError>;

    /// Shutdown plugin
    async fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Install a version of the tool
    async fn install(&self, version: &str, options: &InstallOptions)
        -> Result<String, PluginError>;

    /// Uninstall a version of the tool
    async fn uninstall(&self, version: &str) -> Result<(), PluginError>;

    /// List available versions
    async fn list_versions(&self) -> Result<Vec<VersionInfo>, PluginError>;

    /// List installed versions
    async fn list_installed(&self) -> Result<Vec<String>, PluginError>;

    /// Check if a version is installed
    async fn is_installed(&self, version: &str) -> Result<bool, PluginError>;

    /// Get the latest version
    async fn get_latest_version(&self) -> Result<VersionInfo, PluginError>;

    /// Update to latest or specific version
    async fn update(&self, version: Option<&str>) -> Result<String, PluginError>;

    /// Switch to a specific version
    async fn switch_version(&self, version: &str) -> Result<(), PluginError>;

    /// Verify installation
    async fn verify_installation(&self, version: &str) -> Result<bool, PluginError>;

    /// Clean up plugin cache
    async fn cleanup(&self) -> Result<(), PluginError>;

    /// Get plugin configuration
    async fn get_config(&self) -> Result<HashMap<String, String>, PluginError>;

    /// Set plugin configuration
    async fn set_config(&self, config: HashMap<String, String>) -> Result<(), PluginError>;

    /// Get specific configuration value
    async fn get_config_value(&self, key: &str) -> Result<Option<String>, PluginError>;

    /// Set specific configuration value
    async fn set_config_value(&self, key: &str, value: &str) -> Result<(), PluginError>;

    /// Execute plugin-specific command
    async fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, PluginError>;

    /// Get plugin help information
    fn get_help(&self) -> String;

    /// Check if plugin supports a specific feature
    fn supports_feature(&self, feature: &str) -> bool;
}

/// Plugin factory trait for creating plugins
#[async_trait]
pub trait PluginFactory: Send + Sync {
    /// Create a new plugin instance
    async fn create_plugin(
        &self,
        config: &crate::config::PluginConfig,
    ) -> Result<Box<dyn Plugin>, PluginError>;

    /// Get supported plugin types
    fn supported_types(&self) -> Vec<String>;

    /// Validate plugin configuration
    fn validate_config(&self, config: &crate::config::PluginConfig) -> Result<(), PluginError>;
}

/// Plugin loader trait for loading plugins from different sources
#[async_trait]
pub trait PluginLoader: Send + Sync {
    /// Load plugin from source
    async fn load_plugin(
        &self,
        source: &crate::config::PluginSource,
    ) -> Result<Box<dyn Plugin>, PluginError>;

    /// Check if source is supported
    fn supports_source(&self, source_type: &crate::config::PluginSourceType) -> bool;

    /// Validate plugin source
    async fn validate_source(
        &self,
        source: &crate::config::PluginSource,
    ) -> Result<(), PluginError>;
}

impl Default for PluginMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "0.1.0".to_string(),
            description: String::new(),
            author: String::new(),
            homepage: None,
            repository: None,
            supported_platforms: vec![
                "linux".to_string(),
                "macos".to_string(),
                "windows".to_string(),
            ],
            tags: Vec::new(),
            dependencies: Vec::new(),
            min_plm_version: None,
        }
    }
}

impl VersionInfo {
    /// Create new version info
    pub fn new(version: &str, platform: &str, download_url: &str) -> Self {
        Self {
            version: version.to_string(),
            platform: platform.to_string(),
            download_url: download_url.to_string(),
            checksum: None,
            release_date: None,
            prerelease: false,
        }
    }

    /// Set checksum
    pub fn with_checksum(mut self, checksum: &str) -> Self {
        self.checksum = Some(checksum.to_string());
        self
    }

    /// Set release date
    pub fn with_release_date(mut self, date: &str) -> Self {
        self.release_date = Some(date.to_string());
        self
    }

    /// Mark as prerelease
    pub fn as_prerelease(mut self) -> Self {
        self.prerelease = true;
        self
    }
}

/// Validation summary
#[derive(Debug, Default)]
pub struct ValidationSummary {
    pub valid_plugins: usize,
    pub invalid_plugins: usize,
    pub errors: Vec<String>,
}

impl ValidationSummary {
    /// Check if all plugins are valid
    pub fn is_all_valid(&self) -> bool {
        self.invalid_plugins == 0
    }

    /// Get total plugin count
    pub fn total_plugins(&self) -> usize {
        self.valid_plugins + self.invalid_plugins
    }
}

impl InstallOptions {
    /// Create new install options
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable force installation
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Enable debug mode
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// Skip confirmation prompts
    pub fn yes(mut self) -> Self {
        self.yes = true;
        self
    }

    /// Enable quiet mode
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Set custom installation directory
    pub fn install_dir(mut self, dir: &str) -> Self {
        self.install_dir = Some(dir.to_string());
        self
    }

    /// Add environment variable
    pub fn env_var(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }
}
