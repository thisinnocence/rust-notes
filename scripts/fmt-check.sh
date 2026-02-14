#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

usage() {
  echo "usage: ./scripts/fmt-check.sh [--check]"
  echo "  no flag  : format all .rs files under src/"
  echo "  --check  : check formatting only, no write"
}

mode="format"
# 仅支持一个可选参数 --check
# 默认是 format 模式, 传 --check 时切换为 check 模式
if [[ "${1:-}" == "--check" ]]; then
  mode="check"
  # 移除已消费的参数, 方便后续检测是否还有非法参数
  shift
fi

# 到这里参数应该被消费完毕, 否则视为非法用法
if [[ $# -ne 0 ]]; then
  usage
  exit 1
fi

if ! command -v rustfmt >/dev/null 2>&1; then
  echo "[fmt-check] rustfmt not found"
  echo "[fmt-check] install toolchain first: rustup component add rustfmt"
  exit 1
fi

# 扫描 src 下所有 .rs 文件并排序, 结果读入数组 files
mapfile -t files < <(rg --files src -g '*.rs' | sort)

if [[ ${#files[@]} -eq 0 ]]; then
  echo "[fmt-check] no .rs files under src/"
  exit 0
fi

if [[ "$mode" == "check" ]]; then
  # --check 模式下 rustfmt 不会修改文件, 只会检查格式是否符合规范
  echo "[fmt-check] rustfmt --check on src/**/*.rs"
  rustfmt --edition 2024 --check "${files[@]}"
  echo "[fmt-check] done"
else
  echo "[fmt-check] rustfmt on src/**/*.rs"
  rustfmt --edition 2024 "${files[@]}"
  echo "[fmt-check] done"
fi
