#!/usr/bin/env bash

# 严格模式：
# -e: 任意命令失败立即退出
# -u: 使用未定义变量时报错
# -o pipefail: 管道中任一子命令失败都算失败
set -euo pipefail

# 切换到仓库根目录，确保在任意路径调用都可用。
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

echo "[run-all] cargo check --bins"
cargo check --bins

echo "[run-all] run all src/bin/*.rs"

# 按文件名排序后逐个运行，保证和章节编号一致。
for file in $(ls src/bin/*.rs | sort); do
  bin_name="$(basename "$file" .rs)"
  echo "[run-all] cargo run --bin $bin_name"
  cargo run --bin "$bin_name"
done

echo "[run-all] done"
