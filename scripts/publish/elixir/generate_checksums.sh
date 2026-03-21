#!/usr/bin/env bash
#
# Generate checksum file for Elixir NIF binaries from GitHub release.
#
# Usage: ./generate_checksums.sh <version>
# Example: ./generate_checksums.sh 1.0.0-rc.12
#
# Must be run BEFORE `mix hex.publish` because RustlerPrecompiled
# validates checksums during compilation.

set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
REPO="kreuzberg-dev/tree-sitter-language-pack"
CHECKSUM_FILE="crates/ts-pack-elixir/checksum-Elixir.TreeSitterLanguagePack.exs"

TARGETS=(
  "aarch64-apple-darwin"
  "aarch64-unknown-linux-gnu"
  "x86_64-unknown-linux-gnu"
)

NIF_VERSIONS=("2.16" "2.17")

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Generating checksums for v${VERSION}..."

CHECKSUMS=()

for TARGET in "${TARGETS[@]}"; do
  for NIF_VERSION in "${NIF_VERSIONS[@]}"; do
    FILENAME="libts_pack_elixir-v${VERSION}-nif-${NIF_VERSION}-${TARGET}.so.tar.gz"
    URL="https://github.com/${REPO}/releases/download/v${VERSION}/${FILENAME}"

    echo "Downloading: $FILENAME"

    if curl -fsSL -o "${TMPDIR}/${FILENAME}" "$URL"; then
      if command -v sha256sum &>/dev/null; then
        CHECKSUM=$(sha256sum "${TMPDIR}/${FILENAME}" | cut -d' ' -f1)
      elif command -v shasum &>/dev/null; then
        CHECKSUM=$(shasum -a 256 "${TMPDIR}/${FILENAME}" | cut -d' ' -f1)
      else
        echo "ERROR: No sha256sum or shasum command found"
        exit 1
      fi

      echo "  Checksum: sha256:${CHECKSUM}"
      CHECKSUMS+=("  \"${FILENAME}\" => \"sha256:${CHECKSUM}\",")
    else
      echo "  WARNING: Failed to download $FILENAME (may not exist for this target/nif combo)"
    fi
  done
done

if [[ ${#CHECKSUMS[@]} -eq 0 ]]; then
  echo "ERROR: No checksums generated"
  exit 1
fi

mapfile -t SORTED_CHECKSUMS < <(printf '%s\n' "${CHECKSUMS[@]}" | sort)

echo "Writing checksum file: $CHECKSUM_FILE"
{
  echo "%{"
  for CHECKSUM in "${SORTED_CHECKSUMS[@]}"; do
    echo "$CHECKSUM"
  done
  echo "}"
} >"$CHECKSUM_FILE"

echo ""
echo "Done! Generated checksums for ${#SORTED_CHECKSUMS[@]} files."
cat "$CHECKSUM_FILE"
