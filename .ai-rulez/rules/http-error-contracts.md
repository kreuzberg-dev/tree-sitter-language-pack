---
priority: high
---

# HTTP Error Contracts

When updating handlers in crates/spikard-http, translate domain failures into the JSON
payloads maintained under testing_data/status_codes and testing_data/validation_errors;
add the matching fixture files and assertions in packages/python/tests/test_all_fixtures.py
or the focused integration suites, and keep every testing_data/**/schema.json aligned
with the new variants.
