---
priority: high
description: "Tower Middleware Patterns - Spikard HTTP Stack"
---

# Tower Middleware Patterns - Spikard HTTP Stack

**Compression · Rate limiting · Auth · CORS · Request lifecycle management**

## Architecture Overview

Spikard's HTTP middleware stack is built on Axum + Tower, composing pure-Rust middleware into a configurable pipeline. All middleware can be enabled/disabled via `ServerConfig` during initialization.

Reference: `/Users/naamanhirschfeld/workspace/spikard/crates/spikard-http/src/middleware/`

## Core Middleware Layers

### 1. Compression Middleware

- **Enabled:** Via `CompressionConfig { gzip, brotli, min_size, quality }`
- **Implementation:** Axum's built-in `axum::middleware::compression`
- **Behavior:**
    - Gzip compression (default enabled, quality=6)
    - Brotli compression (default enabled)
    - Minimum body size threshold (default 1024 bytes)
- **Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/compression/`
- **Node Binding:** Extracted via `extract_server_config()` in `/Users/naamanhirschfeld/workspace/spikard/packages/node/native/src/lib.rs:83-95`
- **Python Binding:** Configured in `spikard/config.py:CompressionConfig`

### 2. Rate Limiting Middleware

- **Enabled:** Via `RateLimitConfig { per_second, burst, ip_based }`
- **Behavior:**
    - Token bucket algorithm with configurable burst
    - IP-based or header-based rate limiting
    - Returns 429 Too Many Requests when limit exceeded
    - Per-second throughput control
- **Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/rate_limit/` (10+ fixtures)
- **Configuration Path:** `crates/spikard-http/src/server/rate_limit_middleware.rs`
- **Default:** Disabled (optional configuration)

### 3. Authentication Middleware

- **JWT Auth:** Via `JwtConfig { secret, algorithm, validate_claims }`
    - Validates Authorization: Bearer tokens
    - Signature verification using configurable algorithm
    - Claim validation against request headers
    - Fixtures: `/Users/naamanhirschfeld/workspace/spikard/testing_data/auth/` (JWT, API key, basic)
- **API Key Auth:** Via `ApiKeyConfig { header_name, key }`
    - Supports custom header names (default: "X-API-Key")
    - Returns 401 Unauthorized for missing/invalid keys
- **Implementation:** Tower layer composition in `server/auth_middleware.rs`
- **Error Handling:** Returns 401 with `WWW-Authenticate` header

### 4. CORS Middleware

- **Enabled:** Via `CorsConfig { allowed_origins, allowed_methods, allowed_headers, max_age }`
- **Behavior:**
    - Preflight request handling (OPTIONS)
    - Origin validation against allowlist
    - Custom header allowlisting
    - Credentials support
    - Max-Age caching for preflight
- **Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/cors/` (origin validation, preflight, headers)
- **Implementation:** Tower CORS middleware composition
- **Default:** Open CORS (all origins allowed)

### 5. Request ID Middleware

- **Enabled:** Via `ServerConfig::enable_request_id: bool`
- **Behavior:**
    - Generates X-Request-ID header if not present
    - UUID v4 format (128-bit random)
    - Attached to all logs for request tracing
    - Propagated to response headers
- **Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/request_id/` (5+ fixtures)
- **Default:** Enabled
- **Integration:** Works with `RequestData::request_id` field

### 6. Request Timeout Middleware

- **Enabled:** Via `ServerConfig::request_timeout: u64` (milliseconds)
- **Behavior:**
    - Cancels long-running handlers
    - Returns 408 Request Timeout
    - Affects all routes uniformly
- **Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/request_timeout/`
- **Default:** None (unlimited)

### 7. Content-Type Validation Middleware

- **Location:** `crates/spikard-http/src/middleware/mod.rs`
- **Function:** `validate_content_type_middleware()`
- **Behavior:**
    - Validates Content-Type header for POST/PUT/PATCH
    - Transforms multipart/form-data to JSON
    - Parses application/x-www-form-urlencoded to JSON
    - Validates JSON syntax when expecting JSON bodies
    - Enforces Content-Length for body requests
- **Fixtures:**
    - Multipart: `testing_data/multipart/` (file uploads, mixed fields)
    - Form-urlencoded: Tested inline
    - JSON validation: `testing_data/json_bodies/` (validation errors → 400)
- **Extension Pattern:** Uses Axum request extensions:
    - `PreReadBody` - already-read body bytes
    - `PreParsedJson` - pre-parsed JSON value (avoids double parsing)

## Middleware Stack Order (Left-to-Right Execution)

```
Request
  ↓
