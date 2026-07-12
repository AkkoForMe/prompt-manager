# 参与贡献

感谢你愿意改进 Prompt Manager。Bug 报告、功能建议、文档和代码贡献都很欢迎。

## 提交问题

提交 Issue 前，请先搜索是否已有相同问题。Bug 报告尽量包含：

- 操作系统和应用版本
- 可以稳定复现的步骤
- 预期行为与实际行为
- 已隐藏账号、密钥和提示词内容的截图或日志

请勿在公开 Issue 中粘贴真实提示词、访问令牌、账号密码或导出的数据库内容。

## 本地开发

```bash
npm install
npm run tauri dev
```

提交代码前请运行：

```bash
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
```

## Pull Request

1. Fork 本仓库并从主分支创建功能分支。
2. 每个 Pull Request 聚焦一个明确问题。
3. 说明修改原因、主要变化和验证方式。
4. 涉及界面时附上不含敏感信息的截图。
5. 确认自动检查通过后再请求合并。

提交贡献即表示你同意贡献内容按本项目的 MIT License 发布。
