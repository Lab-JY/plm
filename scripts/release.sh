#!/bin/bash

# PLM 发布脚本
# 用于创建新版本和触发自动构建

set -e

echo "🚀 PLM 发布脚本"
echo "==============="

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_step() {
    echo -e "${BLUE}📋 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查参数
if [ $# -eq 0 ]; then
    echo "用法: $0 <version>"
    echo ""
    echo "示例:"
    echo "  $0 0.1.0    # 发布版本 0.1.0"
    echo "  $0 0.2.0    # 发布版本 0.2.0"
    echo ""
    echo "版本格式: MAJOR.MINOR.PATCH (遵循语义化版本)"
    exit 1
fi

VERSION=$1

# 验证版本格式
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "无效的版本格式: $VERSION"
    echo "版本格式应为: MAJOR.MINOR.PATCH (例如: 1.0.0)"
    exit 1
fi

TAG_NAME="v$VERSION"

print_step "准备发布版本 $VERSION"

# 检查是否在 Git 仓库中
if [ ! -d ".git" ]; then
    print_error "当前目录不是 Git 仓库"
    exit 1
fi

# 检查工作目录是否干净
if [ -n "$(git status --porcelain)" ]; then
    print_error "工作目录不干净，请先提交或暂存更改"
    git status --short
    exit 1
fi

# 检查是否在主分支
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
    print_warning "当前不在主分支 (当前: $CURRENT_BRANCH)"
    read -p "是否继续？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "发布已取消"
        exit 1
    fi
fi

# 检查标签是否已存在
if git tag -l | grep -q "^$TAG_NAME$"; then
    print_error "标签 $TAG_NAME 已存在"
    exit 1
fi

# 更新 Cargo.toml 版本
print_step "更新 Cargo.toml 版本"
if command -v sed &> /dev/null; then
    # macOS 和 Linux 兼容的 sed 命令
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    else
        sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    fi
    print_success "Cargo.toml 版本已更新为 $VERSION"
else
    print_warning "sed 命令不可用，请手动更新 Cargo.toml 中的版本"
fi

# 运行测试
print_step "运行测试"
if cargo test; then
    print_success "所有测试通过"
else
    print_error "测试失败，发布已取消"
    exit 1
fi

# 运行格式检查
print_step "检查代码格式"
if cargo fmt --all -- --check; then
    print_success "代码格式检查通过"
else
    print_error "代码格式不符合规范，请运行 'cargo fmt --all'"
    exit 1
fi

# 运行 Clippy 检查
print_step "运行 Clippy 检查"
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_success "Clippy 检查通过"
else
    print_error "Clippy 检查失败，请修复警告"
    exit 1
fi

# 构建发布版本
print_step "构建发布版本"
if cargo build --release; then
    print_success "发布版本构建成功"
else
    print_error "构建失败，发布已取消"
    exit 1
fi

# 生成变更日志
print_step "生成变更日志"
CHANGELOG_FILE="CHANGELOG-$VERSION.md"
cat > "$CHANGELOG_FILE" << EOF
# PLM $VERSION 发布说明

## 🚀 新功能
- 

## 🐛 Bug 修复
- 

## 📈 改进
- 

## 🔧 其他
- 

## 📦 下载
- [Linux (x86_64)](https://github.com/plm/plm/releases/download/$TAG_NAME/plm-$TAG_NAME-x86_64-unknown-linux-gnu.tar.gz)
- [macOS (Intel)](https://github.com/plm/plm/releases/download/$TAG_NAME/plm-$TAG_NAME-x86_64-apple-darwin.tar.gz)
- [macOS (Apple Silicon)](https://github.com/plm/plm/releases/download/$TAG_NAME/plm-$TAG_NAME-aarch64-apple-darwin.tar.gz)
- [Windows](https://github.com/plm/plm/releases/download/$TAG_NAME/plm-$TAG_NAME-x86_64-pc-windows-msvc.zip)
EOF

print_success "变更日志已生成: $CHANGELOG_FILE"
print_warning "请编辑 $CHANGELOG_FILE 添加具体的变更内容"

# 询问是否继续
echo ""
read -p "是否继续创建标签和推送？(y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "发布已暂停，您可以："
    echo "1. 编辑 $CHANGELOG_FILE"
    echo "2. 运行 'git add . && git commit -m \"Prepare release $VERSION\"'"
    echo "3. 重新运行此脚本"
    exit 0
fi

# 提交版本更改
print_step "提交版本更改"
git add Cargo.toml "$CHANGELOG_FILE"
git commit -m "chore: prepare release $VERSION"
print_success "版本更改已提交"

# 创建标签
print_step "创建标签 $TAG_NAME"
git tag -a "$TAG_NAME" -m "Release $VERSION"
print_success "标签 $TAG_NAME 已创建"

# 推送到远程仓库
print_step "推送到远程仓库"
git push origin "$CURRENT_BRANCH"
git push origin "$TAG_NAME"
print_success "标签和提交已推送到远程仓库"

# 清理临时文件
rm -f "$CHANGELOG_FILE"

echo ""
print_success "🎉 版本 $VERSION 发布完成！"
echo ""
echo "📋 接下来会自动进行："
echo "  ✅ GitHub Actions 将自动构建多平台二进制文件"
echo "  ✅ 创建 GitHub Release"
echo "  ✅ 上传构建产物到 Release"
echo "  ✅ 发布到 Crates.io (如果配置了 token)"
echo ""
echo "🔗 查看发布状态:"
echo "  GitHub Actions: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^.]*\).*/\1/')/actions"
echo "  Releases: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^.]*\).*/\1/')/releases"
echo ""
echo "📦 安装命令 (发布完成后):"
echo "  curl -fsSL https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^.]*\).*/\1/')/releases/download/$TAG_NAME/install.sh | sh"