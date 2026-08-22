#!/usr/bin/env bash
# Build the wasm-bindgen package so its type declarations exist for TypeScript snippet
# validation.
#
# `alef snippets check` validates wasm/typescript snippets at compile level: it points `tsc` at
# crates/ts-pack-core-wasm/package.json's `types` field
# (pkg/nodejs/ts_pack_core_wasm.d.ts). `pkg/` is gitignored and never checked in, so on a fresh
# checkout every wasm snippet fails with "Cannot find module
# '@xberg-io/tree-sitter-language-pack-wasm'" — the failure describes the environment, not the
# snippet. This script is the wasm snippet session's `before` hook in alef.toml.
#
# TSLP_LANGUAGES=mojo,nim,norg is deliberate, not arbitrary: all three are in
# crates/ts-pack-core/build.rs's DEFAULT_WASM_SKIP_GRAMMARS (their scanners need libc/libc++
# facilities wasi-libc does not provide freestanding), so build.rs skips compiling them into the
# wasm32 target rather than failing. That keeps `static_compiled` empty, which in turn skips
# compiling the utf8proc archive — so this build needs no wasi-sysroot / wasi-sdk at all, only the
# `wasm32-unknown-unknown` Rust target. The three names still have to be real, cloneable grammars
# (parser.c must exist before the skip check runs), which is why these three specifically, not an
# empty selection: an unset/empty TSLP_LANGUAGES falls back to build.rs's ~50-language
# WASM_DEFAULT_LANGUAGES set, which does need wasi-sysroot and takes far longer than this session's
# timeout budget affords. Snippets are only type-checked, never executed, and the generated TS API
# surface (`process()`, etc.) does not vary with which grammars are statically linked in, so a
# near-empty static language set produces the same public declarations as a full build. ~keep
set -euo pipefail

ROOT="$(CDPATH='' cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
CRATE_DIR="${ROOT}/crates/ts-pack-core-wasm"
DECLARATION_FILE="${CRATE_DIR}/pkg/nodejs/ts_pack_core_wasm.d.ts"

if command -v rustup >/dev/null 2>&1; then
  rustup target add wasm32-unknown-unknown >/dev/null
fi

(
  cd "$CRATE_DIR"
  PROJECT_ROOT="$ROOT" TSLP_LANGUAGES=mojo,nim,norg TSLP_LINK_MODE=static \
    wasm-pack build --release --target nodejs --out-dir pkg/nodejs
)

if [ ! -f "$DECLARATION_FILE" ]; then
  echo "ERROR: wasm-pack reported success but $DECLARATION_FILE does not exist" >&2
  exit 1
fi

echo "built wasm type declarations at ${DECLARATION_FILE}"
