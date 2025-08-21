//! PLM 集成测试

use async_trait::async_trait;
use plm::config::PluginSource;
use plm::traits::{InstallOptions, Plugin, PluginError, PluginMetadata, PluginStatus, VersionInfo};
use plm::{PluginConfig, PluginManager, ProjectConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tokio;

/// 测试用的模拟插件
pub struct MockPlugin {
    metadata: PluginMetadata,
    status: PluginStatus,
    installed_versions: Vec<String>,
}

impl MockPlugin {
    pub fn new(name: &str) -> Self {
        let metadata = PluginMetadata {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: format!("测试插件 {}", name),
            author: "PLM Test Suite".to_string(),
            homepage: Some("https://test.plm.dev".to_string()),
            repository: Some("https://github.com/plm/test".to_string()),
            supported_platforms: vec!["linux".to_string(), "macos".to_string()],
            tags: vec!["test".to_string()],
            dependencies: vec![],
            min_plm_version: Some("0.1.0".to_string()),
        };

        Self {
            metadata,
            status: PluginStatus::Inactive,
            installed_versions: vec!["1.0.0".to_string()],
        }
    }
}

#[async_trait]
impl Plugin for MockPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn status(&self) -> PluginStatus {
        self.status.clone()
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        self.status = PluginStatus::Active;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), PluginError> {
        self.status = PluginStatus::Inactive;
        Ok(())
    }

    async fn install(
        &self,
        version: &str,
        _options: &InstallOptions,
    ) -> Result<String, PluginError> {
        Ok(format!("/tmp/test-{}-{}", self.metadata.name, version))
    }

    async fn uninstall(&self, _version: &str) -> Result<(), PluginError> {
        Ok(())
    }

    async fn list_versions(&self) -> Result<Vec<VersionInfo>, PluginError> {
        Ok(vec![
            VersionInfo::new("1.0.0", "linux-x64", "https://test.com/v1.0.0"),
            VersionInfo::new("1.1.0", "linux-x64", "https://test.com/v1.1.0"),
        ])
    }

    async fn list_installed(&self) -> Result<Vec<String>, PluginError> {
        Ok(self.installed_versions.clone())
    }

    async fn is_installed(&self, version: &str) -> Result<bool, PluginError> {
        Ok(self.installed_versions.contains(&version.to_string()))
    }

    async fn get_latest_version(&self) -> Result<VersionInfo, PluginError> {
        Ok(VersionInfo::new(
            "1.1.0",
            "linux-x64",
            "https://test.com/v1.1.0",
        ))
    }

    async fn update(&self, version: Option<&str>) -> Result<String, PluginError> {
        let target_version = version.unwrap_or("1.1.0");
        Ok(target_version.to_string())
    }

    async fn switch_version(&self, _version: &str) -> Result<(), PluginError> {
        Ok(())
    }

    async fn verify_installation(&self, _version: &str) -> Result<bool, PluginError> {
        Ok(true)
    }

    async fn cleanup(&self) -> Result<(), PluginError> {
        Ok(())
    }

    async fn get_config(&self) -> Result<HashMap<String, String>, PluginError> {
        Ok(HashMap::new())
    }

    async fn set_config(&self, _config: HashMap<String, String>) -> Result<(), PluginError> {
        Ok(())
    }

    async fn get_config_value(&self, _key: &str) -> Result<Option<String>, PluginError> {
        Ok(None)
    }

    async fn set_config_value(&self, _key: &str, _value: &str) -> Result<(), PluginError> {
        Ok(())
    }

    async fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, PluginError> {
        Ok(format!("执行命令: {} {:?}", command, args))
    }

    fn get_help(&self) -> String {
        format!("测试插件 {} 的帮助信息", self.metadata.name)
    }

    fn supports_feature(&self, feature: &str) -> bool {
        matches!(feature, "install" | "uninstall" | "update" | "config")
    }
}

#[tokio::test]
async fn test_plugin_manager_creation() {
    let config = ProjectConfig::default_for_project("test-project", ".");
    let manager = PluginManager::from_project_config(config).await;
    assert!(manager.is_ok());
}

#[tokio::test]
async fn test_plugin_registration_and_initialization() {
    let config = ProjectConfig::default_for_project("test-project", ".");
    let mut manager = PluginManager::from_project_config(config).await.unwrap();

    // 注册测试插件
    let mock_plugin = Arc::new(MockPlugin::new("test-node"));
    manager
        .register_plugin_for_test("test-node".to_string(), mock_plugin)
        .await
        .unwrap();

    // 初始化
    let result = manager.initialize().await;
    assert!(result.is_ok());

    // 验证插件已注册
    let plugins = manager.list_plugins().await;
    assert!(plugins.contains(&"test-node".to_string()));
}

#[tokio::test]
async fn test_plugin_installation() {
    let config = ProjectConfig::default_for_project("test-project", ".");
    let mut manager = PluginManager::from_project_config(config).await.unwrap();

    // 注册和初始化
    let mock_plugin = Arc::new(MockPlugin::new("test-python"));
    manager
        .register_plugin_for_test("test-python".to_string(), mock_plugin)
        .await
        .unwrap();
    manager.initialize().await.unwrap();

    // 测试安装
    let options = InstallOptions::new();
    let result = manager
        .install_plugin("test-python", Some("1.0.0"), &options)
        .await;
    assert!(result.is_ok());

    let install_path = result.unwrap();
    assert!(install_path.contains("test-python"));
    assert!(install_path.contains("1.0.0"));
}

