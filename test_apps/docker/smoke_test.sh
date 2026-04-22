#!/usr/bin/env bash
# Docker smoke test — pulls the published GHCR image and validates all core functionality.
#
# Usage:
#   ./smoke_test.sh                           # uses latest tag
#   VERSION=1.0.0-rc.10 ./smoke_test.sh       # specific version
set -euo pipefail

IMAGE="ghcr.io/kreuzberg-dev/ts-pack"
VERSION="${VERSION:-latest}"
TAG="${IMAGE}:${VERSION}"
PASS=0
FAIL=0

assert() {
  if eval "$2"; then
    PASS=$((PASS + 1))
    echo "  PASS: $1"
  else
    FAIL=$((FAIL + 1))
    echo "  FAIL: $1"
  fi
}

echo "=== Docker Smoke Tests ==="
echo "Image: $TAG"
echo ""

echo "--- Pull image ---"
docker pull "$TAG"
assert "image pulled" "docker image inspect $TAG >/dev/null 2>&1"

echo "--- Version ---"
VER=$(docker run --rm "$TAG" --version)
echo "  $VER"
assert "version output" "echo '$VER' | grep -q 'ts-pack'"

echo "--- Help ---"
HELP=$(docker run --rm "$TAG" --help)
assert "help shows commands" "echo '$HELP' | grep -q 'download'"

echo "--- Parse Python ---"
OUT=$(echo "def hello(): pass" | docker run --rm -i "$TAG" parse - --language python)
assert "python parse" "echo '$OUT' | grep -q 'module'"

echo "--- Parse JavaScript ---"
OUT=$(echo "function test() { return 1; }" | docker run --rm -i "$TAG" parse - --language javascript)
assert "javascript parse" "echo '$OUT' | grep -q 'program'"

echo "--- Parse Rust ---"
OUT=$(echo "fn main() {}" | docker run --rm -i "$TAG" parse - --language rust)
assert "rust parse" "echo '$OUT' | grep -q 'source_file'"

echo "--- Parse Go ---"
OUT=$(echo "package main" | docker run --rm -i "$TAG" parse - --language go)
assert "go parse" "echo '$OUT' | grep -q 'source_file'"

echo "--- Parse JSON format ---"
OUT=$(echo "x = 1" | docker run --rm -i "$TAG" parse - --language python --format json)
assert "json format output" "echo '$OUT' | grep -q 'kind'"

echo "--- Process Python ---"
OUT=$(echo "import os; def main(): pass" | docker run --rm -i "$TAG" process - --language python --structure --imports)
assert "process language field" "echo '$OUT' | grep -q '\"language\"'"
assert "process structure field" "echo '$OUT' | grep -q '\"structure\"'"
assert "process imports field" "echo '$OUT' | grep -q '\"imports\"'"

echo "--- Process with chunking ---"
OUT=$(printf "def a():\n    pass\n\ndef b():\n    pass\n\ndef c():\n    pass\n" | docker run --rm -i "$TAG" process - --language python --structure --chunk-size 30)
assert "process chunks field" "echo '$OUT' | grep -q '\"chunks\"'"

echo "--- Cache dir ---"
DIR=$(docker run --rm "$TAG" cache-dir)
assert "cache-dir non-empty" "test -n '$DIR'"

echo "--- Image size ---"
SIZE=$(docker image inspect "$TAG" --format '{{.Size}}')
SIZE_MB=$((SIZE / 1024 / 1024))
echo "  Image size: ${SIZE_MB} MB"
assert "image under 500MB" "test $SIZE_MB -lt 500"

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
test "$FAIL" -eq 0
