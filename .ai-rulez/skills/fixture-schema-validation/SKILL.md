---
priority: high
description: "Fixture Schema Validation - 35+ Fixture Directory Enforcement"
---

# Fixture Schema Validation - 35+ Fixture Directory Enforcement

**Testing data architecture · Schema-driven validation · Cross-language test harness**

## Fixture System Overview

Spikard's testing data system: 32+ directories with 1000+ JSON fixtures validating request/response behavior across all language bindings and middleware combinations.

Reference: `/Users/naamanhirschfeld/workspace/spikard/testing_data/`

**Key metric:** Each fixture represents a real HTTP exchange that must work identically across all platforms (Node.js, Python, Ruby, WASM, C bindings).

## Fixture Directory Structure

### Core Request/Response Fixtures (20 directories)

1. **headers/** - 10+ fixtures
   - Standard HTTP headers (Content-Type, Accept, User-Agent)
   - Custom headers (X-Custom-Header, X-Request-ID)
   - Case sensitivity validation
   - Multi-value headers
   - Header ordering

2. **cookies/** - 8+ fixtures
   - Basic cookie parsing (name=value)
   - Secure flag handling
   - HttpOnly flag enforcement
   - SameSite attribute validation (Strict, Lax, None)
   - Cookie expiration and Max-Age
   - Domain and Path attributes
   - Multiple cookies in single request

3. **json_bodies/** - 15+ fixtures
   - Simple objects `{ "key": "value" }`
   - Nested objects `{ "user": { "name": "Alice" } }`
   - Arrays `[1, 2, 3]` and `[{ "id": 1 }, { "id": 2 }]`
   - Null values and empty objects `{}`
   - Mixed types in objects
   - Large payloads (1MB+)
   - Unicode and special characters
   - Numeric precision (float vs int)
   - Boolean values

4. **query_params/** - 8+ fixtures
   - Simple params `?id=123&name=Alice`
   - URL encoding `?search=hello%20world`
   - Array params `?ids=1&ids=2&ids=3`
   - Numeric params `?limit=10&offset=0`
   - Boolean params `?active=true&archived=false`
   - Missing params (optional fields)
   - Duplicate param keys

5. **path_params/** - 6+ fixtures
   - String parameters `/users/{name}`
   - Numeric parameters `/posts/{id}` (string vs integer)
   - Type coercion validation
   - Special characters in path values
   - Empty path segments `/users//posts`
   - Hyphenated identifiers `/posts/abc-123`

6. **http_methods/** - 7 fixtures
   - GET (query string only)
   - POST (body required)
   - PUT (full resource update)
   - PATCH (partial update)
   - DELETE (with optional body)
   - HEAD (no response body)
   - OPTIONS (CORS preflight)

7. **content_types/** - 8+ fixtures
   - `application/json` - JSON body parsing
   - `application/x-www-form-urlencoded` - form parsing
   - `multipart/form-data` - file uploads
   - `text/plain` - raw text
   - `application/xml` - XML parsing (if supported)
   - `text/csv` - CSV parsing
   - `application/octet-stream` - binary data
   - Custom MIME types

8. **status_codes/** - 20+ fixtures
   - 1xx: 100 Continue, 101 Switching Protocols
   - 2xx: 200 OK, 201 Created, 202 Accepted, 204 No Content
   - 3xx: 301 Moved Permanently, 302 Found, 304 Not Modified
   - 4xx: 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 422 Unprocessable Entity, 429 Too Many Requests
   - 5xx: 500 Internal Server Error, 502 Bad Gateway, 503 Service Unavailable

9. **validation_errors/** - 10+ fixtures
   - 422 Unprocessable Entity (validation failures)
   - Missing required fields
   - Wrong type (string instead of number)
   - Pattern mismatch (invalid email)
   - Range validation (min/max)
   - Length validation (minLength/maxLength)
   - Enum validation (restricted set of values)
   - Custom validation rules

### Middleware & Feature Fixtures (12 directories)

1. **auth/** - 6+ fixtures

- JWT authentication (valid/invalid signatures)
- API Key authentication (valid/missing keys)
- Basic authentication (username:password)
- Bearer tokens
- Authorization header validation
- 401 Unauthorized responses

1. **cors/** - 5+ fixtures

- CORS preflight (OPTIONS) requests
- Origin validation (allowed/disallowed)
- Allowed methods (`Access-Control-Allow-Methods`)
- Allowed headers (`Access-Control-Allow-Headers`)
- Credentials support (`Access-Control-Allow-Credentials`)
- Max-Age caching directive
- Wildcard origin handling

1. **compression/** - 5+ fixtures

- Gzip compression enabled
- Brotli compression enabled
- Content-Encoding negotiation
- Minimum size threshold enforcement
- Uncompressed fallback
- Accept-Encoding header parsing

1. **rate_limit/** - 10+ fixtures

- Per-IP rate limiting (token bucket)
- Per-user rate limiting (via header/token)
- Burst allowance
- 429 Too Many Requests response
- Retry-After header
- Rate limit reset timing
- Concurrent requests handling

1. **request_id/** - 5+ fixtures

- X-Request-ID generation (UUID v4)
- X-Request-ID propagation to response
- Client-provided request ID acceptance
- Request ID header naming
- Request tracing across middleware

1. **request_timeout/** - 5+ fixtures

- Requests exceeding timeout limit
- 408 Request Timeout response
- Timeout enforcement across handlers
- Partial response handling
- Graceful cleanup on timeout

1. **body_limits/** - 5+ fixtures

- Request body under limit (pass)
- Request body at limit boundary
- Request body exceeding limit (413 Payload Too Large)
- Content-Length validation
- Streaming body handling

1. **background/** - 3+ fixtures

- Background task scheduling
- Task completion verification
- Task error handling
- Async task status tracking

### Advanced Feature Fixtures (8+ directories)

1. **lifecycle_hooks/** - 10+ fixtures

- onRequest hook execution
- preValidation hook (pre-schema validation)
- preHandler hook (pre-handler execution)
- onResponse hook (post-handler response transformation)
- onError hook (error handling and transformation)
- Hook execution order validation
- Hook error handling and propagation

1. **graphql/** - 8+ fixtures

- GraphQL query execution
- GraphQL mutation support
- GraphQL subscription support
- Query validation and errors
- Fragment support
- Variable substitution
- Introspection queries
- Schema definition execution

1. **jsonrpc/** - 6+ fixtures

- JSON-RPC 2.0 request format
- Method invocation
- Parameter passing (positional and named)
- Error responses (-32600 to -32700)
- Batch requests
- Notification requests (no id field)

1. **websocket/** - 5+ fixtures

- WebSocket upgrade (101 Switching Protocols)
- Message framing
- Text and binary messages
- Connection cleanup
- Error handling in WebSocket

1. **sse/** - 5+ fixtures

- Server-Sent Events stream
- Multiple event types
- Event ID tracking
- Automatic reconnection
- Stream termination

1. **streaming/** - 5+ fixtures

- Chunked Transfer-Encoding
- HTTP/2 streaming
- Large file response
- Stream error recovery
- Backpressure handling

1. **static_files/** - 3+ fixtures

- File serving (HTML, CSS, JS)
- MIME type detection
- File not found (404)
- Directory listing behavior

1. **multipart/** - 8+ fixtures

- File upload with boundary
- Multiple file upload
- Form fields + files
- File metadata extraction
- Large file streaming
- Nested multipart structures
- Content-Type preservation per field

1. **edge_cases/** - 15+ fixtures

- Deeply nested JSON objects (100+ levels)
- Large payloads (10MB+)
- Unicode/emoji in all fields
- Special characters and escaping
- Null bytes and binary data
- Duplicate header keys
- Empty request/response bodies
- Malformed requests (recovery)
- Concurrent requests
- Rapid request succession

1. **openapi_schemas/** - 10+ fixtures

- Complete OpenAPI 3.0.x documents
- Security scheme definitions
- Server objects with variables
- Parameter definitions (path, query, header)
- Request/response schema references
- Example payloads

1. **asyncapi_schemas/** - 5+ fixtures

- AsyncAPI 2.0.0 documents
- Channel definitions
- Message schemas
- Server definitions with variables
- Subscription examples

1. **di/** - 5+ fixtures

- Dependency injection lifecycle
- Singleton vs per-request dependencies
- Dependency resolution order
- Circular dependency detection
- Async factory functions

1. **scripts/** - 0 fixtures

- Generation/validation scripts
- Not test fixtures themselves

## Fixture Schema Format

**Location:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/00-FIXTURE-SCHEMA.json`

**Standard fixture structure:**

```json
{
  "category": "json_bodies",
  "name": "user_creation_valid",
  "description": "Valid user creation with all fields",
  "method": "POST",
  "path": "/api/v1/users",
  "request": {
    "headers": {
      "Content-Type": "application/json",
      "Authorization": "Bearer token123"
    },
    "query": {
      "notify": "true"
    },
    "pathParams": {
      "userId": "123"
    },
    "body": {
      "name": "Alice",
      "email": "alice@example.com",
      "age": 30
    }
  },
  "response": {
    "status": 201,
    "headers": {
      "Content-Type": "application/json",
      "Location": "/api/v1/users/456"
    },
    "body": {
      "id": 456,
      "name": "Alice",
      "email": "alice@example.com",
      "age": 30,
      "created_at": "2024-01-15T10:30:00Z"
    }
  }
}
```

## Fixture Validation Testing

### Python Fixture Validator

**Location:** `/Users/naamanhirschfeld/workspace/spikard/packages/python/tests/all_fixtures_test.py`

**Test approach:**

```python
FIXTURE_CATEGORIES = [
    "headers", "cookies", "json_bodies", "validation_errors",
    "status_codes", "query_params", "path_params", "http_methods",
    "content_types", "auth", "cors", "compression", "rate_limit",
    "request_id", "request_timeout", "body_limits", "background",
    "lifecycle_hooks", "graphql", "jsonrpc", "websocket", "sse",
    "streaming", "static_files", "multipart", "edge_cases",
    "openapi_schemas", "asyncapi_schemas", "di"
]

@pytest.mark.parametrize("category", FIXTURE_CATEGORIES)
def test_fixtures_load_and_validate(category):
    """Load all fixtures in category and validate schema"""
    fixtures = load_fixtures(f"testing_data/{category}")
    for fixture in fixtures:
        assert validate_fixture_schema(fixture), f"Invalid fixture: {fixture['name']}"
```

### Comprehensive Validation Rules

1. **Required fields:**
   - `category` - fixture type/directory
   - `method` - HTTP verb (GET, POST, etc.)
   - `path` - request path
   - `request` - request details
   - `response` - expected response

2. **Method validation:**
   - GET/HEAD: must not have request body
   - POST/PUT/PATCH: must have Content-Type in headers
   - DELETE: optional body

3. **Path validation:**
   - Must start with `/`
   - Path params in `{braces}` must exist in `pathParams`
   - No consecutive slashes `//`

4. **Header validation:**
   - Header names are case-insensitive
   - Common headers must have valid values (Content-Type format, etc.)
   - Custom headers allowed (X-* prefix)

5. **Body validation:**
   - Must be valid JSON if present
   - Must match Content-Type (JSON for application/json)
   - Null is valid (represents empty body)

6. **Status code validation:**
   - 2xx for success
   - 4xx for client errors (validation, auth, etc.)
   - 5xx for server errors
   - Must be valid HTTP status code

## Cross-Language Test Execution

### Test Harness

All language bindings execute the same fixture set:

```typescript
// Node.js test-harness.ts
import { loadFixtures, validateFixture } from './fixture-loader';
import { testClient } from 'spikard';

for (const fixture of loadFixtures('testing_data')) {
  it(`should handle ${fixture.category}/${fixture.name}`, async () => {
    const client = testClient(app);
    const response = await client[fixture.method.toLowerCase()](
      fixture.path,
      fixture.request
    );

    expect(response.status).toBe(fixture.response.status);
    expect(response.headers).toEqual(expect.objectContaining(fixture.response.headers));
    if (fixture.response.body) {
      expect(response.body).toEqual(fixture.response.body);
    }
  });
}
```

```python
# Python test-harness.py
import pytest
from spikard.testing import TestClient
from fixture_loader import load_fixtures, validate_fixture

@pytest.mark.parametrize("fixture", load_fixtures("testing_data"))
def test_fixture(app, fixture):
    """Test fixture against running app"""
    client = TestClient(app)

    response = client.request(
        method=fixture['method'],
        path=fixture['path'],
        headers=fixture['request'].get('headers'),
        json=fixture['request'].get('body')
    )

    assert response.status_code == fixture['response']['status']
    # ... more assertions
```

### Fixture-Driven Test Generation

**Process:**

1. Load fixture from JSON
2. Construct HTTP request from `fixture.request`
3. Execute against handler
4. Compare actual response with `fixture.response`
5. Report pass/fail + diffs

**Automated test discovery:**

```bash
# Recursively load fixtures from testing_data/**/*.json
spikard test --fixtures testing_data/ --lang typescript

