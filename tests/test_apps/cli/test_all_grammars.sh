#!/usr/bin/env bash
# Test ALL grammars — loads fixtures from all_grammars.json and parses each.
# Requires: ts-pack CLI + python3 (for JSON parsing)
#
# Usage:
#   ./test_all_grammars.sh                    # uses ts-pack from PATH
#   TS_PACK_BIN=./target/release/ts-pack ./test_all_grammars.sh
set -euo pipefail

BINARY="${TS_PACK_BIN:-ts-pack}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FIXTURES="${SCRIPT_DIR}/../fixtures/all_grammars.json"

echo "=== All Grammars Test Suite ==="
echo "Binary: $BINARY"
echo "Fixtures: $FIXTURES"
echo ""

python3 -c "
import json, subprocess, sys, os

fixtures = json.load(open('$FIXTURES'))
binary = '$BINARY'
passed = 0
failed = 0
skipped = 0
errors = []

# Optional filter: only test specific languages (comma-separated)
filter_langs = os.environ.get('TS_PACK_LANGUAGES', '')
allowed = set(filter_langs.split(',')) if filter_langs else None

print(f'Testing {len(fixtures)} languages...')
if allowed:
    print(f'Filtered to: {sorted(allowed)}')
print()

for f in fixtures:
    lang = f['language']
    source = f['source']
    if f.get('skip'):
        skipped += 1
        continue
    if allowed and lang not in allowed:
        skipped += 1
        continue
    try:
        result = subprocess.run(
            [binary, 'parse', '-', '--language', lang],
            input=source, capture_output=True, text=True, timeout=10,
        )
        if result.returncode == 0 and result.stdout.strip():
            passed += 1
        else:
            failed += 1
            errors.append(f'{lang}: exit={result.returncode} stderr={result.stderr.strip()[:100]}')
    except subprocess.TimeoutExpired:
        failed += 1
        errors.append(f'{lang}: TIMEOUT')
    except Exception as e:
        failed += 1
        errors.append(f'{lang}: {e}')

if errors:
    print('Failures:')
    for e in errors:
        print(f'  FAIL: {e}')
    print()

print(f'=== Results: {passed} passed, {failed} failed, {skipped} skipped out of {len(fixtures)} ===')
sys.exit(1 if failed > 0 else 0)
"
