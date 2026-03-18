#!/usr/bin/env bash
# Install ts-pack CLI — tree-sitter language pack
# Usage: curl -fsSL https://raw.githubusercontent.com/kreuzberg-dev/tree-sitter-language-pack/main/install.sh | bash
set -euo pipefail

REPO="kreuzberg-dev/tree-sitter-language-pack"
BINARY="ts-pack"
INSTALL_DIR="${TS_PACK_INSTALL_DIR:-/usr/local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info() { echo -e "${CYAN}==>${NC} $*"; }
ok() { echo -e "${GREEN}==>${NC} $*"; }
warn() { echo -e "${YELLOW}==>${NC} $*"; }
error() {
  echo -e "${RED}error:${NC} $*" >&2
  exit 1
}

detect_platform() {
  local os arch

  case "$(uname -s)" in
  Linux*) os="linux" ;;
  Darwin*) os="macos" ;;
  MINGW* | MSYS* | CYGWIN*) os="windows" ;;
  *) error "Unsupported OS: $(uname -s)" ;;
  esac

  case "$(uname -m)" in
  x86_64 | amd64) arch="x86_64" ;;
  aarch64 | arm64) arch="aarch64" ;;
  *) error "Unsupported architecture: $(uname -m)" ;;
  esac

  # Prefer musl on Linux for maximum portability
  if [[ "$os" == "linux" && "$arch" == "x86_64" ]]; then
    if [[ "${TS_PACK_USE_MUSL:-true}" == "true" ]]; then
      echo "linux-x86_64-musl"
      return
    fi
  fi

  if [[ "$os" == "macos" ]]; then
    echo "macos-arm64"
  elif [[ "$os" == "windows" ]]; then
    echo "windows-x86_64"
  else
    echo "${os}-${arch}"
  fi
}

get_latest_version() {
  local url="https://api.github.com/repos/${REPO}/releases/latest"
  local version

  version=$(curl -fsSL "$url" 2>/dev/null | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')

  if [[ -z "$version" ]]; then
    # Fallback: get latest tag
    version=$(curl -fsSL "https://api.github.com/repos/${REPO}/tags?per_page=1" 2>/dev/null | grep '"name"' | head -1 | sed 's/.*"name": *"//;s/".*//')
  fi

  if [[ -z "$version" ]]; then
    error "Could not determine latest version. Set TS_PACK_VERSION to install a specific version."
  fi

  echo "$version"
}

main() {
  local platform version url archive_name tmp_dir

  info "Installing ${BINARY}..."

  platform=$(detect_platform)
  version="${TS_PACK_VERSION:-$(get_latest_version)}"
  version="${version#v}" # strip leading v

  info "Platform: ${platform}"
  info "Version:  ${version}"

  if [[ "$platform" == *"windows"* ]]; then
    archive_name="ts-pack-${platform}.zip"
  else
    archive_name="ts-pack-${platform}.tar.gz"
  fi

  url="https://github.com/${REPO}/releases/download/v${version}/${archive_name}"

  info "Downloading ${url}..."

  tmp_dir=$(mktemp -d)
  trap 'rm -rf "$tmp_dir"' EXIT

  if ! curl -fsSL "$url" -o "${tmp_dir}/${archive_name}"; then
    error "Download failed. Check that version v${version} exists and has CLI binaries."
  fi

  info "Extracting..."
  if [[ "$archive_name" == *.zip ]]; then
    unzip -q "${tmp_dir}/${archive_name}" -d "${tmp_dir}"
  else
    tar -xzf "${tmp_dir}/${archive_name}" -C "${tmp_dir}"
  fi

  if [[ ! -f "${tmp_dir}/${BINARY}" && ! -f "${tmp_dir}/${BINARY}.exe" ]]; then
    error "Binary not found in archive. Contents: $(ls "${tmp_dir}")"
  fi

  info "Installing to ${INSTALL_DIR}..."

  if [[ -w "$INSTALL_DIR" ]]; then
    cp "${tmp_dir}/${BINARY}"* "${INSTALL_DIR}/"
    chmod +x "${INSTALL_DIR}/${BINARY}"
  else
    warn "Requires sudo to install to ${INSTALL_DIR}"
    sudo cp "${tmp_dir}/${BINARY}"* "${INSTALL_DIR}/"
    sudo chmod +x "${INSTALL_DIR}/${BINARY}"
  fi

  ok "Installed ${BINARY} v${version} to ${INSTALL_DIR}/${BINARY}"
  echo ""
  echo "  Run '${BINARY} --help' to get started."
  echo "  Run '${BINARY} download python' to download a parser."
  echo ""
}

main "$@"
