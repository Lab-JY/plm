//! PLM 基础使用示例

use plm::{init_from_config, quick_setup};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 PLM 基础使用示例");

    // 1. 快速设置项目
    println!("\n📁 初始化项目配置...");
    quick_setup("my-project", ".").await?;

    // 2. 从配置文件加载 PLM
    println!("\n⚙️ 加载 PLM...");
    let mut manager = init_from_config("plm.json").await?;
    
    // 3. 初始化
    println!("\n🔧 初始化 PLM...");
    manager.initialize().await?;

    // 4. 列出可用插件
    println!("\n📋 列出插件...");
    let plugins = manager.list_plugins().await;
    println!("发现 {} 个插件", plugins.len());
    
    for plugin_name in &plugins {
        println!("  - {}", plugin_name);
    }

    // 5. 发现新插件
    println!("\n🔍 发现插件...");
    let discovered = manager.discover_plugins().await?;
    println!("发现 {} 个新插件", discovered);

    // 6. 验证插件
    println!("\n✅ 验证插件...");
    let summary = manager.validate_all_plugins().await?;
    println!("验证结果: {} 个有效, {} 个无效", summary.valid_plugins, summary.invalid_plugins);

    // 7. 保存配置
    println!("\n💾 保存配置...");
    manager.save_config("plm.json").await?;

    // 8. 关闭
    println!("\n🔄 关闭 PLM...");
    manager.shutdown().await?;

    println!("\n✨ 示例完成!");
    Ok(())
}