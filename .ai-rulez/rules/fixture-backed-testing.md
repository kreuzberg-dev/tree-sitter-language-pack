---
priority: critical
---

# Fixture-Backed Testing

Every feature change must expand the Python-driven integration suite in
`packages/python/tests/` and keep the JSON fixtures under `testing_data/` in sync.
Prefer validating new scenarios by adding fixture files and asserting them in
`packages/python/tests/test_all_fixtures.py`. Run `task test` locally before merging
so the Rust, Python, and JavaScript checks that CI executes stay green.
