#!/usr/bin/env bash
# Smoke test for ts-pack CLI (installed via Homebrew or cargo install)
set -euo pipefail

BINARY="${TS_PACK_BIN:-ts-pack}"

echo "=== CLI Smoke Tests ==="
echo "Binary: $BINARY"
echo ""

# 1. Version / help
echo "--- help ---"
$BINARY --help | head -5
echo "  PASS: --help"

# 2. cache-dir
echo "--- cache-dir ---"
DIR=$($BINARY cache-dir)
echo "  Cache dir: $DIR"
[ -n "$DIR" ] && echo "  PASS: cache-dir returns non-empty"

# 3. list --manifest (requires network)
echo "--- list --manifest ---"
LANGS=$($BINARY list --manifest 2>&1 | wc -l)
echo "  Languages from manifest: $LANGS"
[ "$LANGS" -ge 100 ] && echo "  PASS: manifest has 100+ languages"

# 4. download python
echo "--- download python ---"
$BINARY download python
echo "  PASS: download python"

# 5. list --downloaded
echo "--- list --downloaded ---"
DOWNLOADED=$($BINARY list --downloaded)
echo "$DOWNLOADED" | grep -q "python" && echo "  PASS: python in downloaded list"

# 6. parse from stdin
echo "--- parse ---"
echo "def hello(): pass" | $BINARY parse - --language python --format sexp | head -3
echo "  PASS: parse stdin"

# 7. process from stdin
echo "--- process ---"
RESULT=$(echo "def hello(): pass" | $BINARY process - --language python --structure)
echo "$RESULT" | grep -q '"language"' && echo "  PASS: process returns JSON with language"

# 8. clean
echo "--- clean ---"
$BINARY clean --force
echo "  PASS: clean --force"

echo ""
echo "All CLI smoke tests passed!"
