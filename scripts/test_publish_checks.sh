#!/usr/bin/env bash
# Test publish workflow idempotency checks locally
set -euo pipefail

VERSION="${1:-1.0.0-rc.1}"
echo "Testing registry checks for version: $VERSION"
echo ""

PASS=0
FAIL=0

check_registry() {
  local name="$1" url="$2" expected="$3"
  local MAX_ATTEMPTS=3
  for attempt in $(seq 1 "$MAX_ATTEMPTS"); do
    STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
      --retry 2 --retry-delay 3 --connect-timeout 10 --max-time 30 \
      "$url")
    if [[ "$STATUS" == "200" ]]; then
      if [[ "$expected" == "exists" ]]; then
        echo "  PASS  $name: EXISTS (expected)"
        PASS=$((PASS + 1))
      else
        echo "  FAIL  $name: EXISTS (expected NOT FOUND)"
        FAIL=$((FAIL + 1))
      fi
      return 0
    elif [[ "$STATUS" == "404" ]]; then
      if [[ "$expected" == "not_found" ]]; then
        echo "  PASS  $name: NOT FOUND (expected)"
        PASS=$((PASS + 1))
      else
        echo "  FAIL  $name: NOT FOUND (expected EXISTS)"
        FAIL=$((FAIL + 1))
      fi
      return 0
    fi
    echo "  ...   $name: attempt $attempt unexpected status $STATUS, retrying..."
    sleep $((attempt * 2))
  done
  echo "  FAIL  $name: UNREACHABLE after $MAX_ATTEMPTS attempts"
  FAIL=$((FAIL + 1))
}

echo "1. Unreleased version should NOT exist on any registry:"
check_registry "crates.io (ts-pack-core/$VERSION)" \
  "https://crates.io/api/v1/crates/ts-pack-core/$VERSION" "not_found"
check_registry "PyPI (tree-sitter-language-pack/$VERSION)" \
  "https://pypi.org/pypi/tree-sitter-language-pack/$VERSION/json" "not_found"
check_registry "npm (@kreuzberg/tree-sitter-language-pack/$VERSION)" \
  "https://registry.npmjs.org/@kreuzberg%2Ftree-sitter-language-pack/$VERSION" "not_found"

echo ""
echo "2. Known published version SHOULD exist (PyPI 0.11.0):"
check_registry "PyPI (tree-sitter-language-pack/0.11.0)" \
  "https://pypi.org/pypi/tree-sitter-language-pack/0.11.0/json" "exists"

echo ""
echo "3. Completely fake package should NOT exist:"
check_registry "PyPI (nonexistent-pkg-xyz/9.9.9)" \
  "https://pypi.org/pypi/nonexistent-pkg-xyz-abc-123/9.9.9/json" "not_found"

echo ""
echo "---"
echo "Results: $PASS passed, $FAIL failed"

if [[ "$FAIL" -gt 0 ]]; then
  echo "FAILED"
  exit 1
fi
echo "ALL PASSED"
