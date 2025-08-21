//! PLM 核心插件管理器实现

use crate::config::{PluginConfig, ProjectConfig};
use crate::traits::{Plugin, PluginError, InstallOptions, ValidationSummary};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::fs;

/// PLM 插件管理器
/// 
/// 负责管理插件的生命周期，包括注册、初始化、安装、卸载等操作
pub struct PluginManager {
    plugins: HashMap<String, Arc<dyn Plugin>>,
    config: ProjectConfig,
}

impl PluginManager {
    /// 创建新的插件管理器实例
    pub async fn new() -> Result<Self, PluginError> {
        let config = ProjectConfig::default_for_project("default", ".");
        Ok(Self {
            plugins: HashMap::new(),
            config,
        })
    }

    /// 从项目配置创建插件管理器
    pub async fn from_project_config(config: ProjectConfig) -> Result<Self, PluginError> {
        Ok(Self {
            plugins: HashMap::new(),
            config,
        })
    }

    /// 初始化插件管理器
    pub async fn initialize(&mut self) -> Result<(), PluginError> {
        // 初始化所有已注册的插件
        for (name, plugin) in &mut self.plugins {
            if let Err(e) = Arc::get_mut(plugin)
                .ok_or_else(|| PluginError::PluginError(format!("无法获取插件 {} 的可变引用", name)))?
                .initialize()
                .await
            {
                return Err(PluginError::PluginError(format!("插件 {} 初始化失败: {}", name, e)));
            }
        }
        Ok(())
    }

    /// 关闭插件管理器
    pub async fn shutdown(&mut self) -> Result<(), PluginError> {
        // 关闭所有插件
        for (name, plugin) in &mut self.plugins {
            if let Err(e) = Arc::get_mut(plugin)
                .ok_or_else(|| PluginError::PluginError(format!("无法获取插件 {} 的可变引用", name)))?
                .shutdown()
                .await
            {
                eprintln!("警告: 插件 {} 关闭失败: {}", name, e);
            }
        }
        self.plugins.clear();
        Ok(())
    }

    /// 注册插件（用于测试）
    pub async fn register_plugin_for_test(&mut self, name: String, plugin: Arc<dyn Plugin>) -> Result<(), PluginError> {
        self.plugins.insert(name, plugin);
        Ok(())
    }

    /// 获取插件
    pub async fn get_plugin(&self, name: &str) -> Result<Arc<dyn Plugin>, PluginError> {
        self.plugins
            .get(name)
            .cloned()
            .ok_or_else(|| PluginError::NotFound(name.to_string()))
    }

    /// 列出所有插件名称
    pub async fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// 安装插件
    pub async fn install_plugin(
        &self,
        name: &str,
        version: Option<&str>,
        options: &InstallOptions,
    ) -> Result<String, PluginError> {
        let plugin = self.get_plugin(name).await?;
        let version = version.unwrap_or("latest");
        plugin.install(version, options).await
    }

    /// 卸载插件
    pub async fn uninstall_plugin(&self, name: &str, version: &str) -> Result<(), PluginError> {
        let plugin = self.get_plugin(name).await?;
        plugin.uninstall(version).await
    }

    /// 发现插件
    pub async fn discover_plugins(&self) -> Result<usize, PluginError> {
        // 简化的发现逻辑 - 返回当前已注册的插件数量
        Ok(self.plugins.len())
    }

    /// 验证所有插件
    pub async fn validate_all_plugins(&self) -> Result<ValidationSummary, PluginError> {
        let mut summary = ValidationSummary {
            valid_plugins: 0,
            invalid_plugins: 0,
            errors: Vec::new(),
        };

        for (name, plugin) in &self.plugins {
            // 简化的验证逻辑 - 检查插件元数据
            let metadata = plugin.metadata();
            if !metadata.name.is_empty() && !metadata.version.is_empty() {
                summary.valid_plugins += 1;
            } else {
                summary.invalid_plugins += 1;
                summary.errors.push(format!("插件 {} 元数据不完整", name));
            }
        }

        Ok(summary)
    }

    /// 保存配置到文件
    pub async fn save_config(&self, path: &str) -> Result<(), PluginError> {
        let config_json = serde_json::to_string_pretty(&self.config)
            .map_err(|e| PluginError::ConfigError(format!("序列化配置失败: {}", e)))?;
        
        fs::write(path, config_json)
            .await
            .map_err(|e| PluginError::ConfigError(format!("写入配置文件失败: {}", e)))?;
        
        Ok(())
    }

    /// 获取项目配置
    pub fn get_config(&self) -> &ProjectConfig {
        &self.config
    }

    /// 更新项目配置
    pub fn update_config(&mut self, config: ProjectConfig) {
        self.config = config;
    }

    /// 添加插件配置
    pub fn add_plugin_config(&mut self, plugin_config: PluginConfig) {
        self.config.add_plugin(plugin_config);
    }

    /// 移除插件配置
    pub fn remove_plugin_config(&mut self, name: &str) {
        self.config.remove_plugin(name);
    }

    /// 获取插件配置
    pub fn get_plugin_config(&self, name: &str) -> Option<&PluginConfig> {
        self.config.get_plugin(name)
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        // 在析构时尝试清理资源
        if !self.plugins.is_empty() {
            eprintln!("警告: PluginManager 被销毁时仍有 {} 个插件未正确关闭", self.plugins.len());
        }
    }
}