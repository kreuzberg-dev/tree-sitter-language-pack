---
priority: high
---

# Workspace Organization

Place reusable domain types and logic in crates/spikard/src/ and keep feature-specific
glue isolated within sibling crates (spikard-http, spikard-cli, bindings); mirror module
changes across Cargo manifests and refresh the relevant docs/adr/* notes whenever the
layering or routing changes.
