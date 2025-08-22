//! 基础使用示例
//! 
//! 这个示例展示了如何使用 PLM 进行基本的插件管理操作

use plm::{init_default, quick_setup, PluginManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PLM 基础使用示例");
    
    // 1. 快速设置项目配置
    println!("\n📋 步骤 1: 设置项目配置");
    quick_setup("example-project", ".").await?;
    println!("✅ 项目配置创建完成");
    
    // 2. 初始化插件管理器
    println!("\n🔧 步骤 2: 初始化插件管理器");
    let mut manager = init_default().await?;
    println!("✅ 插件管理器初始化完成");
    
    // 3. 初始化管理器
    println!("\n⚡ 步骤 3: 启动插件管理器");
    manager.initialize().await?;
    println!("✅ 插件管理器启动完成");
    
    // 4. 发现插件
    println!("\n🔍 步骤 4: 发现插件");
    let plugin_count = manager.discover_plugins().await?;
    println!("📦 发现 {} 个插件", plugin_count);
    
    // 5. 列出所有插件
    println!("\n📋 步骤 5: 列出所有插件");
    let plugins = manager.list_plugins().await;
    if plugins.is_empty() {
        println!("📭 当前没有注册的插件");
    } else {
        println!("📦 已注册的插件:");
        for plugin in &plugins {
            println!("  - {}", plugin);
        }
    }
    
    // 6. 验证所有插件
    println!("\n✅ 步骤 6: 验证插件");
    let summary = manager.validate_all_plugins().await?;
    println!("📊 验证结果:");
    println!("  ✅ 有效插件: {}", summary.valid_plugins);
    println!("  ❌ 无效插件: {}", summary.invalid_plugins);
    if !summary.errors.is_empty() {
        println!("  🚨 错误信息:");
        for error in &summary.errors {
            println!("    - {}", error);
        }
    }
    
    // 7. 保存配置
    println!("\n💾 步骤 7: 保存配置");
    manager.save_config("./plm_example.json").await?;
    println!("✅ 配置已保存到 plm_example.json");
    
    // 8. 关闭管理器
    println!("\n🔚 步骤 8: 关闭插件管理器");
    manager.shutdown().await?;
    println!("✅ 插件管理器已关闭");
    
    println!("\n🎉 基础使用示例完成！");
    
    Ok(())
}