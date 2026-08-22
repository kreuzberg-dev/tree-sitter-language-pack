#!/usr/bin/env bash
# Point Gradle at an Android SDK, then compile the module's Kotlin sources so the kotlin_android
# snippet session's classpath is not an empty directory.
#
# The Kotlin validator resolves its classpath via alef's `alefPrintClasspath` Gradle init-script
# task, which reports `compileDebugKotlin`'s `destinationDirectory` -- a path that exists on the
# classpath whether or not anything has been compiled into it. A fresh checkout has never run
# that task, so every snippet's `import io.xberg.tslp.android.*` fails with "unresolved reference
# 'io'" (521 of them in CI run 32520643605).
#
# `local.properties` (not an env var) is how this script tells Gradle where the SDK is: alef's
# snippet runner spawns every command -- `before` hooks included -- through an environment
# allowlist that does not carry ANDROID_HOME/ANDROID_SDK_ROOT (see
# src/snippets/validators/mod.rs::SANITIZED_ENVIRONMENT_VARIABLES in the alef repo), even though
# the CI runner sets them. Writing `sdk.dir` directly sidesteps that entirely and needs no alef
# change. The candidate paths below are what's actually present in each environment this repo
# runs in: `/usr/local/lib/android/sdk` is the GitHub-hosted ubuntu-latest runner's preinstalled
# SDK; the `Library/Android/sdk` and `Android/Sdk` paths are Android Studio's own defaults; the
# `android-commandlinetools` paths are Homebrew's. ~keep
set -euo pipefail

ROOT="$(CDPATH='' cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
MODULE_DIR="${ROOT}/packages/kotlin-android"

CANDIDATE_SDK_ROOTS=(
  "${ANDROID_HOME:-}"
  "${ANDROID_SDK_ROOT:-}"
  "/usr/local/lib/android/sdk"
  "${HOME}/Library/Android/sdk"
  "${HOME}/Android/Sdk"
  "/opt/homebrew/share/android-commandlinetools"
  "/usr/local/share/android-commandlinetools"
)

sdk_root=""
for candidate in "${CANDIDATE_SDK_ROOTS[@]}"; do
  if [ -n "$candidate" ] && [ -d "${candidate}/platforms" ]; then
    sdk_root="$candidate"
    break
  fi
done

if [ -z "$sdk_root" ]; then
  echo "ERROR: no Android SDK found (checked ANDROID_HOME, ANDROID_SDK_ROOT, and common install paths)" >&2
  exit 1
fi

echo "sdk.dir=${sdk_root}" >"${MODULE_DIR}/local.properties"

(
  cd "$MODULE_DIR"
  ./gradlew compileDebugKotlin --no-daemon
)
