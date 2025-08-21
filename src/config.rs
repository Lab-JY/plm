//! PLM 配置管理模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::traits::PluginError;

/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
    pub global_settings: GlobalSettings,
    pub plugins: HashMap<String, PluginConfig>,
    pub sources: Vec<PluginSource>,
    
    // 兼容性字段
    pub project_name: String,
    pub project_root: String,
    pub version: String,
    pub settings: GlobalSettings,
}

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub root_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 全局设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub cache_dir: String,
    pub registry_url: String,
    pub auto_update: bool,
    pub parallel_downloads: u32,
    pub verify_checksums: bool,
    pub auto_discovery: bool,
    pub validate_on_install: bool,
    pub enable_hooks: bool,
    pub plugin_dir: String,
    pub log_level: String,
    pub download_timeout: u64,
}

/// 插件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    pub enabled: bool,
    pub version: Option<String>,
    pub source: Option<PluginSource>,
    pub settings: HashMap<String, serde_json::Value>,
    pub auto_update: bool,
}

/// 插件源类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginSourceType {
    Builtin,
    Local,
    Git,
    Http,
    Registry,
}

/// 插件源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSource {
    #[serde(rename = "type")]
    pub source_type: PluginSourceType,
    pub url: String,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub token: Option<String>,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            cache_dir: "~/.plm/cache".to_string(),
            registry_url: "https://registry.plm.dev".to_string(),
            auto_update: true,
            parallel_downloads: 4,
            verify_checksums: true,
            auto_discovery: true,
            validate_on_install: true,
            enable_hooks: true,
            plugin_dir: "~/.plm/plugins".to_string(),
            log_level: "info".to_string(),
            download_timeout: 300,
        }
    }
}

impl ProjectConfig {
    /// 为项目创建默认配置
    pub fn default_for_project(name: &str, root_path: &str) -> Self {
        let now = Utc::now();
        let settings = GlobalSettings::default();
        Self {
            project: ProjectInfo {
                name: name.to_string(),
                version: "1.0.0".to_string(),
                description: None,
                root_path: root_path.to_string(),
                created_at: now,
                updated_at: now,
            },
            global_settings: settings.clone(),
            plugins: HashMap::new(),
            sources: vec![
                PluginSource {
                    source_type: PluginSourceType::Registry,
                    url: "https://registry.plm.dev".to_string(),
                    branch: None,
                    tag: None,
                    token: None,
                },
            ],
            // 兼容性字段
            project_name: name.to_string(),
            project_root: root_path.to_string(),
            version: "1.0.0".to_string(),
            settings,
        }
    }

    /// 从文件加载配置
    pub async fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: ProjectConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// 从文件加载配置（兼容性方法）
    pub async fn load_from_file(path: &str) -> Result<Self, PluginError> {
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| PluginError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| PluginError::ConfigError(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }

    /// 保存配置到文件
    pub async fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    /// 保存配置到文件（兼容性方法）
    pub async fn save_to_file(&self, path: &str) -> Result<(), PluginError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| PluginError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        tokio::fs::write(path, content).await
            .map_err(|e| PluginError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), PluginError> {
        if self.project_name.is_empty() {
            return Err(PluginError::ConfigError("Project name cannot be empty".to_string()));
        }

        if self.project_root.is_empty() {
            return Err(PluginError::ConfigError("Project root cannot be empty".to_string()));
        }

        for (name, plugin) in &self.plugins {
            if plugin.name != *name {
                return Err(PluginError::ConfigError(
                    format!("Plugin name mismatch: key '{}' vs config '{}'", name, plugin.name)
                ));
            }
        }

        for source in &self.sources {
            if source.url.is_empty() {
                return Err(PluginError::ConfigError("Plugin source URL cannot be empty".to_string()));
            }
        }

        Ok(())
    }

    /// 添加插件配置
    pub fn add_plugin(&mut self, plugin: PluginConfig) {
        self.plugins.insert(plugin.name.clone(), plugin);
    }

    /// 获取插件配置
    pub fn get_plugin(&self, plugin_name: &str) -> Option<&PluginConfig> {
        self.plugins.get(plugin_name)
    }

    /// 获取可变插件配置
    pub fn get_plugin_mut(&mut self, plugin_name: &str) -> Option<&mut PluginConfig> {
        self.plugins.get_mut(plugin_name)
    }

    /// 获取所有插件配置
    pub fn get_plugins(&self) -> &HashMap<String, PluginConfig> {
        &self.plugins
    }

    /// 获取项目名称
    pub fn get_project_name(&self) -> &str {
        &self.project_name
    }

    /// 获取项目根路径
    pub fn get_project_root(&self) -> &str {
        &self.project_root
    }

    /// 更新插件设置
    pub fn update_plugin_setting(&mut self, plugin_name: &str, key: &str, value: serde_json::Value) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            plugin.set_setting(key, value);
            Ok(())
        } else {
            Err(format!("Plugin '{}' not found", plugin_name))
        }
    }

    /// 移除插件
    pub fn remove_plugin(&mut self, plugin_name: &str) -> Option<PluginConfig> {
        self.plugins.remove(plugin_name)
    }

    /// 启用插件
    pub fn enable_plugin(&mut self, plugin_name: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            plugin.enabled = true;
            Ok(())
        } else {
            Err(format!("Plugin '{}' not found", plugin_name))
        }
    }

    /// 禁用插件
    pub fn disable_plugin(&mut self, plugin_name: &str) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            plugin.enabled = false;
            Ok(())
        } else {
            Err(format!("Plugin '{}' not found", plugin_name))
        }
    }
}

