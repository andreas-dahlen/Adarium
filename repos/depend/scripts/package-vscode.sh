#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
EXTENSION_DIR="$ROOT_DIR/depend-vscode"

echo "Building Depend..."

cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --release

echo "Copying Rust executable..."

mkdir -p "$EXTENSION_DIR/bin/linux-x64"

install -m 755 \
    "$ROOT_DIR/target/release/depend" \
    "$EXTENSION_DIR/bin/linux-x64/depend"