#!/usr/bin/env bash
set -euo pipefail

npx --yes markdownlint-cli notes/*.md