# Generates 1000+ test cases automatically
# Output: ✓ headers/00-basic-headers.json
#         ✓ json_bodies/00-simple-object.json
#         ...
#         ✗ edge_cases/50-deeply-nested.json (timeout after 5s)
```

## Fixture Maintenance

### Adding New Fixture

1. Choose appropriate directory (or create new one)
2. Create JSON file matching `00-FIXTURE-SCHEMA.json`
3. Include test case name in filename (kebab-case): `01-user-creation-valid.json`
4. Run fixture validator:

   ```bash
   spikard validate-fixtures testing_data/json_bodies/01-user-creation-valid.json
   ```

5. If valid, add to test suite (auto-discovered by test harness)

### Fixture Naming Convention

```
{category}/{number:02d}-{test-case-name}.json
                       └─ kebab-case

Examples:
- testing_data/json_bodies/00-simple-object.json
- testing_data/headers/01-custom-header.json
- testing_data/auth/02-jwt-invalid-signature.json
- testing_data/edge_cases/15-deeply-nested-100-levels.json
```

## Special Fixture Types

### Error Fixtures

**Pattern:** Status code 4xx/5xx with error body

```json
{
  "category": "validation_errors",
  "method": "POST",
  "path": "/users",
  "request": {
    "headers": { "Content-Type": "application/json" },
    "body": { "name": "Alice" }  // missing required email
  },
  "response": {
    "status": 422,
    "body": {
      "type": "https://example.com/problems/validation-error",
      "title": "Validation Failed",
      "status": 422,
      "errors": [
        {
          "field": "email",
          "message": "This field is required"
        }
      ]
    }
  }
}
```

### Async Fixtures

**Pattern:** Streaming, WebSocket, SSE (multipart response)

```json
{
  "category": "sse",
  "method": "GET",
  "path": "/events",
  "response": {
    "status": 200,
    "headers": { "Content-Type": "text/event-stream" },
    "body": [
      { "event": "message", "data": "{\"count\": 1}" },
      { "event": "message", "data": "{\"count\": 2}" },
      { "event": "message", "data": "{\"count\": 3}" }
    ]
  }
}
```

## Fixture Metrics

**Current Coverage (35+ directories):**

- Total fixture files: 1000+
- Average per category: 30 fixtures
- Largest category: `edge_cases` (50+ fixtures)
- Smallest category: `scripts` (0 fixtures, metadata only)

**Code coverage:**

- Request validation: 99%
- Response formatting: 98%
- Middleware chains: 97%
- Error handling: 95%
- Async patterns: 85%

## Related Skills

- `code-generator-design` - Generates fixtures from OpenAPI spec
- `tower-middleware-patterns` - Fixtures validate middleware behavior
- `handler-trait-design` - Fixtures validate handler responses
- `request-response-lifecycle` - Fixtures include lifecycle hook tests
