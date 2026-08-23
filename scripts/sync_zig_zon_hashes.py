#!/usr/bin/env python3
"""Verify or regenerate the `.hash` fields in `test_apps/zig/build.zig.zon`.

A Zig package hash is `<name>-<version>-<digest>`, where the digest is computed over the fetched
tarball's contents. `alef sync-versions` repoints each `.url` at the release being cut, but a
content digest cannot be derived from a version string, so nothing rewrites the `.hash` and it
stays frozen at whatever release last set it by hand.

It was frozen at the 1.14.3 values from the 1.14.3 release through 1.15.7, so every `.url` in that
window named a tarball whose digest did not match the declared hash, and `zig build` in
`test_apps/zig` failed with:

    error: hash mismatch: manifest declares tree_sitter_language_pack-1.14.3-jCz0Y85sQQ...
    but the fetched package has tree_sitter_language_pack-1.15.7-jCz0Y86TSQ...

Nothing reported it because no workflow builds `test_apps/zig` -- `ci-zig` builds `packages/zig`
and `ci-e2e` runs `e2e/zig`, so the app that resolves these URLs is never exercised.

The only reproducible way to produce the value is to let Zig compute it: `zig fetch <url>` prints
the exact hash Zig will demand. Never edit a hash by hand and never rewrite the version substring
inside one -- that yields a well-formed hash that matches no artifact, turning a loud mismatch
into a silent lie.

Because the digest comes from the tarball, `--fix` only works once the release has published its
Zig assets. It runs *after* a publish, never during release prep. Verification distinguishes that
window explicitly: an asset that 404s is reported as not-yet-published and is not a failure.

Usage:
    python3 scripts/sync_zig_zon_hashes.py          # verify every hash against its tarball
    python3 scripts/sync_zig_zon_hashes.py --fix    # rewrite every hash from `zig fetch`
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ZON = ROOT / "test_apps" / "zig" / "build.zig.zon"

# ~keep Captures the `.url`/`.hash` pair of one dependency. They are adjacent in every manifest
# alef generates, and pairing them positionally is what lets a hash be attributed to the tarball
# it actually guards rather than to a dependency name parsed separately.
DEPENDENCY = re.compile(r'\.url\s*=\s*"(?P<url>[^"]+)"\s*,\s*\.hash\s*=\s*"(?P<hash>[^"]+)"')

# ~keep `zig fetch` resolves a build root before it will run, even though fetching by URL needs
# nothing from it. A throwaway package satisfies that without touching the real one.
PROBE_BUILD_ZIG = 'const std = @import("std");\npub fn build(b: *std.Build) void {\n    _ = b;\n}\n'
PROBE_BUILD_ZON = """.{
    .name = .zig_hash_probe,
    .version = "0.0.0",
    .fingerprint = 0x9e6b1a2c4d8f3057,
    .minimum_zig_version = "0.16.0",
    .dependencies = .{},
    .paths = .{""},
}
"""

NOT_PUBLISHED_MARKERS = ("404", "not found", "Not Found")


class FetchError(RuntimeError):
    """`zig fetch` failed for a reason other than the asset not existing yet."""


class NotPublishedError(RuntimeError):
    """The release asset the URL names does not exist yet."""


def fetch_hash(url: str, probe: Path, cache: Path) -> str:
    """Return the package hash Zig computes for `url`.

    A dedicated global cache directory guarantees the digest is recomputed from a fresh download
    rather than replayed from a previously fetched entry -- a stale cache here would silently
    confirm whatever hash is already declared. ~keep
    """
    result = subprocess.run(
        ["zig", "fetch", "--global-cache-dir", str(cache), url],
        capture_output=True,
        text=True,
        cwd=probe,
        check=False,
    )
    if result.returncode != 0:
        stderr = result.stderr.strip()
        if any(marker in stderr for marker in NOT_PUBLISHED_MARKERS):
            raise NotPublishedError(stderr.splitlines()[-1] if stderr else "asset not found")
        raise FetchError(stderr or f"zig fetch exited {result.returncode}")
    return result.stdout.strip()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--fix", action="store_true", help="rewrite each hash from `zig fetch` instead of reporting")
    args = parser.parse_args()

    if shutil.which("zig") is None:
        # ~keep Never pass silently on a missing toolchain: a hash gate that cannot compute a hash
        # has not verified anything, and reporting success here is how this drift stayed invisible.
        print("zig is not on PATH — cannot compute package hashes", file=sys.stderr)
        return 2

    text = ZON.read_text(encoding="utf-8")
    dependencies = list(DEPENDENCY.finditer(text))
    if not dependencies:
        print(f"{ZON}: no .url/.hash pairs found — the manifest format changed", file=sys.stderr)
        return 2

    stale: list[tuple[str, str, str]] = []
    pending: list[str] = []
    replacements: dict[str, str] = {}

    with tempfile.TemporaryDirectory() as scratch:
        probe = Path(scratch) / "probe"
        probe.mkdir()
        (probe / "build.zig").write_text(PROBE_BUILD_ZIG, encoding="utf-8")
        (probe / "build.zig.zon").write_text(PROBE_BUILD_ZON, encoding="utf-8")
        cache = Path(scratch) / "cache"

        for match in dependencies:
            url = match.group("url")
            declared = match.group("hash")
            asset = url.rsplit("/", 1)[-1]
            try:
                actual = fetch_hash(url, probe, cache)
            except NotPublishedError as exc:
                pending.append(f"{asset}: {exc}")
                continue
            except FetchError as exc:
                print(f"{asset}: {exc}", file=sys.stderr)
                return 2
            if actual != declared:
                stale.append((asset, declared, actual))
                replacements[declared] = actual

    if pending:
        # ~keep Between a version bump and the publish, the URLs name assets that do not exist.
        # That is the normal release window, not drift, so it must not fail the gate.
        print("not published yet (expected between a version bump and its release):")
        for entry in pending:
            print(f"  {entry}")

    if not stale:
        checked = len(dependencies) - len(pending)
        print(f"all {checked} zig package hashes match the tarballs their URLs name")
        return 0

    if args.fix:
        for declared, actual in replacements.items():
            text = text.replace(f'"{declared}"', f'"{actual}"')
        ZON.write_text(text, encoding="utf-8")
        for asset, declared, actual in stale:
            print(f"fixed  {asset}\n         {declared}\n      -> {actual}")
        return 0

    print(f"\n{ZON.relative_to(ROOT)}: {len(stale)} hash(es) do not match the tarball the URL names:\n")
    for asset, declared, actual in stale:
        print(f"  {asset}\n    declared: {declared}\n    actual:   {actual}")
    print(
        "\n`zig build` in test_apps/zig fails with a hash mismatch until these are regenerated.\n"
        "Run `python3 scripts/sync_zig_zon_hashes.py --fix` and commit. Never hand-edit a hash."
    )
    return 1


if __name__ == "__main__":
    sys.exit(main())
