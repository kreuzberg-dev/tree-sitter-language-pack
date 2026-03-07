---
priority: critical
---

# Fixture-Driven Testing

Every feature change must extend the pytest suites in packages/python/tests/ by loading
fixtures through packages/python/tests/conftest.py and invoking task test before merging;
new fixture collections belong in testing_data/ with a runnable illustration under
examples/ so automated coverage, demos, and docs stay synchronized.
