#!/bin/bash

# PLM Pre-commit Hook
# 在提交前自动检查和修复代码格式

set -e

echo "🔍 PLM Pre-commit Hook"
echo "===================="

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

# 检查是否有 Rust 文件被修改
check_rust_files() {
    local rust_files=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(rs)$' || true)
    if [ -z "$rust_files" ]; then
        print_success "没有 Rust 文件被修改，跳过检查"
        exit 0
    fi
    echo "检测到以下 Rust 文件被修改:"
    echo "$rust_files" | sed 's/^/  - /'
}

# 检查代码格式
check_format() {
    print_step "检查代码格式"
    
    if ! cargo fmt --all -- --check > /dev/null 2>&1; then
        print_warning "代码格式不符合规范，正在自动修复..."
        cargo fmt --all
        
        # 将格式化后的文件添加到暂存区
        git add -u
        
        print_success "代码格式已自动修复并添加到暂存区"
    else
        print_success "代码格式检查通过"
    fi
}

# 运行 Clippy 检查
check_clippy() {
    print_step "运行 Clippy 检查"
    
    # 先尝试运行 clippy 并捕获输出
    if clippy_output=$(cargo clippy --all-targets --all-features -- -D warnings 2>&1); then
        print_success "Clippy 检查通过"
    else
        print_error "Clippy 检查失败"
        echo ""
        echo "请修复以下问题后重新提交:"
        echo "$clippy_output"
        echo ""
        echo "💡 常见修复方法:"
        echo "  - 运行 'cargo clippy --fix' 自动修复部分问题"
        echo "  - 查看具体错误信息并手动修复"
        echo "  - 如需跳过检查，使用: git commit --no-verify"
        exit 1
    fi
}

# 运行测试
run_tests() {
    print_step "运行快速测试"
    
    if cargo test --lib > /dev/null 2>&1; then
        print_success "测试通过"
    else
        print_error "测试失败"
        echo ""
        echo "请修复测试问题后重新提交:"
        cargo test --lib
        exit 1
    fi
}

# 检查 Cargo.toml 格式
check_cargo_toml() {
    print_step "检查 Cargo.toml 格式"
    
    if command -v taplo &> /dev/null; then
        if taplo format --check Cargo.toml > /dev/null 2>&1; then
            print_success "Cargo.toml 格式正确"
        else
            print_warning "Cargo.toml 格式不正确，正在修复..."
            taplo format Cargo.toml
            git add Cargo.toml
            print_success "Cargo.toml 格式已修复"
        fi
    else
        print_warning "taplo 未安装，跳过 Cargo.toml 格式检查"
        echo "  💡 安装命令: cargo install taplo-cli"
    fi
}

# 主函数
main() {
    check_rust_files
    check_format
    check_clippy
    run_tests
    check_cargo_toml
    
    print_success "所有检查通过，可以提交！"
    echo ""
    echo "🎉 提交准备就绪！"
}

# 执行主函数
main "$@"