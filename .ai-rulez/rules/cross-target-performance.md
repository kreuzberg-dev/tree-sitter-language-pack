---
priority: medium
---

# Cross-Target Performance

Consolidate heavy computation inside the shared Rust core (crates/spikard) and expose
thin bindings in crates/spikard-py, packages/python, crates/spikard-node, crates/spikard-rb,
and crates/spikard-php; stress-test large or deeply nested payloads with testing_data/edge_cases
and verify optimized builds via task build:rust, task build:py, task build:node, task build:ruby,
and task build:php.
