#!/bin/bash

# 安装 Git Hooks 脚本

set -e

echo "🔧 安装 PLM Git Hooks"
echo "==================="

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}📋 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️ $1${NC}"
}

# 检查是否在 Git 仓库中
if [ ! -d ".git" ]; then
    echo "❌ 错误: 当前目录不是 Git 仓库"
    exit 1
fi

# 创建 hooks 目录（如果不存在）
if [ ! -d ".git/hooks" ]; then
    mkdir -p .git/hooks
    print_step "创建 .git/hooks 目录"
fi

# 安装 pre-commit hook
print_step "安装 pre-commit hook"

if [ -f ".git/hooks/pre-commit" ]; then
    print_warning "pre-commit hook 已存在，创建备份"
    cp .git/hooks/pre-commit .git/hooks/pre-commit.backup
fi

# 创建 pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash

# PLM Pre-commit Hook
# 调用项目中的 pre-commit 脚本

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

if [ -f "$PROJECT_ROOT/scripts/pre-commit.sh" ]; then
    exec "$PROJECT_ROOT/scripts/pre-commit.sh" "$@"
else
    echo "❌ 错误: 找不到 pre-commit 脚本"
    exit 1
fi
EOF

# 设置执行权限
chmod +x .git/hooks/pre-commit

print_success "pre-commit hook 安装完成"

# 可选：安装其他 hooks
read -p "是否安装 pre-push hook？(y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_step "安装 pre-push hook"
    
    cat > .git/hooks/pre-push << 'EOF'
#!/bin/bash

# PLM Pre-push Hook
# 在推送前运行完整的检查

echo "🚀 运行 pre-push 检查..."

# 运行完整构建
if [ -f "scripts/build.sh" ]; then
    ./scripts/build.sh ci
else
    echo "⚠️ 找不到构建脚本，运行基本检查"
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test
fi

echo "✅ pre-push 检查完成"
EOF

    chmod +x .git/hooks/pre-push
    print_success "pre-push hook 安装完成"
fi

echo ""
print_success "Git hooks 安装完成！"
echo ""
echo "📋 已安装的 hooks:"
echo "  ✅ pre-commit - 提交前检查代码格式和质量"
if [ -f ".git/hooks/pre-push" ]; then
    echo "  ✅ pre-push - 推送前运行完整测试"
fi
echo ""
echo "💡 使用说明:"
echo "  - 现在每次 git commit 时会自动检查和修复代码格式"
echo "  - 如果需要跳过检查，使用: git commit --no-verify"
echo "  - 手动运行检查: ./scripts/pre-commit.sh"