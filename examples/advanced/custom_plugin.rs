//! PLM 高级使用示例 - 自定义插件

use plm::{PluginManager, ProjectConfig, PluginConfig};
use plm::config::PluginSource;
use plm::traits::{Plugin, PluginMetadata, PluginError, InstallOptions, VersionInfo, PluginStatus};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;

/// 自定义工具插件
pub struct CustomToolPlugin {
    metadata: PluginMetadata,
    config: HashMap<String, String>,
    status: PluginStatus,
}

impl CustomToolPlugin {
    pub fn new() -> Self {
        let metadata = PluginMetadata {
            name: "custom-tool".to_string(),
            version: "1.0.0".to_string(),
            description: "一个自定义工具插件示例".to_string(),
            author: "PLM Team".to_string(),
            homepage: Some("https://plm.dev".to_string()),
            repository: Some("https://github.com/plm/custom-tool".to_string()),
            supported_platforms: vec!["linux".to_string(), "macos".to_string(), "windows".to_string()],
            tags: vec!["development".to_string(), "custom".to_string()],
            dependencies: vec![],
            min_plm_version: Some("0.1.0".to_string()),
        };

        Self {
            metadata,
            config: HashMap::new(),
            status: PluginStatus::Inactive,
        }
    }
}

#[async_trait]
impl Plugin for CustomToolPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn status(&self) -> PluginStatus {
        self.status.clone()
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        println!("🔧 初始化自定义工具插件...");
        self.status = PluginStatus::Active;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), PluginError> {
        println!("🔄 关闭自定义工具插件...");
        self.status = PluginStatus::Inactive;
        Ok(())
    }

    async fn install(&self, version: &str, options: &InstallOptions) -> Result<String, PluginError> {
        if !options.quiet {
            println!("📦 安装自定义工具 v{}...", version);
        }
        
        // 模拟安装过程
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        let install_path = format!("/usr/local/bin/custom-tool-{}", version);
        
        if !options.quiet {
            println!("✅ 自定义工具 v{} 已安装到 {}", version, install_path);
        }
        
        Ok(install_path)
    }

    async fn uninstall(&self, version: &str) -> Result<(), PluginError> {
        println!("🗑️ 卸载自定义工具 v{}...", version);
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        println!("✅ 自定义工具 v{} 已卸载", version);
        Ok(())
    }

    async fn list_versions(&self) -> Result<Vec<VersionInfo>, PluginError> {
        Ok(vec![
            VersionInfo::new("1.0.0", "linux-x64", "https://releases.example.com/v1.0.0"),
            VersionInfo::new("1.1.0", "linux-x64", "https://releases.example.com/v1.1.0"),
            VersionInfo::new("2.0.0", "linux-x64", "https://releases.example.com/v2.0.0"),
        ])
    }

    async fn list_installed(&self) -> Result<Vec<String>, PluginError> {
        Ok(vec!["1.0.0".to_string()])
    }

    async fn is_installed(&self, version: &str) -> Result<bool, PluginError> {
        Ok(version == "1.0.0")
    }

    async fn get_latest_version(&self) -> Result<VersionInfo, PluginError> {
        Ok(VersionInfo::new("2.0.0", "linux-x64", "https://releases.example.com/v2.0.0"))
    }

    async fn update(&self, version: Option<&str>) -> Result<String, PluginError> {
        let target_version = version.unwrap_or("2.0.0");
        println!("🔄 更新自定义工具到 v{}...", target_version);
        Ok(target_version.to_string())
    }

    async fn switch_version(&self, version: &str) -> Result<(), PluginError> {
        println!("🔀 切换自定义工具到 v{}...", version);
        Ok(())
    }

    async fn verify_installation(&self, version: &str) -> Result<bool, PluginError> {
        println!("🔍 验证自定义工具 v{} 安装...", version);
        Ok(true)
    }

    async fn cleanup(&self) -> Result<(), PluginError> {
        println!("🧹 清理自定义工具缓存...");
        Ok(())
    }

    async fn get_config(&self) -> Result<HashMap<String, String>, PluginError> {
        Ok(self.config.clone())
    }

    async fn set_config(&self, _config: HashMap<String, String>) -> Result<(), PluginError> {
        Ok(())
    }

    async fn get_config_value(&self, key: &str) -> Result<Option<String>, PluginError> {
        Ok(self.config.get(key).cloned())
    }

    async fn set_config_value(&self, key: &str, value: &str) -> Result<(), PluginError> {
        println!("⚙️ 设置配置: {} = {}", key, value);
        Ok(())
    }

    async fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, PluginError> {
        println!("🚀 执行命令: {} {:?}", command, args);
        Ok("命令执行成功".to_string())
    }

    fn get_help(&self) -> String {
        "自定义工具插件 - PLM 插件开发示例".to_string()
    }

    fn supports_feature(&self, feature: &str) -> bool {
        matches!(feature, "install" | "uninstall" | "update" | "config" | "execute")
    }
}

