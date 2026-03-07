---
priority: high
description: "Spikard Workspace Architecture"
---

# Spikard Workspace Architecture

**Multi-crate Rust workspace · Layered binding architecture · Fixture-driven testing**

Rust Workspace: `crates/spikard/` (core), `crates/spikard-http/` (tower-http), `crates/spikard-cli/` (CLI), `crates/spikard-py/` (PyO3), `crates/spikard-node/` (napi-rs), `crates/spikard-rb/` (magnus), `crates/spikard-php/` (ext-php-rs).

Binding Principles: All middleware in Rust; bindings expose config APIs only. Language-neutral Handler trait: `Pin<Box<dyn Future<Output = HandlerResult> + Send>>`. Each binding wraps with Arc<dyn Handler>. No PyO3/napi-rs/magnus/ext-php-rs in core crates.

Python & Testing: Package scaffold `packages/python/spikard`; integration tests in `packages/python/tests/` with conftest.py. Shared fixtures `testing_data/` with schema.json per scenario. Fixture-driven: `testing_data/{headers,cookies,json_bodies,validation_errors,edge_cases}/`. Coverage: 95% Rust core, 80%+ Python/JS/Ruby/PHP.
