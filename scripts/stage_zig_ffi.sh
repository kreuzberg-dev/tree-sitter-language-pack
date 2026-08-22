#!/usr/bin/env bash
# Build ts-pack-core-ffi so packages/zig/build.zig's default `-Dffi_path` (`../../target/release`,
# relative to packages/zig) has a library to link.
#
# `alef snippets check` validates zig snippets at compile level, and for zig that means a real
# `zig build-exe`: the generated per-snippet `build.zig` calls `b.addExecutable` and
# `b.default_step.dependOn(&executable.step)`, so it links, not just type-checks. Without a
# library at the default `ffi_path`, every zig snippet fails with "unable to find dynamic system
# library 'ts_pack_core_ffi'" -- the failure describes the environment, not the snippet. This
# script is the zig snippet session's `before` hook in alef.toml, and mirrors
# `stage_go_native.sh`'s FFI build (no copy step needed here: zig's own default already points at
# `target/release`, unlike Go's `.lib/<platform>` layout).
#
# TSLP_LANGUAGES is deliberately left unset for the same reason stage_go_native.sh leaves it
# unset: an empty grammar selection needs no `parsers/` tree, TSLP_OFFLINE keeps that path off the
# network, and snippets are only compiled, never executed, so a grammar-free library links exactly
# the same. ~keep
set -euo pipefail

ROOT="$(CDPATH='' cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
CRATE=ts-pack-core-ffi
LIB_STEM=libts_pack_core_ffi
BUILT_DIR="${ROOT}/target/release"

(
  cd "$ROOT"
  PROJECT_ROOT="$ROOT" TSLP_OFFLINE=1 TSLP_LINK_MODE=dynamic \
    cargo build --locked --release -p "$CRATE"
)

if ! ls "${BUILT_DIR}/${LIB_STEM}".* >/dev/null 2>&1; then
  echo "ERROR: cargo reported success but no ${LIB_STEM}.* exists in ${BUILT_DIR}" >&2
  exit 1
fi

echo "staged ${LIB_STEM} at ${BUILT_DIR}"