#[tokio::test]
async fn test_plugin_validation() {
    let config = ProjectConfig::default_for_project("test-project", ".");
    let mut manager = PluginManager::from_project_config(config).await.unwrap();

    // 注册多个测试插件
    let plugins = vec!["test-go", "test-rust", "test-java"];
    for plugin_name in &plugins {
        let mock_plugin = Arc::new(MockPlugin::new(plugin_name));
        manager
            .register_plugin_for_test(plugin_name.to_string(), mock_plugin)
            .await
            .unwrap();
    }

    manager.initialize().await.unwrap();

    // 验证所有插件
    let validation_result = manager.validate_all_plugins().await;
    assert!(validation_result.is_ok());

    let summary = validation_result.unwrap();
    assert_eq!(summary.valid_plugins, plugins.len());
    assert_eq!(summary.invalid_plugins, 0);
    assert!(summary.errors.is_empty());
}

#[tokio::test]
async fn test_config_management() {
    let mut config = ProjectConfig::default_for_project("test-project", ".");

    // 添加插件配置
    let mut plugin_config = PluginConfig::new("test-config");
    plugin_config.enabled = true;
    plugin_config.set_version("2.0.0");
    plugin_config.set_source(PluginSource::registry("https://test.registry.com"));
    plugin_config.set_setting("debug", serde_json::Value::Bool(true));

    config.add_plugin(plugin_config);

    // 验证配置
    let plugin_configs = config.get_plugins();
    assert!(plugin_configs.contains_key("test-config"));

    let test_config = &plugin_configs["test-config"];
    assert!(test_config.enabled);
    assert_eq!(test_config.get_version(), Some("2.0.0"));

    let debug_setting = test_config.get_setting("debug");
    assert!(debug_setting.is_some());
    assert_eq!(debug_setting.unwrap(), &serde_json::Value::Bool(true));
}

#[tokio::test]
async fn test_plugin_discovery() {
    let config = ProjectConfig::default_for_project("test-project", ".");
    let mut manager = PluginManager::from_project_config(config).await.unwrap();

    // 注册一些插件
    let plugins = vec!["discoverable-1", "discoverable-2"];
    for plugin_name in &plugins {
        let mock_plugin = Arc::new(MockPlugin::new(plugin_name));
        manager
            .register_plugin_for_test(plugin_name.to_string(), mock_plugin)
            .await
            .unwrap();
    }

    manager.initialize().await.unwrap();

    // 测试发现功能
    let discovered_count = manager.discover_plugins().await;
    assert!(discovered_count.is_ok());

    // 验证插件列表
    let all_plugins = manager.list_plugins().await;
    for plugin_name in &plugins {
        assert!(all_plugins.contains(&plugin_name.to_string()));
    }
}

#[tokio::test]
async fn test_config_save_and_load() {
    let temp_file = "test-config.json";

    // 创建配置并保存
    let mut config = ProjectConfig::default_for_project("test-save-load", ".");
    let mut plugin_config = PluginConfig::new("test-save-plugin");
    plugin_config.enabled = true;
    plugin_config.set_version("1.5.0");
    config.add_plugin(plugin_config);

    let manager = PluginManager::from_project_config(config).await.unwrap();
    manager.save_config(temp_file).await.unwrap();

    // 加载配置并验证
    let loaded_config = ProjectConfig::load(temp_file).await.unwrap();
    let loaded_plugins = loaded_config.get_plugins();

    assert!(loaded_plugins.contains_key("test-save-plugin"));
    let loaded_plugin = &loaded_plugins["test-save-plugin"];
    assert!(loaded_plugin.enabled);
    assert_eq!(loaded_plugin.get_version(), Some("1.5.0"));

    // 清理测试文件
    let _ = std::fs::remove_file(temp_file);
}

#[tokio::test]
async fn test_plugin_lifecycle() {
    let config = ProjectConfig::default_for_project("test-lifecycle", ".");
    let mut manager = PluginManager::from_project_config(config).await.unwrap();

    // 注册插件
    let mock_plugin = Arc::new(MockPlugin::new("lifecycle-test"));
    manager
        .register_plugin_for_test("lifecycle-test".to_string(), mock_plugin)
        .await
        .unwrap();

    // 测试完整生命周期
    manager.initialize().await.unwrap();

    let options = InstallOptions::new();
    let install_result = manager
        .install_plugin("lifecycle-test", Some("1.0.0"), &options)
        .await;
    assert!(install_result.is_ok());

    // 模拟更新操作 - 在实际实现中这应该是一个更新方法
    let plugin_result = manager.get_plugin("lifecycle-test").await;
    assert!(plugin_result.is_ok());

    if let Ok(plugin) = plugin_result {
        let update_result = plugin.update(Some("1.1.0")).await;
        assert!(update_result.is_ok());
    }

    let uninstall_result = manager.uninstall_plugin("lifecycle-test", "1.0.0").await;
    assert!(uninstall_result.is_ok());

    manager.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_error_handling() {
    let config = ProjectConfig::default_for_project("test-errors", ".");
    let mut manager = PluginManager::from_project_config(config).await.unwrap();

    manager.initialize().await.unwrap();

    // 测试安装不存在的插件
    let options = InstallOptions::new();
    let result = manager
        .install_plugin("non-existent-plugin", Some("1.0.0"), &options)
        .await;
    assert!(result.is_err());

    // 测试获取不存在的插件
    let result = manager.get_plugin("non-existent-plugin").await;
    assert!(result.is_err());
}
