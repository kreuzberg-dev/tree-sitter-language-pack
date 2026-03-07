---
priority: critical
---

# Fixture-Aligned Error Handling

Keep every fallible path in the Rust workspace (`crates/spikard`, `crates/spikard-http`,
bindings crates) returning the structured payload described in
`testing_data/validation_errors/schema.json`. Reuse the shared error constructor so
Python (`crates/spikard-py`), Node (`crates/spikard-node`), Ruby (`crates/spikard-rb`),
and PHP (`crates/spikard-php`) adapters raise translated host-language errors while
preserving the same JSON body that `packages/python/tests/test_all_fixtures.py` asserts on.
