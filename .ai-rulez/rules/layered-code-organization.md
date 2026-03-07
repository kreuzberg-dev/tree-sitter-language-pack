---
priority: high
---

# Layered Code Organization

Implement cross-cutting logic in `crates/spikard/src` and expose it through thin adapters
in `crates/spikard-http`, `crates/spikard-py`, `crates/spikard-node`, `crates/spikard-rb`,
and `crates/spikard-php`. Keep build metadata confined to each binding's manifest
(`pyproject.toml`, `crates/spikard-node/package.json`, `composer.json`) and register
new workflows in `Taskfile.yaml` so `task build`/`task lint` continue to orchestrate
the monorepo.
