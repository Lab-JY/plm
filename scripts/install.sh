#!/bin/bash

# PLM 安装脚本
# 自动检测系统并下载安装最新版本的 PLM

set -e

echo "🚀 PLM 安装脚本"
echo "==============="

# 颜色定义
RED='\033[0;31m'
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

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 默认配置
REPO="Lab-JY/plm"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="plm"

# 检测操作系统和架构
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case $os in
        linux*)
            OS="linux"
            ;;
        darwin*)
            OS="darwin"
            ;;
        mingw*|msys*|cygwin*)
            OS="windows"
            ;;
        *)
            print_error "不支持的操作系统: $os"
            exit 1
            ;;
    esac
    
    case $arch in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            print_error "不支持的架构: $arch"
            exit 1
            ;;
    esac
    
    # 构建目标三元组
    case $OS in
        linux)
            TARGET="${ARCH}-unknown-linux-gnu"
            ARCHIVE_EXT="tar.gz"
            ;;
        darwin)
            TARGET="${ARCH}-apple-darwin"
            ARCHIVE_EXT="tar.gz"
            ;;
        windows)
            TARGET="${ARCH}-pc-windows-msvc"
            ARCHIVE_EXT="zip"
            BINARY_NAME="plm.exe"
            ;;
    esac
    
    print_step "检测到平台: $OS ($ARCH)"
    print_step "目标: $TARGET"
}

# 获取最新版本
get_latest_version() {
    print_step "获取最新版本信息"
    
    if command -v curl &> /dev/null; then
        VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget &> /dev/null; then
        VERSION=$(wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        print_error "需要 curl 或 wget 来下载文件"
        exit 1
    fi
    
    if [ -z "$VERSION" ]; then
        print_error "无法获取最新版本信息"
        exit 1
    fi
    
    print_success "最新版本: $VERSION"
}

# 下载文件
download_file() {
    local url=$1
    local output=$2
    
    print_step "下载: $url"
    
    if command -v curl &> /dev/null; then
        curl -fsSL -o "$output" "$url"
    elif command -v wget &> /dev/null; then
        wget -q -O "$output" "$url"
    else
        print_error "需要 curl 或 wget 来下载文件"
        exit 1
    fi
}

# 安装二进制文件
install_binary() {
    local archive_name="plm-${VERSION}-${TARGET}.${ARCHIVE_EXT}"
    local download_url="https://github.com/$REPO/releases/download/$VERSION/$archive_name"
    local temp_dir=$(mktemp -d)
    local archive_path="$temp_dir/$archive_name"
    
    print_step "下载 PLM $VERSION"
    download_file "$download_url" "$archive_path"
    
    print_step "解压文件"
    cd "$temp_dir"
    
    case $ARCHIVE_EXT in
        tar.gz)
            tar -xzf "$archive_name"
            ;;
        zip)
            if command -v unzip &> /dev/null; then
                unzip -q "$archive_name"
            else
                print_error "需要 unzip 命令来解压 zip 文件"
                exit 1
            fi
            ;;
    esac
    
    # 查找二进制文件
    if [ -f "$BINARY_NAME" ]; then
        BINARY_PATH="$BINARY_NAME"
    elif [ -f "plm" ]; then
        BINARY_PATH="plm"
    else
        print_error "在解压的文件中找不到二进制文件"
        ls -la
        exit 1
    fi
    
    print_step "安装到 $INSTALL_DIR"
    
    # 检查是否需要 sudo
    if [ -w "$INSTALL_DIR" ]; then
        cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
    else
        print_warning "需要管理员权限安装到 $INSTALL_DIR"
        sudo cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
    fi
    
    # 设置执行权限
    if [ -w "$INSTALL_DIR/$BINARY_NAME" ]; then
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    else
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi
    
    # 清理临时文件
    cd /
    rm -rf "$temp_dir"
    
    print_success "PLM $VERSION 安装完成"
}

# 验证安装
verify_installation() {
    print_step "验证安装"
    
    if command -v plm &> /dev/null; then
        local installed_version=$(plm --version 2>/dev/null | head -n1 || echo "unknown")
        print_success "PLM 已成功安装"
        echo "  版本: $installed_version"
        echo "  路径: $(which plm)"
    else
        print_error "PLM 安装失败或不在 PATH 中"
        echo ""
        echo "请检查:"
        echo "1. $INSTALL_DIR 是否在您的 PATH 环境变量中"
        echo "2. 是否有执行权限"
        echo ""
        echo "手动添加到 PATH (添加到 ~/.bashrc 或 ~/.zshrc):"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        exit 1
    fi
}

# 显示使用说明
show_usage() {
    echo ""
    print_success "🎉 安装完成！"
    echo ""
    echo "📋 快速开始:"
    echo "  plm --help              # 查看帮助"
    echo "  plm init --name my-app  # 初始化项目"
    echo "  plm discover            # 发现插件"
    echo "  plm install node        # 安装插件"
    echo ""
    echo "📚 更多信息:"
    echo "  文档: https://github.com/$REPO"
    echo "  示例: https://github.com/$REPO/tree/main/examples"
    echo ""
}

# 主函数
main() {
    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --version)
                VERSION="$2"
                shift 2
                ;;
            --help)
                echo "PLM 安装脚本"
                echo ""
                echo "用法: $0 [选项]"
                echo ""
                echo "选项:"
                echo "  --install-dir DIR   安装目录 (默认: /usr/local/bin)"
                echo "  --version VERSION   指定版本 (默认: 最新版本)"
                echo "  --help             显示此帮助信息"
                echo ""
                echo "示例:"
                echo "  $0                                    # 安装最新版本"
                echo "  $0 --version v0.1.0                  # 安装指定版本"
                echo "  $0 --install-dir ~/.local/bin        # 安装到用户目录"
                exit 0
                ;;
            *)
                print_error "未知选项: $1"
                echo "使用 --help 查看帮助"
                exit 1
                ;;
        esac
    done
    
    detect_platform
    
    if [ -z "$VERSION" ]; then
        get_latest_version
    else
        print_step "使用指定版本: $VERSION"
    fi
    
    install_binary
    verify_installation
    show_usage
}

# 运行主函数
main "$@"