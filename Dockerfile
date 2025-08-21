# PLM Docker Image
FROM rust:1.70-slim as builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 创建一个虚拟的 src/main.rs 来缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# 复制源代码
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# 构建应用
RUN touch src/main.rs && cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -r -s /bin/false plm

# 复制二进制文件
COPY --from=builder /app/target/release/plm /usr/local/bin/plm

# 设置权限
RUN chmod +x /usr/local/bin/plm

# 切换到非 root 用户
USER plm

# 设置工作目录
WORKDIR /workspace

# 暴露端口（如果需要）
# EXPOSE 8080

# 设置入口点
ENTRYPOINT ["plm"]
CMD ["--help"]