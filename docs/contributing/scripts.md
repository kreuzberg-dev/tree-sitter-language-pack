---
description: "Reference for the Python utility scripts in scripts/ — cloning grammars, pinning revisions, generating docs, and more."
---

# Scripts reference

The `scripts/` directory has seven Python utilities for grammar management and code generation. All scripts require Python 3 and the project's dev dependencies (`uv sync`).

## `clone_vendors.py`

Clones grammar repositories into `parsers/`, one directory per language. Uses the `rev` field from `sources/language_definitions.json` to check out the pinned commit.

```bash
# Clone all grammars
uv run scripts/clone_vendors.py

# Clone a specific subset
uv run scripts/clone_vendors.py --languages python,rust,javascript
```

Already-cloned grammars at the correct revision are skipped. Run this before building with `TSLP_LANGUAGES` or before adding a new language.

## `pin_vendors.py`

Updates `rev` fields in `language_definitions.json` by querying each grammar's upstream repo for its latest commit on the configured branch.

```bash
uv run scripts/pin_vendors.py
```

After running, review the diff in `sources/language_definitions.json`, then run `clone_vendors.py` and `task test` to verify nothing broke.

## `check_grammar_updates.py`

Reports grammars where the pinned `rev` is behind the upstream branch head, without making any changes.

```bash
uv run scripts/check_grammar_updates.py
```

Useful as a read-only audit before deciding which grammars to update.

## `generate_grammar_table.py`

Generates `docs/supported-languages.md` from `sources/language_definitions.json`.

```bash
# Write to docs/supported-languages.md
uv run scripts/generate_grammar_table.py

# Preview on stdout
uv run scripts/generate_grammar_table.py --stdout

# Write to a custom path
uv run scripts/generate_grammar_table.py --output /tmp/langs.md
```

The `docs:generate:languages` task in Taskfile.yml runs this automatically.

## `generate_readme.py`

Generates per-binding README files from Jinja2 templates in `scripts/readme_templates/` and configuration in `scripts/readme_config.yaml`.

```bash
# Generate all READMEs
uv run scripts/generate_readme.py

# Dry-run (preview diffs, no writes)
uv run scripts/generate_readme.py --dry-run

# Validate that checked-in READMEs match (used in CI)
uv run scripts/generate_readme.py --check
```

The `generate-readme`, `generate-readme:dry-run`, and `generate-readme:check` tasks wrap these flags.

## `lint_grammar_licenses.py`

Queries the GitHub License API for every grammar in `language_definitions.json` and fails if any uses a copyleft license (GPL, AGPL, LGPL, MPL, or similar). Results are cached locally.

```bash
uv run scripts/lint_grammar_licenses.py

# Refresh cached license data
uv run scripts/lint_grammar_licenses.py --update-cache

# Skip cache entirely
uv run scripts/lint_grammar_licenses.py --no-cache
```

Exit code 0 means all grammars use permissive licenses. The `lint:licenses` task runs this in CI.

## `sync_versions.py`

Reads the version string from `[workspace.package]` in the root `Cargo.toml` and writes it into every binding's package manifest:

- `pyproject.toml` (Python)
- `crates/ts-pack-node/package.json` (Node.js)
- `crates/ts-pack-elixir/mix.exs` (Elixir)
- `crates/ts-pack-java/pom.xml` (Java)
- Ruby gemspec
- WASM `Cargo.toml` and `package.json`

```bash
uv run scripts/sync_versions.py
```

Run this whenever you bump the workspace version before a release.

## CI scripts

`scripts/ci/` has three helper scripts used by GitHub Actions, not meant to be run manually:

| Script | Purpose |
|--------|---------|
| `ci/go/vendor-ffi.py` | Vendor the FFI headers for the Go binding CI |
| `ci/php/vendor-core.py` | Vendor the FFI library for the PHP binding CI |
| `ci/ruby/vendor-core.py` | Vendor the FFI library for the Ruby binding CI |
