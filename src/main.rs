//! PLM CLI - Plugin Lifecycle Manager

use clap::{Parser, Subcommand};
use colored::Colorize;
use plm::{init_from_config, quick_setup};

#[derive(Parser)]
#[command(name = "plm")]
#[command(about = "Plugin Lifecycle Manager")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Configuration file path
    #[arg(short, long, default_value = "plm.json")]
    config: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize PLM in current project
    Init {
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
        /// Project root directory
        #[arg(short, long, default_value = ".")]
        root: String,
    },
    /// Install a plugin
    Install {
        /// Plugin name
        name: String,
        /// Plugin version
        #[arg(short, long)]
        version: Option<String>,
        /// Force installation
        #[arg(short, long)]
        force: bool,
    },
    /// Uninstall a plugin
    Uninstall {
        /// Plugin name
        name: String,
        /// Plugin version
        version: String,
    },
    /// List plugins
    List {
        /// Show only installed plugins
        #[arg(short, long)]
        installed: bool,
    },
    /// Show plugin information
    Info {
        /// Plugin name
        name: String,
    },
    /// Discover available plugins
    Discover,
    /// Validate plugins
    Validate {
        /// Plugin name (validate all if not specified)
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Configure plugin settings
    Config {
        /// Plugin name
        name: String,
        /// Setting key
        key: Option<String>,
        /// Setting value
        value: Option<String>,
    },
    /// Export configuration
    Export {
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
    /// Import configuration
    Import {
        /// Input file path
        #[arg(short, long)]
        input: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    match cli.command {
        Commands::Init { name, root } => {
            let project_name = name.unwrap_or_else(|| {
                std::env::current_dir()
                    .ok()
                    .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                    .unwrap_or_else(|| "my-project".to_string())
            });

            quick_setup(&project_name, &root).await?;
            println!("✅ PLM 已初始化完成");
        }

        Commands::Install {
            name,
            version,
            force,
        } => {
            let mut manager = init_from_config(&cli.config).await?;
            manager.initialize().await?;

            let mut options = plm::traits::InstallOptions::new();
            if force {
                options = options.force();
            }
            if !cli.verbose {
                options = options.quiet();
            }

            let install_path = manager
                .install_plugin(&name, version.as_deref(), &options)
                .await?;
            println!("✅ {} installed to {}", name.green(), install_path);

            // Save updated configuration
            manager.save_config(&cli.config).await?;
        }

        Commands::Uninstall { name, version } => {
            let mut manager = init_from_config(&cli.config).await?;
            manager.initialize().await?;

            manager.uninstall_plugin(&name, &version).await?;
            println!("✅ {} {} uninstalled", name.green(), version);
        }

        Commands::List { installed: _ } => {
            let manager = init_from_config(&cli.config).await?;
            let plugins = manager.list_plugins().await;

            if plugins.is_empty() {
                println!("No plugins found");
                return Ok(());
            }

            println!("Available plugins:");
            for plugin_name in plugins {
                let plugin = manager.get_plugin(&plugin_name).await?;
                let metadata = plugin.metadata();
                let status_icon = match plugin.status() {
                    plm::traits::PluginStatus::Active => "✓".green(),
                    plm::traits::PluginStatus::Inactive => "✗".red(),
                    plm::traits::PluginStatus::Loading => "⏳".yellow(),
                    plm::traits::PluginStatus::Error(_) => "⚠".red(),
                };

                println!(
                    "  {} {} - {}",
                    status_icon,
                    plugin_name.cyan(),
                    metadata.description
                );
            }
        }

        Commands::Info { name } => {
            let manager = init_from_config(&cli.config).await?;
            let plugin = manager.get_plugin(&name).await?;
            let metadata = plugin.metadata();

            println!("{}", format!("Plugin Information: {}", name).bold().blue());
            println!("  Name: {}", metadata.name);
            println!("  Version: {}", metadata.version);
            println!("  Description: {}", metadata.description);
            println!("  Author: {}", metadata.author);

            if let Some(homepage) = &metadata.homepage {
                println!("  Homepage: {}", homepage);
            }

            if let Some(repository) = &metadata.repository {
                println!("  Repository: {}", repository);
            }

            println!(
                "  Supported Platforms: {}",
                metadata.supported_platforms.join(", ")
            );

            if !metadata.tags.is_empty() {
                println!("  Tags: {}", metadata.tags.join(", "));
            }
        }

        Commands::Discover => {
            let mut manager = init_from_config(&cli.config).await?;
            manager.initialize().await?;

            let count = manager.discover_plugins().await?;
            if count > 0 {
                println!("✅ Discovered {} new plugins", count);
                manager.save_config(&cli.config).await?;
            } else {
                println!("ℹ️  No new plugins found");
            }
        }

        Commands::Validate { name } => {
            let manager = init_from_config(&cli.config).await?;

            if let Some(plugin_name) = name {
                let plugin = manager.get_plugin(&plugin_name).await?;
                // 简化的验证逻辑 - 检查插件元数据
                let metadata = plugin.metadata();
                let is_valid = !metadata.name.is_empty() && !metadata.version.is_empty();

                if is_valid {
                    println!("✅ {} - Valid", plugin_name.green());
                } else {
                    println!("❌ {} - Invalid (incomplete metadata)", plugin_name.red());
                }
            } else {
                let summary = manager.validate_all_plugins().await?;
                println!("📊 Validation Summary:");
                println!(
                    "  Valid plugins: {}",
                    summary.valid_plugins.to_string().green()
                );
                println!(
                    "  Invalid plugins: {}",
                    summary.invalid_plugins.to_string().red()
                );

                if !summary.errors.is_empty() {
                    println!("  Errors:");
                    for error in &summary.errors {
                        println!("    - {}", error.red());
                    }
                }
            }
        }

        Commands::Config { name, key, value } => {
            let mut manager = init_from_config(&cli.config).await?;

            match (key, value) {
                (Some(k), Some(v)) => {
                    // Set configuration value
                    let json_value = serde_json::Value::String(v.clone());
                    // 获取可变配置并更新
                    let mut config = manager.get_config().clone();
                    if let Some(plugin_config) = config.get_plugin_mut(&name) {
                        plugin_config.set_setting(&k, json_value);
                        manager.update_config(config);
                    } else {
                        println!("Plugin '{}' not found", name);
                        return Ok(());
                    }
                    manager.save_config(&cli.config).await?;
                    println!("✅ Set {} {} = {}", name.cyan(), k, v);
                }
                (Some(k), None) => {
                    // Get configuration value
                    if let Some(plugin) = manager.get_config().get_plugin(&name) {
                        if let Some(value) = plugin.get_setting(&k) {
                            println!("{} {} = {}", name.cyan(), k, value);
                        } else {
                            println!("Setting '{}' not found for plugin '{}'", k, name);
                        }
                    } else {
                        println!("Plugin '{}' not found", name);
                    }
                }
                (None, None) => {
                    // Show all configuration
                    if let Some(plugin_config) = manager.get_config().get_plugin(&name) {
                        println!("Configuration for {}:", name.cyan());
                        for (key, value) in &plugin_config.settings {
                            println!("  {} = {}", key, value);
                        }
                    } else {
                        println!("Plugin '{}' not found", name);
                    }
                }
                _ => {
                    eprintln!("Invalid configuration command");
                    std::process::exit(1);
                }
            }
        }

        Commands::Export { output } => {
            let manager = init_from_config(&cli.config).await?;
            manager.save_config(&output).await?;
            println!("✅ Configuration exported to {}", output);
        }

        Commands::Import { input } => {
            let mut manager = init_from_config(&cli.config).await?;
            // 加载新配置并更新管理器
            let new_config = plm::config::ProjectConfig::load_from_file(&input).await?;
            manager.update_config(new_config);
            manager.save_config(&cli.config).await?;
            println!("✅ Configuration imported from {}", input);
        }
    }

    Ok(())
}
