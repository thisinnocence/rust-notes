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
if [[ "${1:-}" == "--check" ]]; then
  mode="check"
  shift
fi

if [[ $# -ne 0 ]]; then
  usage
  exit 1
fi

if ! command -v rustfmt >/dev/null 2>&1; then
  echo "[fmt-check] rustfmt not found"
  echo "[fmt-check] install toolchain first: rustup component add rustfmt"
  exit 1
fi

mapfile -t files < <(rg --files src -g '*.rs' | sort)

if [[ ${#files[@]} -eq 0 ]]; then
  echo "[fmt-check] no .rs files under src/"
  exit 0
fi

if [[ "$mode" == "check" ]]; then
  echo "[fmt-check] rustfmt --check on src/**/*.rs"
  rustfmt --edition 2024 --check "${files[@]}"
  echo "[fmt-check] done"
else
  echo "[fmt-check] rustfmt on src/**/*.rs"
  rustfmt --edition 2024 "${files[@]}"
  echo "[fmt-check] done"
fi
