---
priority: high
---

# HTTP Input Validation

Handlers under `crates/spikard-http/src` must validate headers, cookies, and payloads
against the schemas in `testing_data/headers`, `testing_data/cookies`, and
`testing_data/json_bodies`. Reject unexpected or malformed values with structured
errors returned to the caller, and cover each guard with an integration test tied to
the corresponding fixture set.