/// 高级 PLM 使用示例
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 PLM 高级使用示例 - 自定义插件");

    // 1. 创建自定义项目配置
    println!("\n📋 创建自定义配置...");
    let mut config = ProjectConfig::default_for_project("advanced-project", ".");
    
    // 2. 添加自定义插件配置
    let mut custom_plugin_config = PluginConfig::new("custom-tool");
    custom_plugin_config.enabled = true;
    custom_plugin_config.set_version("1.0.0");
    custom_plugin_config.set_source(PluginSource::local("./plugins/custom-tool"));
    custom_plugin_config.set_setting("debug_mode", serde_json::Value::Bool(true));
    custom_plugin_config.set_setting("max_connections", serde_json::Value::Number(serde_json::Number::from(100)));
    
    config.add_plugin(custom_plugin_config);

    // 3. 创建插件管理器
    println!("\n⚙️ 创建插件管理器...");
    let mut manager = PluginManager::from_project_config(config).await?;

    // 4. 注册自定义插件
    println!("\n🔌 注册自定义插件...");
    let custom_plugin = Arc::new(CustomToolPlugin::new());
    manager.register_plugin_for_test("custom-tool".to_string(), custom_plugin).await?;

    // 5. 初始化
    println!("\n🔧 初始化插件管理器...");
    manager.initialize().await?;

    // 6. 安装插件
    println!("\n📦 安装插件...");
    let options = plm::traits::InstallOptions::new();
    match manager.install_plugin("custom-tool", Some("1.0.0"), &options).await {
        Ok(path) => println!("✅ 插件已安装到: {}", path),
        Err(e) => println!("❌ 安装失败: {}", e),
    }

    // 7. 列出插件
    println!("\n📋 列出插件...");
    let plugins = manager.list_plugins().await;
    for plugin_name in &plugins {
        if let Ok(plugin) = manager.get_plugin(plugin_name).await {
            let metadata = plugin.metadata();
            println!("  📦 {} v{} - {}", metadata.name, metadata.version, metadata.description);
            println!("     作者: {}", metadata.author);
            println!("     状态: {:?}", plugin.status());
        }
    }

    // 8. 配置插件
    println!("\n⚙️ 配置插件...");
    manager.get_config_mut().update_plugin_setting(
        "custom-tool", 
        "log_level", 
        serde_json::Value::String("debug".to_string())
    )?;

    // 9. 验证插件
    println!("\n✅ 验证插件...");
    let summary = manager.validate_all_plugins().await?;
    println!("验证结果:");
    println!("  ✅ 有效插件: {}", summary.valid_plugins);
    println!("  ❌ 无效插件: {}", summary.invalid_plugins);
    
    if !summary.errors.is_empty() {
        println!("  错误:");
        for error in &summary.errors {
            println!("    - {}", error);
        }
    }

    // 10. 发现插件
    println!("\n🔍 发现新插件...");
    let discovered = manager.discover_plugins().await?;
    println!("发现 {} 个新插件", discovered);

    // 11. 保存配置
    println!("\n💾 保存配置...");
    manager.save_config("advanced-plm.json").await?;

    // 12. 关闭
    println!("\n🔄 关闭插件管理器...");
    manager.shutdown().await?;

    println!("\n✨ 高级示例完成!");
    Ok(())
}