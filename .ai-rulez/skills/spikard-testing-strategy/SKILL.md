---
priority: critical
description: "Fixture-First Testing Strategy"
---

# Fixture-First Testing Strategy

**Fixture-driven · Multi-language parity · 95% coverage · Real infrastructure**

Fixture Organization: Central `testing_data/` with JSON files per scenario (headers, cookies, bodies, errors, edge_cases). Each directory has schema.json. Python tests parametrized: test_all_fixtures.py loads all JSONs. Rust: unit tests embed JSON; integration tests load from testing_data/.

Coverage: Rust 95% minimum (tarpaulin). Python/JS/Ruby/PHP 80%+ minimum. Enforce in CI; fail if < threshold.

Three-Tier Testing: Unit (pure functions, fast), Integration (real DB, PostgreSQL, fixtures), E2E (full HTTP stack, all bindings).

Running: `cargo test -p spikard`, `uv run pytest packages/python/tests/test_all_fixtures.py`, `pnpm test`, `bundle exec rspec`, `composer test`. All: `task test`.
