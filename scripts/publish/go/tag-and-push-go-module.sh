#!/usr/bin/env bash
set -euo pipefail

tag="${1:?Release tag argument required (e.g. v1.6.0)}"

# For v1 modules the Go proxy expects: packages/go/vX.Y.Z
# (no major-version subdirectory; that convention only applies to v2+)
module_tag="packages/go/${tag}"

repo="${GITHUB_REPOSITORY:-kreuzberg-dev/tree-sitter-language-pack}"
sha=$(git rev-parse "${tag}^{commit}")

create_tag() {
  local t="$1"

  if git rev-parse "$t" >/dev/null 2>&1; then
    echo "::notice::Go module tag $t already exists locally; skipping."
    return
  fi

  if git ls-remote --tags origin | grep -q "refs/tags/${t}$"; then
    echo "::notice::Go module tag $t already exists on remote; skipping."
    return
  fi

  git tag -a "$t" "$tag" -m "Go module tag ${t}"

  if ! git push origin "refs/tags/${t}" 2>/dev/null; then
    echo "::warning::git push failed for tag $t, trying GitHub API..."
    gh api "repos/${repo}/git/refs" \
      -f "ref=refs/tags/${t}" \
      -f "sha=${sha}" \
      --silent
  fi

  echo "Go module tag created: $t (sha: ${sha:0:12})"
}

create_tag "$module_tag"
