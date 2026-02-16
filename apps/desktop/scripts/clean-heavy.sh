#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$APP_DIR/../.." && pwd)"

HEAVY_PATHS=(
  "$REPO_ROOT/target"
  "$APP_DIR/dist"
  "$APP_DIR/node_modules/.vite"
  "$APP_DIR/src-tauri/target"
)

for path in "${HEAVY_PATHS[@]}"; do
  if [ -e "$path" ]; then
    rm -rf "$path"
    echo "removed $path"
  fi
done