impl PluginConfig {
    /// 创建新的插件配置
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            enabled: false,
            version: None,
            source: None,
            settings: HashMap::new(),
            auto_update: false,
        }
    }

    /// 获取版本
    pub fn get_version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// 设置版本
    pub fn set_version(&mut self, version: &str) {
        self.version = Some(version.to_string());
    }

    /// 设置插件源
    pub fn set_source(&mut self, source: PluginSource) {
        self.source = Some(source);
    }

    /// 设置配置项
    pub fn set_setting(&mut self, key: &str, value: serde_json::Value) {
        self.settings.insert(key.to_string(), value);
    }

    /// 获取配置项
    pub fn get_setting(&self, key: &str) -> Option<&serde_json::Value> {
        self.settings.get(key)
    }

    /// 移除配置项
    pub fn remove_setting(&mut self, key: &str) -> Option<serde_json::Value> {
        self.settings.remove(key)
    }

    /// 清空所有设置
    pub fn clear_settings(&mut self) {
        self.settings.clear();
    }

    /// 获取所有设置
    pub fn get_all_settings(&self) -> &HashMap<String, serde_json::Value> {
        &self.settings
    }
}

impl PluginSource {
    /// 创建本地插件源
    pub fn local(path: &str) -> Self {
        PluginSource {
            source_type: PluginSourceType::Local,
            url: path.to_string(),
            branch: None,
            tag: None,
            token: None,
        }
    }

    /// 创建注册表插件源
    pub fn registry(url: &str) -> Self {
        PluginSource {
            source_type: PluginSourceType::Registry,
            url: url.to_string(),
            branch: None,
            tag: None,
            token: None,
        }
    }

    /// 创建 Git 插件源
    pub fn git(url: &str, branch: Option<&str>) -> Self {
        PluginSource {
            source_type: PluginSourceType::Git,
            url: url.to_string(),
            branch: branch.map(|s| s.to_string()),
            tag: None,
            token: None,
        }
    }

    /// 创建简单的 Git 插件源（使用默认分支）
    pub fn git_simple(url: &str) -> Self {
        PluginSource {
            source_type: PluginSourceType::Git,
            url: url.to_string(),
            branch: None,
            tag: None,
            token: None,
        }
    }

    /// 创建 HTTP 插件源
    pub fn http(url: &str) -> Self {
        PluginSource {
            source_type: PluginSourceType::Http,
            url: url.to_string(),
            branch: None,
            tag: None,
            token: None,
        }
    }

    /// 获取源的 URL
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// 获取源类型名称
    pub fn get_type_name(&self) -> &'static str {
        match self.source_type {
            PluginSourceType::Local => "local",
            PluginSourceType::Registry => "registry",
            PluginSourceType::Git => "git",
            PluginSourceType::Http => "http",
            PluginSourceType::Builtin => "builtin",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_config_creation() {
        let config = ProjectConfig::default_for_project("test-project", "/tmp");
        assert_eq!(config.project_name, "test-project");
        assert_eq!(config.project_root, "/tmp");
        assert!(config.plugins.is_empty());
    }

    #[test]
    fn test_plugin_config_creation() {
        let mut plugin = PluginConfig::new("test-plugin");
        assert_eq!(plugin.name, "test-plugin");
        assert!(!plugin.enabled);
        assert!(plugin.version.is_none());

        plugin.set_version("1.0.0");
        assert_eq!(plugin.get_version(), Some("1.0.0"));
    }

    #[test]
    fn test_plugin_source_creation() {
        let local_source = PluginSource::local("/path/to/plugin");
        assert_eq!(local_source.get_url(), "/path/to/plugin");
        assert_eq!(local_source.get_type_name(), "local");

        let registry_source = PluginSource::registry("https://registry.example.com");
        assert_eq!(registry_source.get_url(), "https://registry.example.com");
        assert_eq!(registry_source.get_type_name(), "registry");

        let git_source = PluginSource::git("https://github.com/user/repo.git", Some("main"));
        assert_eq!(git_source.get_url(), "https://github.com/user/repo.git");
        assert_eq!(git_source.get_type_name(), "git");
    }

    #[test]
    fn test_plugin_settings() {
        let mut plugin = PluginConfig::new("test-plugin");
        
        plugin.set_setting("debug", serde_json::Value::Bool(true));
        plugin.set_setting("timeout", serde_json::Value::Number(serde_json::Number::from(30)));
        
        assert_eq!(plugin.get_setting("debug"), Some(&serde_json::Value::Bool(true)));
        assert_eq!(plugin.get_setting("timeout"), Some(&serde_json::Value::Number(serde_json::Number::from(30))));
        assert_eq!(plugin.get_setting("nonexistent"), None);
    }
}