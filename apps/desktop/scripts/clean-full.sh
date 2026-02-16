#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$APP_DIR/../.." && pwd)"

# Start with heavy artifacts.
"$SCRIPT_DIR/clean-heavy.sh"

FULL_PATHS=(
  "$REPO_ROOT/node_modules"
  "$APP_DIR/node_modules"
)

for path in "${FULL_PATHS[@]}"; do
  if [ -e "$path" ]; then
    rm -rf "$path"
    echo "removed $path"
  fi
done
