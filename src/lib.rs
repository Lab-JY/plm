//! PLM - Plugin Lifecycle Manager
//!
//! This library provides a complete plugin lifecycle management system that can be
//! integrated into any Rust project through simple configuration.

pub mod config;
pub mod core;
pub mod traits;

// Re-export main types for easy use
pub use config::{PluginConfig, ProjectConfig};
pub use core::PluginManager;
pub use traits::{Plugin, PluginError, PluginMetadata};

/// Initialize plugin manager from project configuration
pub async fn init_from_config(config_path: &str) -> Result<PluginManager, PluginError> {
    let project_config = ProjectConfig::load_from_file(config_path).await?;
    PluginManager::from_project_config(project_config).await
}

/// Initialize plugin manager with default configuration
pub async fn init_default() -> Result<PluginManager, PluginError> {
    PluginManager::new().await
}

/// Quick setup for projects - creates default configuration
pub async fn quick_setup(project_name: &str, project_root: &str) -> Result<(), PluginError> {
    let config = ProjectConfig::default_for_project(project_name, project_root);
    config
        .save_to_file(&format!("{}/plm.json", project_root))
        .await?;
    println!("✅ PLM 配置文件已创建: {}/plm.json", project_root);
    Ok(())
}
