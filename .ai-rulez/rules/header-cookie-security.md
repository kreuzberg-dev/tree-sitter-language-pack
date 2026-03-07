---
priority: high
---

# Header & Cookie Security

Authentication, header, and cookie code must enforce the scenarios captured in
testing_data/headers and testing_data/cookies—reject deviations from those schemas, add
explicit fixtures plus assertions in packages/python/tests/test_integration_query_params.py
for new header names or cookie attributes, and keep Secure/HttpOnly/SameSite defaults
intact.
