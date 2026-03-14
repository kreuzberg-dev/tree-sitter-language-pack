#!/usr/bin/env bash
set -euo pipefail
tag="${1:?Release tag argument required (e.g. v1.0.0)}"
version="${tag#v}"
module_tag="packages/go/v${version}"
git tag "$module_tag" "$tag"
git push origin "$module_tag"
