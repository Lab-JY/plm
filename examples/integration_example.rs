//! 集成示例：如何在你的项目中使用 PLM

use async_trait::async_trait;
use plm::config::PluginSource;
use plm::traits::{InstallOptions, Plugin, PluginError, PluginMetadata, PluginStatus};
use plm::{PluginConfig, PluginManager, ProjectConfig};
use std::collections::HashMap;

/// 示例：自定义插件实现
pub struct CustomToolPlugin {
    metadata: PluginMetadata,
    config: HashMap<String, String>,
}

impl CustomToolPlugin {
    pub fn new() -> Self {
        let metadata = PluginMetadata {
            name: "custom-tool".to_string(),
            version: "1.0.0".to_string(),
            description: "A custom tool plugin example".to_string(),
            author: "Your Company".to_string(),
            homepage: Some("https://your-company.com".to_string()),
            repository: Some("https://github.com/your-company/custom-tool".to_string()),
            supported_platforms: vec![
                "linux".to_string(),
                "macos".to_string(),
                "windows".to_string(),
            ],
            tags: vec!["development".to_string(), "custom".to_string()],
            dependencies: vec![],
            min_plm_version: Some("0.1.0".to_string()),
        };

        Self {
            metadata,
            config: HashMap::new(),
        }
    }
}

#[async_trait]
impl Plugin for CustomToolPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn status(&self) -> PluginStatus {
        PluginStatus::Active
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        println!("Initializing custom tool plugin...");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), PluginError> {
        println!("Shutting down custom tool plugin...");
        Ok(())
    }

    async fn install(
        &self,
        version: &str,
        options: &InstallOptions,
    ) -> Result<String, PluginError> {
        if !options.quiet {
            println!("Installing custom tool version {}...", version);
        }

        // 模拟安装过程
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let install_path = format!("/usr/local/bin/custom-tool-{}", version);

        if !options.quiet {
            println!("Custom tool {} installed to {}", version, install_path);
        }

        Ok(install_path)
    }

    async fn uninstall(&self, version: &str) -> Result<(), PluginError> {
        println!("Uninstalling custom tool version {}...", version);
        // 模拟卸载过程
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Custom tool {} uninstalled", version);
        Ok(())
    }

    async fn list_versions(&self) -> Result<Vec<plm::traits::VersionInfo>, PluginError> {
        Ok(vec![
            plm::traits::VersionInfo::new("1.0.0", "linux-x64", "https://example.com/v1.0.0"),
            plm::traits::VersionInfo::new("1.1.0", "linux-x64", "https://example.com/v1.1.0"),
            plm::traits::VersionInfo::new("2.0.0", "linux-x64", "https://example.com/v2.0.0"),
        ])
    }

    async fn list_installed(&self) -> Result<Vec<String>, PluginError> {
        Ok(vec!["1.0.0".to_string(), "1.1.0".to_string()])
    }

    async fn is_installed(&self, version: &str) -> Result<bool, PluginError> {
        Ok(matches!(version, "1.0.0" | "1.1.0"))
    }

    async fn get_latest_version(&self) -> Result<plm::traits::VersionInfo, PluginError> {
        Ok(plm::traits::VersionInfo::new(
            "2.0.0",
            "linux-x64",
            "https://example.com/v2.0.0",
        ))
    }

    async fn update(&self, version: Option<&str>) -> Result<String, PluginError> {
        let target_version = version.unwrap_or("2.0.0");
        println!("Updating custom tool to version {}...", target_version);
        Ok(target_version.to_string())
    }

    async fn switch_version(&self, version: &str) -> Result<(), PluginError> {
        println!("Switching custom tool to version {}...", version);
        Ok(())
    }

    async fn verify_installation(&self, version: &str) -> Result<bool, PluginError> {
        println!("Verifying custom tool version {}...", version);
        Ok(true)
    }

    async fn cleanup(&self) -> Result<(), PluginError> {
        println!("Cleaning up custom tool cache...");
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
        println!("Setting {} = {}", key, value);
        Ok(())
    }

    async fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, PluginError> {
        println!("Executing command: {} {:?}", command, args);
        Ok("Command executed successfully".to_string())
    }

    fn get_help(&self) -> String {
        "Custom Tool Plugin - A demonstration plugin for Plugin Manager".to_string()
    }

    fn supports_feature(&self, feature: &str) -> bool {
        matches!(feature, "install" | "uninstall" | "update" | "config")
    }
}

/// 示例：在你的应用中集成 Plugin Manager
pub struct MyApplication {
    plugin_manager: PluginManager,
}

impl MyApplication {
    /// 创建新的应用实例
    pub async fn new(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 从配置文件加载插件管理器
        let mut plugin_manager = plm::init_from_config(config_path).await?;

        // 初始化插件管理器
        plugin_manager.initialize().await?;

        Ok(Self { plugin_manager })
    }

