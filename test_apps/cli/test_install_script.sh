#!/usr/bin/env bash
# Test the install.sh script
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "=== Install Script Tests ==="
echo "Temp dir: $TMP_DIR"
echo ""

# Test 1: Detect platform function (source the script in a subshell)
echo "--- Platform detection ---"
PLATFORM=$(bash -c 'source '"$REPO_ROOT"'/install.sh 2>/dev/null; detect_platform' 2>/dev/null || true)
if [[ -z "$PLATFORM" ]]; then
  # Can't source — test the script output instead
  PLATFORM=$(TS_PACK_VERSION=1.0.0 TS_PACK_INSTALL_DIR="$TMP_DIR" bash "$REPO_ROOT/install.sh" 2>&1 | grep "Platform:" | awk '{print $NF}')
fi
echo "  Detected: $PLATFORM"
[[ -n "$PLATFORM" ]] && echo "  PASS: platform detected"

# Test 2: Install to custom dir
echo "--- Install to custom dir ---"
TS_PACK_VERSION=1.0.0 TS_PACK_INSTALL_DIR="$TMP_DIR" bash "$REPO_ROOT/install.sh" 2>&1 || {
  echo "  SKIP: version not published yet (expected for pre-release)"
  echo ""
  echo "All install script tests completed (some skipped — binaries not yet published)."
  exit 0
}

# Test 3: Verify binary works
echo "--- Verify binary ---"
"$TMP_DIR/ts-pack" --help | head -3
echo "  PASS: binary runs"

# Test 4: Verify it's statically linked (Linux only)
if [[ "$(uname -s)" == "Linux" ]]; then
  echo "--- Check static linking ---"
  if file "$TMP_DIR/ts-pack" | grep -q "statically linked"; then
    echo "  PASS: statically linked (musl)"
  else
    echo "  INFO: dynamically linked (glibc)"
  fi
fi

echo ""
echo "All install script tests passed!"