Compression (response)
  ↓
Rate Limit (per-IP token bucket)
  ↓
Timeout (cancel after N ms)
  ↓
Request ID (generate UUID if missing)
  ↓
Auth (JWT/API Key validation)
  ↓
CORS (preflight + origin validation)
  ↓
Content-Type Validation (multipart/form parsing)
  ↓
Handler Execution
  ↓
Response
```

## Language Bindings Integration

### Node.js (`packages/node/native/src/lib.rs`)

- Configuration extraction: `extract_server_config()` lines 60-200+
- Maps JavaScript config object to Rust `ServerConfig`
- All middleware fields directly correspond to JavaScript API
- Example:

  ```typescript
  const app = new Spikard({
    port: 8000,
    compression: { gzip: true, brotli: true, minSize: 1024 },
    rateLimit: { perSecond: 100, burst: 20, ipBased: true },
    auth: { type: 'jwt', secret: 'xyz...' },
  });
  ```

### Python (`packages/python/spikard/config.py`)

- All middleware configs as dataclass-style objects
- Type hints for IDE support
- Validation of numeric ranges
- Example:

  ```python
  app = Spikard(
      compression=CompressionConfig(gzip=True, brotli=True, min_size=1024),
      rate_limit=RateLimitConfig(per_second=100, burst=20, ip_based=True),
  )
  ```

### Ruby (`packages/ruby/lib/spikard/config.rb`)

- Hash-based configuration
- Automatic conversion to Rust types

## Testing Patterns

### Unit Tests

- Location: `crates/spikard-http/src/middleware/mod.rs` lines 293-560
- Test coverage:
    - Route info creation
    - Content-Length validation (smaller/larger than actual)
    - GET/DELETE (no body validation)
    - POST/PUT/PATCH (body validation required)
    - Multipart boundary parsing (minimal, with numbers, special chars)
    - JSON parsing (valid/invalid, empty objects)
    - MIME type parsing and validation

### Integration Tests

- Run entire middleware stack end-to-end
- Fixture-driven validation: load from `testing_data/*/` directories
- Test request → middleware → handler → response pipeline
- Verify header preservation and transformation

### Fixture-Based Testing

All middleware behavior validated against fixtures in `testing_data/`:

- `compression/` - 5+ fixtures testing gzip/brotli/size thresholds
- `rate_limit/` - 10+ fixtures testing token bucket, burst
- `auth/` - JWT, API key, basic auth failures
- `cors/` - preflight requests, origin validation
- `request_id/` - UUID generation, propagation
- `headers/` - header case sensitivity, custom headers
- `content_types/` - JSON/multipart/form validation

## Common Patterns

### Adding New Middleware

1. Create module in `crates/spikard-http/src/middleware/`
2. Implement as Axum middleware function:

   ```rust
   pub async fn my_middleware(
       State(config): State<MyConfig>,
       request: Request,
       next: Next,
   ) -> Result<Response, MyError>
   ```

3. Add to `ServerConfig` struct
4. Layer into router in `server/mod.rs`
5. Extract config in language bindings
6. Add fixtures to `testing_data/`

### Accessing Middleware State in Handlers

Middleware state injected via `RequestData`:

```rust
impl Handler for MyHandler {
    async fn handle(&self, req: RequestData) -> HandlerResult {
        let request_id = req.request_id.clone();
        let auth_claims = req.extensions.get::<Claims>();
    }
}
```

### Error Responses

All middleware errors follow problem+json format (`application/problem+json`):

```json
{
  "type": "https://example.com/problems/rate-limit-exceeded",
  "title": "Too Many Requests",
  "status": 429,
  "detail": "Rate limit exceeded: 100 requests/sec"
}
```

## Performance Considerations

- **Zero-cost abstractions:** Disabled middleware adds no overhead
- **Parallel resolution:** Rate limit, auth, CORS evaluated in parallel where possible
- **Early termination:** Middleware can short-circuit with error responses
- **Caching:** Request IDs, auth tokens cached in request extensions
- **Streaming:** Compression applied transparently to streaming responses

## Related Skills

- `request-response-lifecycle` - Request/response hooks around middleware
- `async-runtime-integration` - Tokio coordination with middleware
- `handler-trait-design` - Handler receives RequestData from middleware output
