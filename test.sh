#!/usr/bin/env bash
# Manual smoke test: round-trip src/main.rs through walt and show the result.
# Uses a temp dir so no generated artifacts land in the source tree.
set -euo pipefail

cd "$(dirname "$0")"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "== Original src/main.rs =="
cat src/main.rs

echo "== Encoding to .ars =="
cargo run --quiet -- encode src/main.rs "$TMP_DIR/main.ars"
cat "$TMP_DIR/main.ars"

echo "== Decoding back to Rust =="
cargo run --quiet -- decode "$TMP_DIR/main.ars" "$TMP_DIR/main_decoded.rs"
cat "$TMP_DIR/main_decoded.rs"

echo "== Diff (original vs decoded) =="
diff src/main.rs "$TMP_DIR/main_decoded.rs" || true