    /// 创建带有自定义配置的应用实例
    pub async fn new_with_custom_config() -> Result<Self, Box<dyn std::error::Error>> {
        // 创建自定义项目配置
        let mut config = ProjectConfig::default_for_project("my-app", ".");

        // 添加自定义插件配置
        let mut custom_plugin_config = PluginConfig::new("custom-tool");
        custom_plugin_config.enabled = true;
        custom_plugin_config.set_version("1.0.0");
        custom_plugin_config.set_source(PluginSource::local("./plugins/custom-tool"));
        custom_plugin_config.set_setting("debug_mode", serde_json::Value::Bool(true));

        config.add_plugin(custom_plugin_config);

        // 从配置创建插件管理器
        let mut plugin_manager = PluginManager::from_project_config(config).await?;
        plugin_manager.initialize().await?;

        Ok(Self { plugin_manager })
    }

    /// 安装工具
    pub async fn install_tool(
        &mut self,
        name: &str,
        version: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let options = InstallOptions::new();
        let install_path = self
            .plugin_manager
            .install_plugin(name, version, &options)
            .await?;
        println!("✅ {} installed to {}", name, install_path);
        Ok(())
    }

    /// 列出所有工具
    pub async fn list_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        let plugins = self.plugin_manager.list_plugins().await;

        if plugins.is_empty() {
            println!("No tools installed");
            return Ok(());
        }

        println!("Installed tools:");
        for plugin_name in plugins {
            let plugin = self.plugin_manager.get_plugin(&plugin_name).await?;
            let metadata = plugin.metadata();
            println!(
                "  - {} ({}): {}",
                metadata.name, metadata.version, metadata.description
            );
        }

        Ok(())
    }

    /// 配置工具
    pub async fn configure_tool(
        &mut self,
        name: &str,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 直接通过插件管理器设置配置
        if let Ok(plugin) = self.plugin_manager.get_plugin(name).await {
            plugin.set_config_value(key, value).await?;
            println!("✅ Set {} {} = {}", name, key, value);
        } else {
            println!("⚠️  Plugin '{}' not found", name);
        }

        Ok(())
    }

    /// 验证所有工具
    pub async fn validate_tools(&self) -> Result<(), Box<dyn std::error::Error>> {
        let summary = self.plugin_manager.validate_all_plugins().await?;

        println!("📊 Validation Summary:");
        println!("  Valid tools: {}", summary.valid_plugins);
        println!("  Invalid tools: {}", summary.invalid_plugins);

        if !summary.errors.is_empty() {
            println!("  Errors:");
            for error in &summary.errors {
                println!("    - {}", error);
            }
        }

        Ok(())
    }

    /// 发现新工具
    pub async fn discover_tools(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let count = self.plugin_manager.discover_plugins().await?;
        if count > 0 {
            println!("✅ Discovered {} new tools", count);
        } else {
            println!("ℹ️  No new tools found");
        }
        Ok(())
    }

    /// 保存配置
    pub async fn save_config(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.plugin_manager.save_config(path).await?;
        println!("✅ Configuration saved to {}", path);
        Ok(())
    }

    /// 关闭应用
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.plugin_manager.shutdown().await?;
        println!("✅ Application shutdown complete");
        Ok(())
    }
}

/// 示例主函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 PLM Integration Example");

    // 方式1: 从配置文件创建应用
    println!("\n📁 Creating app from config file...");
    match MyApplication::new("plm.json").await {
        Ok(mut app) => {
            app.list_tools().await?;
            app.shutdown().await?;
        }
        Err(e) => {
            println!("⚠️  Failed to load from config file: {}", e);
            println!("   This is expected if plugin-manager.json doesn't exist");
        }
    }

    // 方式2: 使用自定义配置创建应用
    println!("\n⚙️  Creating app with custom config...");
    let mut app = MyApplication::new_with_custom_config().await?;

    // 演示各种功能
    println!("\n📦 Installing tools...");
    // 注意：这里会失败，因为我们没有实际的插件实现
    // 在真实场景中，你需要注册你的插件工厂或加载器

    println!("\n📋 Listing tools...");
    app.list_tools().await?;

    println!("\n⚙️  Configuring tools...");
    app.configure_tool("custom-tool", "debug_mode", "true")
        .await?;

    println!("\n🔍 Discovering tools...");
    app.discover_tools().await?;

    println!("\n✅ Validating tools...");
    app.validate_tools().await?;

    println!("\n💾 Saving configuration...");
    app.save_config("example-config.json").await?;

    println!("\n🔄 Shutting down...");
    app.shutdown().await?;

    println!("\n✨ Example completed successfully!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_custom_plugin() {
        let mut plugin = CustomToolPlugin::new();

        // 测试初始化
        assert!(plugin.initialize().await.is_ok());

        // 测试元数据
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "custom-tool");
        assert_eq!(metadata.version, "1.0.0");

        // 测试安装
        let options = InstallOptions::new().quiet();
        let install_path = plugin.install("1.0.0", &options).await.unwrap();
        assert!(install_path.contains("custom-tool-1.0.0"));

        // 测试版本列表
        let versions = plugin.list_versions().await.unwrap();
        assert!(!versions.is_empty());

        // 测试配置
        assert!(plugin
            .set_config_value("test_key", "test_value")
            .await
            .is_ok());

        // 测试关闭
        assert!(plugin.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_application_creation() {
        // 测试自定义配置创建
        let result = MyApplication::new_with_custom_config().await;
        assert!(result.is_ok());
    }
}
