---
description: "CI/CD workflow reference — what each GitHub Actions workflow does, when it runs, and how publishing works."
---

# CI/CD reference

The project has 18 GitHub Actions workflows in `.github/workflows/`.

## Per-binding CI

Fifteen workflows, one per binding, run on push and pull request when the relevant paths change:

| Workflow | Binding | Triggered by |
|----------|---------|--------------|
| `ci-rust.yaml` | Rust core | `crates/ts-pack-core/**`, `Cargo.toml` |
| `ci-cli.yaml` | CLI | `crates/ts-pack-cli/**`, `crates/ts-pack-core/**` |
| `ci-python.yaml` | Python | `crates/ts-pack-python/**` |
| `ci-node.yaml` | Node.js | `crates/ts-pack-node/**` |
| `ci-java.yaml` | Java | `crates/ts-pack-java/**` |
| `ci-elixir.yaml` | Elixir | `crates/ts-pack-elixir/**` |
| `ci-ruby.yaml` | Ruby | `crates/ts-pack-ruby/**` |
| `ci-php.yaml` | PHP | `crates/ts-pack-php/**`, `packages/php/**` |
| `ci-go.yaml` | Go | `packages/go/**` |
| `ci-csharp.yaml` | C# | `packages/csharp/**` |
| `ci-wasm.yaml` | WebAssembly | `crates/ts-pack-wasm/**` |
| `ci-c.yaml` | C FFI | `crates/ts-pack-ffi/**` |
| `ci-docker.yaml` | Docker | `docker/**` |
| `ci-all-grammars.yaml` | All grammars | `sources/language_definitions.json`, `crates/ts-pack-core/**` |
| `ci-validate.yaml` | Cross-cutting | `crates/**`, `packages/**`, `scripts/**`, `e2e/**` |

`ci-all-grammars.yaml` is the most expensive workflow — it compiles and tests every grammar. It only runs on push to `main` or PRs that change the core or language definitions.

## Docs workflow

`docs.yaml` runs on push to `main` when docs files change, and can be triggered manually via `workflow_dispatch`. It builds the Zensical docs site and deploys it.

Triggers on changes to: `docs/**`, `zensical.toml`, `pyproject.toml`, `sources/language_definitions.json`, and `scripts/generate_grammar_table.py`.

## Publishing workflows

Both publish workflows are manual (`workflow_dispatch`) — they don't run automatically.

### `publish.yaml` — package releases

Takes a release tag (e.g. `v1.0.0`) and an optional `dry_run` flag. On a real run, it publishes to all registered package registries simultaneously.

### `publish-docker.yaml` — Docker image

Takes a release tag and optional `dry_run`. Builds the multi-arch image (amd64 + arm64) using `docker buildx` and pushes to `ghcr.io`.

## Adding CI for a new binding

When adding a new language binding, copy an existing per-binding workflow (e.g. `ci-python.yaml`) and adjust the `paths` trigger and test command. Keep the path filter tight so the workflow only runs when relevant files change.
