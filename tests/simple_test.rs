//! PLM 简单单元测试

use plm::{ProjectConfig, PluginConfig};
use plm::config::PluginSource;

#[test]
fn test_project_config_creation() {
    let config = ProjectConfig::default_for_project("test-project", "/tmp");
    
    assert_eq!(config.get_project_name(), "test-project");
    assert_eq!(config.get_project_root(), "/tmp");
    assert!(config.get_plugins().is_empty());
}

#[test]
fn test_plugin_config_creation() {
    let mut plugin_config = PluginConfig::new("test-plugin");
    
    assert_eq!(plugin_config.name, "test-plugin");
    assert!(!plugin_config.enabled);
    assert!(plugin_config.get_version().is_none());
    
    // 测试设置版本
    plugin_config.set_version("1.2.3");
    assert_eq!(plugin_config.get_version(), Some("1.2.3"));
    
    // 测试启用插件
    plugin_config.enabled = true;
    assert!(plugin_config.enabled);
}

#[test]
fn test_plugin_source_types() {
    // 测试本地源
    let local_source = PluginSource::local("/path/to/plugin");
    assert_eq!(local_source.get_url(), "/path/to/plugin");
    assert_eq!(local_source.get_type_name(), "local");
    
    // 测试注册表源
    let registry_source = PluginSource::registry("https://registry.example.com");
    assert_eq!(registry_source.get_url(), "https://registry.example.com");
    assert_eq!(registry_source.get_type_name(), "registry");
    
    // 测试 Git 源
    let git_source = PluginSource::git("https://github.com/user/repo.git", Some("main"));
    assert_eq!(git_source.get_url(), "https://github.com/user/repo.git");
    assert_eq!(git_source.get_type_name(), "git");
}

#[test]
fn test_plugin_config_settings() {
    let mut plugin_config = PluginConfig::new("test-plugin");
    
    // 测试设置配置项
    plugin_config.set_setting("debug", serde_json::Value::Bool(true));
    plugin_config.set_setting("timeout", serde_json::Value::Number(serde_json::Number::from(30)));
    plugin_config.set_setting("name", serde_json::Value::String("test".to_string()));
    
    // 测试获取配置项
    assert_eq!(plugin_config.get_setting("debug"), Some(&serde_json::Value::Bool(true)));
    assert_eq!(plugin_config.get_setting("timeout"), Some(&serde_json::Value::Number(serde_json::Number::from(30))));
    assert_eq!(plugin_config.get_setting("name"), Some(&serde_json::Value::String("test".to_string())));
    assert_eq!(plugin_config.get_setting("nonexistent"), None);
}

#[test]
fn test_project_config_plugin_management() {
    let mut config = ProjectConfig::default_for_project("test-project", ".");
    
    // 添加插件配置
    let mut plugin_config = PluginConfig::new("node");
    plugin_config.enabled = true;
    plugin_config.set_version("18.17.0");
    plugin_config.set_source(PluginSource::registry("https://nodejs.org/dist"));
    
    config.add_plugin(plugin_config);
    
    // 验证插件已添加
    let plugins = config.get_plugins();
    assert!(plugins.contains_key("node"));
    
    let node_config = &plugins["node"];
    assert!(node_config.enabled);
    assert_eq!(node_config.get_version(), Some("18.17.0"));
    
    // 测试更新插件设置
    config.update_plugin_setting("node", "registry", serde_json::Value::String("https://registry.npmjs.org".to_string())).unwrap();
    
    let updated_plugins = config.get_plugins();
    let updated_node_config = &updated_plugins["node"];
    assert_eq!(
        updated_node_config.get_setting("registry"), 
        Some(&serde_json::Value::String("https://registry.npmjs.org".to_string()))
    );
}

#[test]
fn test_config_serialization() {
    let mut config = ProjectConfig::default_for_project("serialization-test", "/tmp/test");
    
    let mut plugin_config = PluginConfig::new("test-plugin");
    plugin_config.enabled = true;
    plugin_config.set_version("1.0.0");
    plugin_config.set_source(PluginSource::local("/usr/local/test-plugin"));
    plugin_config.set_setting("feature_x", serde_json::Value::Bool(true));
    
    config.add_plugin(plugin_config);
    
    // 测试序列化
    let json_str = serde_json::to_string_pretty(&config).unwrap();
    assert!(json_str.contains("serialization-test"));
    assert!(json_str.contains("test-plugin"));
    assert!(json_str.contains("1.0.0"));
    
    // 测试反序列化
    let deserialized_config: ProjectConfig = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized_config.get_project_name(), "serialization-test");
    assert_eq!(deserialized_config.get_project_root(), "/tmp/test");
    
    let plugins = deserialized_config.get_plugins();
    assert!(plugins.contains_key("test-plugin"));
    
    let test_plugin = &plugins["test-plugin"];
    assert!(test_plugin.enabled);
    assert_eq!(test_plugin.get_version(), Some("1.0.0"));
    assert_eq!(test_plugin.get_setting("feature_x"), Some(&serde_json::Value::Bool(true)));
}