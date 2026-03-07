---
priority: medium
---

# Consistent Tooling

Honor the repo formatters and linters before committing: run `cargo fmt` (configured by
`rustfmt.toml`), rely on Biome per `biome.json` for JavaScript/TypeScript, and manage
Python tooling through `uv` (`uv.lock`). Invoke them via `task lint` so local changes
match the CI configuration.
