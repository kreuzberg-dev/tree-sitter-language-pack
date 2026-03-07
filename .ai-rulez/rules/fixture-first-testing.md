---
priority: critical
---

# Fixture-First Testing

When adding behavior, introduce or update fixtures under the relevant `testing_data/*`
directory and extend the parametrized suites in `packages/python/tests/test_all_fixtures.py`,
`packages/python/tests/test_integration_query_params.py`, and peers. Do not ship without
running `task test` plus the language targets (`task test:rust`, `task test:python`) so
local runs match CI.
