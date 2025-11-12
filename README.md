# Clap Demo

一个基于 [Clap](https://github.com/clap-rs/clap) 的 Rust 命令行参数解析示例项目。Clap 是 Rust 生态系统中功能最强大、最常用的命令行参数解析库。

## 技术栈

- **Rust**: 1.90
- **Clap**: 4.5.50

## 项目结构

```
clap-demo/
├── src/
│   └── main.rs          # 应用入口，命令行参数定义
├── Cargo.toml           # 项目依赖配置
├── Justfile             # Just 构建工具配置
└── README.md
```

## 功能特性

- 命令行参数解析
- 帮助信息生成
- 版本信息显示
- 参数验证

## 快速开始

### 前置要求

- Rust 1.90 或更高版本
- Cargo (Rust 包管理器)

### 安装和运行

```bash
# 克隆项目
git clone <repository-url>
cd clap-demo

# 构建项目
cargo build

# 运行项目（显示帮助信息）
cargo run -- --help

# 运行项目（使用参数）
cargo run -- --name "John Doe"
```

## 命令行选项

### 可用参数

- `--name <value>`: 设置要使用的名称（可选）

### 使用示例

```bash
# 显示帮助信息
cargo run -- --help

# 显示版本信息
cargo run -- --version

# 使用 name 参数
cargo run -- --name "Alice"
```

## 代码说明

### 主应用 (`src/main.rs`)

应用定义了以下命令行接口：
- **程序名称**: "Clap Example"
- **作者**: user@example.com
- **版本**: 0.1.0
- **描述**: 简要说明程序功能
- **参数**: `--name` (可选，用于设置名称)
- **帮助信息**: 包含详细的使用说明

### 构建应用函数

`build_app()` 函数使用 Clap 的构建器模式创建命令行应用：
- 设置元数据（名称、作者、版本、描述）
- 定义命令行参数
- 配置帮助信息

## 开发

### 运行测试

```bash
cargo test
```

### 构建发布版本

```bash
cargo build --release
```

## 参考资源

- [Clap 官方仓库](https://github.com/clap-rs/clap)
- [Clap 示例代码](https://github.com/clap-rs/clap/tree/master/examples)
- [Clap 文档](https://docs.rs/clap)
