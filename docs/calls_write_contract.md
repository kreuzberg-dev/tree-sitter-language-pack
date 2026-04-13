# CALLS Write Contract

This document defines what is allowed into the canonical Neo4j `:CALLS` write path.

## Canonical Rule

Canonical `:CALLS` edges are written only for exact internal symbol resolutions.

By the time a row reaches `write_calls_by_id(...)`, it must already be:

- an internal project-local call
- resolved to an exact `callee_id`
- accepted by current policy such as `TS_PACK_INCLUDE_INTRA_FILE_CALLS`

The canonical write path must not perform best-effort name lookup inside Neo4j.

## Resolver Outcomes

`call_resolution.rs` resolves each `CallRef` into one of:

- `ResolvedInternal(callee_id, stage)`
- `Filtered(reason)`
- `Unresolved(reason)`

Only `ResolvedInternal(...)` is converted into `SymbolCallRow` for canonical graph writes.

## What Does Not Reach Canonical `:CALLS`

These categories do not become canonical direct-call edges:

- obvious constructor noise such as Rust `Ok`, `Err`, `Some`, `None`
- clearly external Rust scoped calls
- clearly external Go scoped calls
- unresolved calls that do not have an exact internal callee id

Those rows remain visible only through prep/debug telemetry.

## Why

The old fallback path wrote unresolved calls by matching `Node {project_id, name}` in Neo4j.
That was ambiguous and made the write path harder to reason about.

The current contract trades recall for precision:

- graph edges are trustworthy
- unresolved traffic is measurable
- missing graph signal is explicit instead of silently guessed

## Current Measured Gap

With canonical-only `:CALLS`, the remaining missing signal is concentrated in a few buckets:

- `rust:plain:norecv`
- `rust:scoped:norecv`
- `python:plain:norecv`
- `python:member:recv`
- `go:scoped:recv`

These are not Neo4j write-path mysteries. They are upstream extraction/resolution gaps or deliberate policy exclusions.

## Next Correct Additions

New graph signal should only be added when it can be represented precisely.

Examples:

- exact Rust `Self::...` resolution from caller type context
- exact Python member-call resolution from module/alias/type context
- explicit external-call graph families for known external package/module calls

Avoid adding generic fallback name matching back into the canonical write path.
