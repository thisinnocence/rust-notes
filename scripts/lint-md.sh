#!/usr/bin/env bash

# 使用严格模式（strict mode）：
# -e: 任意命令失败立刻退出，避免“带错继续执行”。
# -u: 使用未定义变量时报错，避免隐藏拼写或环境变量问题。
# -o pipefail: pipeline 中任一子命令失败都算失败，不吞错误。
set -euo pipefail

# 使用 pnpm dlx 临时执行 markdownlint-cli：
# - 不需要全局安装（global install）
# - 每次按包名解析并执行 CLI，适合仓库内统一检查（lint）
# - 首次运行通常会先出现 resolved/downloaded/added 等下载日志
# - 依赖准备完成后会立即执行 lint；若无违规，CLI 默认静默成功
# - 检查目标为 notes 目录下全部 Markdown 文件
pnpm dlx markdownlint-cli notes/*.md
