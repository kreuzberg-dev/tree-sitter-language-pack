---
priority: medium
---

# Workspace Separation

Keep language-neutral logic inside `crates/spikard/src` and limit each binding crate
(`spikard-py`, `spikard-node`, `spikard-rb`, `spikard-php`) to thin adapters over that
core. When introducing new modules, register them in the relevant `Cargo.toml`, mirror
usage in `examples/`, and avoid duplicating business rules across bindings.
