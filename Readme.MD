# PLM - Plugin Lifecycle Manager

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/plm.svg)](https://crates.io/crates/plm)

PLM (Plugin Lifecycle Manager) 是一个强大的 Rust 插件生命周期管理系统，可以轻松集成到任何 Rust 项目中，提供完整的插件管理功能。

## ✨ 特性

- 🔌 **插件生命周期管理** - 完整的插件注册、初始化、安装、卸载流程
- ⚙️ **灵活配置系统** - 支持 JSON 配置文件和代码配置
- 🚀 **异步支持** - 基于 Tokio 的异步插件操作
- 🛡️ **类型安全** - 强类型插件接口和错误处理
- 📦 **易于集成** - 简单的 API 设计，快速集成到现有项目
- 🔍 **插件发现** - 自动发现和验证插件
- 📊 **状态管理** - 插件状态跟踪和健康检查

## 🚀 快速开始

### 安装

将 PLM 添加到您的 `Cargo.toml`:

```toml
[dependencies]
plm = "0.1.0"
```

### 基础使用

```rust
use plm::{PluginManager, init_default, quick_setup};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 方式1: 使用默认配置初始化
    let mut manager = init_default().await?;

    // 方式2: 快速设置项目配置
    quick_setup("my-project", ".").await?;

    // 初始化插件管理器
    manager.initialize().await?;

    // 发现插件
    let plugin_count = manager.discover_plugins().await?;
    println!("发现 {} 个插件", plugin_count);

    // 验证所有插件
    let summary = manager.validate_all_plugins().await?;
    println!("有效插件: {}, 无效插件: {}", summary.valid_plugins, summary.invalid_plugins);

    // 关闭管理器
    manager.shutdown().await?;

    Ok(())
}
```

## 📖 详细使用指南

### 1. 项目配置

PLM 使用 JSON 配置文件来管理项目和插件设置：

```json
{
  "project": {
    "name": "my-project",
    "version": "1.0.0",
    "root_path": "."
  },
  "plugins": [
    {
      "name": "example-plugin",
      "version": "1.0.0",
      "enabled": true,
      "config": {
        "setting1": "value1",
        "setting2": "value2"
      }
    }
  ]
}
```

### 2. 自定义插件开发

实现 `Plugin` trait 来创建自定义插件：

```rust
use plm::{Plugin, PluginError, PluginMetadata, InstallOptions};
use async_trait::async_trait;

pub struct MyPlugin {
    name: String,
    initialized: bool,
}

#[async_trait]
impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: self.name.clone(),
            version: "1.0.0".to_string(),
            description: "我的自定义插件".to_string(),
            author: "Your Name".to_string(),
        }
    }

    async fn initialize(&mut self) -> Result<(), PluginError> {
        println!("初始化插件: {}", self.name);
        self.initialized = true;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), PluginError> {
        println!("关闭插件: {}", self.name);
        self.initialized = false;
        Ok(())
    }

    async fn install(&self, version: &str, options: &InstallOptions) -> Result<String, PluginError> {
        println!("安装插件 {} 版本 {}", self.name, version);
        Ok(format!("插件 {} 安装成功", self.name))
    }

    async fn uninstall(&self, version: &str) -> Result<(), PluginError> {
        println!("卸载插件 {} 版本 {}", self.name, version);
        Ok(())
    }
}
```

### 3. 插件管理操作

```rust
use plm::{PluginManager, InstallOptions};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new().await?;

    // 注册插件（用于测试）
    let plugin = Arc::new(MyPlugin {
        name: "test-plugin".to_string(),
        initialized: false,
    });
    manager.register_plugin_for_test("test-plugin".to_string(), plugin).await?;

    // 初始化所有插件
    manager.initialize().await?;

    // 列出所有插件
    let plugins = manager.list_plugins().await;
    println!("已注册插件: {:?}", plugins);

    // 安装插件
    let options = InstallOptions {
        force: false,
        dry_run: false,
        verbose: true,
    };
    let result = manager.install_plugin("test-plugin", Some("1.0.0"), &options).await?;
    println!("安装结果: {}", result);

    // 卸载插件
    manager.uninstall_plugin("test-plugin", "1.0.0").await?;

    // 保存配置
    manager.save_config("./plm.json").await?;

    // 关闭管理器
    manager.shutdown().await?;

    Ok(())
}
```

### 4. 配置管理

```rust
use plm::{ProjectConfig, PluginConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从文件加载配置
    let config = ProjectConfig::load_from_file("plm.json").await?;
    let mut manager = PluginManager::from_project_config(config).await?;

    // 添加插件配置
    let plugin_config = PluginConfig {
        name: "new-plugin".to_string(),
        version: "1.0.0".to_string(),
        enabled: true,
        config: serde_json::json!({
            "timeout": 30,
            "retries": 3
        }),
    };
    manager.add_plugin_config(plugin_config);

    // 获取插件配置
    if let Some(config) = manager.get_plugin_config("new-plugin") {
        println!("插件配置: {:?}", config);
    }

    // 移除插件配置
    manager.remove_plugin_config("old-plugin");

    // 保存更新后的配置
    manager.save_config("plm.json").await?;

    Ok(())
}
```

## 🏗️ 项目结构

```
plugin-manager/
├── src/
│   ├── lib.rs          # 库入口和公共 API
│   ├── main.rs         # CLI 工具入口
│   ├── core.rs         # 核心插件管理器实现
│   ├── config.rs       # 配置管理
│   └── traits.rs       # 插件 trait 定义
├── examples/
│   ├── basic/
│   │   └── simple_usage.rs     # 基础使用示例
│   ├── advanced/
│   │   └── custom_plugin.rs    # 自定义插件示例
│   └── integration_example.rs  # 集成示例
├── tests/
│   ├── simple_test.rs          # 基础测试
│   └── integration_test.rs     # 集成测试
├── docs/                       # 文档目录
├── scripts/
│   └── build.sh               # 构建脚本
└── README.md
```

## 🔧 CLI 工具

PLM 还提供了命令行工具用于插件管理：

```bash
# 初始化项目配置
plm init --name my-project --path .

# 发现插件
plm discover

# 验证插件
plm validate

# 安装插件
plm install plugin-name --version 1.0.0

# 卸载插件
plm uninstall plugin-name --version 1.0.0

# 列出插件
plm list

# 显示插件信息
plm info plugin-name
```

## 📚 示例代码

查看 `examples/` 目录获取更多使用示例：

- [`examples/basic/simple_usage.rs`](examples/basic/simple_usage.rs) - 基础使用示例
- [`examples/advanced/custom_plugin.rs`](examples/advanced/custom_plugin.rs) - 自定义插件开发
- [`examples/integration_example.rs`](examples/integration_example.rs) - 项目集成示例

## 🧪 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test simple_test

# 运行集成测试
cargo test --test integration_test
```

## 🔨 构建项目

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 使用构建脚本
./scripts/build.sh
```

## 📋 API 参考

### 核心类型

- `PluginManager` - 主要的插件管理器
- `Plugin` - 插件 trait 接口
- `PluginConfig` - 插件配置结构
- `ProjectConfig` - 项目配置结构
- `PluginError` - 错误类型定义

### 主要方法

- `init_default()` - 使用默认配置初始化
- `init_from_config(path)` - 从配置文件初始化
- `quick_setup(name, path)` - 快速项目设置
- `register_plugin_for_test()` - 注册测试插件
- `install_plugin()` - 安装插件
- `uninstall_plugin()` - 卸载插件
- `discover_plugins()` - 发现插件
- `validate_all_plugins()` - 验证所有插件

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 Apache 2.0 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🆘 支持

如果您遇到问题或有疑问：

- 查看 [Issues](https://github.com/plm/plm/issues) 页面
- 创建新的 Issue 描述您的问题
- 查看示例代码和文档

## 🗺️ 路线图

- [ ] 插件热重载支持
- [ ] 插件依赖管理
- [ ] Web UI 管理界面
- [ ] 插件市场集成
- [ ] 更多插件模板
- [ ] 性能优化和监控

---

**PLM** - 让插件管理变得简单高效！ 🚀
