---
priority: medium
---

# Lint & Formatting Discipline

Before committing, run task lint to honor the Biome settings in biome.json for JS/TS,
format Rust with cargo fmt --manifest-path Cargo.toml configured by rustfmt.toml, and
sync Python dependencies with uv so pyproject.toml and uv.lock stay consistent—avoid
introducing divergent toolchains or unchecked formatting drift.
