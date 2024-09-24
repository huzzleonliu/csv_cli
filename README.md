# Personal Template

This is a personal template for standard Rust develop.

following tools are used:

### 安装 VSCode 插件
- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### cargo 插件
- Cargo generate
cargo install cargo-generate
- pre-commit
pipx install pre-commit
在新的包生成之后，要运行一次
```bash
pre-commit install
```
- cargo deny
Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。
- 安装 typos
typos 是一个拼写检查工具。
```bash
cargo install typos-cli
```
- 安装 git cliff
git cliff 是一个生成 changelog 的工具。
```bash
cargo install git-cliff
```
- 安装 cargo nextest
cargo nextest 是一个 Rust 增强测试工具。
```bash
cargo install cargo-nextest --locked
```
