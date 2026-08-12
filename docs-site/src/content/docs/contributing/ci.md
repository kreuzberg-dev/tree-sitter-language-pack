---
title: "CI/CD reference"
description: "CI/CD workflow reference — what each GitHub Actions workflow does, when it runs, and how publishing works."
---

The project has 16 GitHub Actions workflows in `.github/workflows/`.

## Overview

| Workflow                | Purpose                                 | Trigger                 |
| ----------------------- | --------------------------------------- | ----------------------- |
| `ci.yaml`               | Main CI — builds and tests all bindings | push/PR to `main`       |
| `ci-rust.yaml`          | Rust core tests and clippy              | push/PR to `main`       |
| `ci-cli.yaml`           | CLI-specific tests                      | push/PR to `main`       |
| `ci-docker.yaml`        | Docker image build and tests            | push/PR to `main`       |
| `ci-e2e.yaml`           | Generated cross-language e2e suites     | push/PR to `main`       |
| `ci-dart.yaml`          | Dart / Flutter binding tests            | push/PR to `main`       |
| `ci-swift.yaml`         | Swift binding tests                     | push/PR to `main`       |
| `ci-zig.yaml`           | Zig binding tests                       | push/PR to `main`       |
| `ci-mobile.yaml`        | Kotlin Android binding tests            | push/PR to `main`       |
| `ci-plugin.yaml`        | Coding-agent plugin tests               | push/PR to `main`       |
| `docs.yaml`             | Build and deploy documentation          | push/PR to `main`, manual |
| `publish.yaml`          | Publish packages to all registries      | manual, release         |
| `publish-docker.yaml`   | Build and push Docker image             | manual, release         |
| `publish-pubdev.yaml`   | Publish the Dart package to pub.dev     | manual, release         |
| `validate-issues.yml`   | Validate issue format                   | issue opened/edited     |
| `validate-pr.yml`       | Validate PR format                      | PR opened/edited/synced |

---

## CI workflows

### `ci.yaml` — main CI

Runs on push to `main` and pull requests when relevant paths change:
`crates/**`, `packages/**`, `e2e/**`, `fixtures/**`, `sources/**`, `scripts/**`,
`docs-site/src/snippets/**`, `docs-site/src/content/docs/reference/**`, `.task/**`,
`Cargo.toml`, `Cargo.lock`, `Taskfile.yml`, `alef.toml`, `rust-toolchain.toml`,
`pyproject.toml`, and the JS workspace manifests.

This is the primary workflow that builds and tests all language bindings.

### `ci-cli.yaml` — CLI

Runs on push to `main` and pull requests when CLI or core paths change:
`crates/ts-pack-cli/**`, `crates/ts-pack-core/**`, `test_apps/cli/**`.

### `ci-docker.yaml` — Docker

Runs on push to `main` and pull requests when Docker or core paths change:
`docker/**`, `crates/ts-pack-core/**`, `crates/ts-pack-cli/**`.

---

## Docs workflow

`docs.yaml` runs on push and pull requests to `main` when docs files change, and you can also trigger it manually via `workflow_dispatch`. It builds the docs site and deploys it (deploy only on push to `main`).

Triggers on changes to: `docs-site/**`, `alef.toml`, and `.github/workflows/docs.yaml`.

---

## Publishing workflows

The publish workflows run automatically on a GitHub release event, and you can also trigger them manually via `workflow_dispatch`.

### `publish.yaml` — package releases

Takes a release tag (for example `vX.Y.Z`), an optional `dry_run` flag, and an optional `targets` list (comma-separated, defaults to `all`). On a real run, it publishes to all registered package registries simultaneously.

### `publish-docker.yaml` — Docker image

Takes a release tag and optional `dry_run`. Builds the multi-arch image (amd64 + arm64) using `docker buildx` and pushes to `ghcr.io`.

### `publish-pubdev.yaml` — Dart package

Publishes the Dart package to pub.dev. Split out from `publish.yaml` because pub.dev uses its own OIDC-based authentication flow.

---

## Validation workflows

### `validate-issues.yml`

Validates the format of newly opened or edited issues using a reusable workflow from `xberg-io/actions`.

### `validate-pr.yml`

Validates the format of pull requests when opened, edited, or synchronized using a reusable workflow from `xberg-io/actions`.
