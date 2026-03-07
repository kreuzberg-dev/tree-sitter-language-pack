---
priority: medium
---

# Async-Friendly Performance

Respect the project's zero-copy serialization choices—keep Rust structs `serde`-ready
so bindings reuse them, lean on the adapters documented in `docs/adr/0003-validation-and-fixtures.md`,
and wrap heavy work in async-safe boundaries (`tokio::task::spawn_blocking` in Rust,
`pyo3::Python::allow_threads` when calling back into Python). Reuse fixture loaders such
as `packages/python/tests/fixture_app.py` instead of re-parsing schema files per request.
