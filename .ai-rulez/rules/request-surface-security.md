---
priority: high
---

# Request Surface Security

Guard every HTTP-facing change with the validation strategy captured in
`docs/adr/0003-validation-and-fixtures.md`: enforce cookie rules via
`testing_data/cookies/*.json`, headers/auth via `testing_data/headers/*.json`, and
CORS expectations via `testing_data/cors/*.json`. Strip secrets from logs and ensure
new handlers never bypass the existing validator layer before reaching business logic.
