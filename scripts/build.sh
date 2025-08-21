#!/bin/bash

# PLM 跨平台构建脚本

set -e

echo "🚀 开始构建 PLM..."

# 清理之前的构建
echo "🧹 清理构建目录..."
cargo clean

# 运行测试
echo "🧪 运行测试..."
cargo test

# 构建发布版本
echo "🔨 构建发布版本..."
cargo build --release

# 检查二进制文件
if [ -f "target/release/plm" ]; then
    echo "✅ 构建成功！"
    echo "📦 二进制文件位置: target/release/plm"
    echo "📏 文件大小: $(du -h target/release/plm | cut -f1)"
    
    # 测试基本功能
    echo "🔍 测试基本功能..."
    ./target/release/plm --version
    ./target/release/plm --help > /dev/null
    echo "✅ 基本功能测试通过！"
else
    echo "❌ 构建失败！"
    exit 1
fi

echo "🎉 PLM 构建完成！"