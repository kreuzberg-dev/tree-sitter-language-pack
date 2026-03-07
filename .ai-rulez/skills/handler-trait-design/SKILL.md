---
priority: high
description: "Handler Trait Design - Language-Agnostic HTTP Handlers"
---

# Handler Trait Design - Language-Agnostic HTTP Handlers

**Cross-language trait · Request/response conversion · Language binding patterns**

## Architecture Overview

Spikard's core design: a single Rust `Handler` trait implemented by all language bindings. This enables truly language-agnostic HTTP routing while maintaining type safety and performance.

Reference: `/Users/naamanhirschfeld/workspace/spikard/crates/spikard-http/src/handler_trait.rs`

## Core Handler Trait

```rust
pub trait Handler: Send + Sync {
    fn handle(
        &self,
        req: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + 'a>>;
}

pub type HandlerResult = Result<HandlerResponse, (StatusCode, String)>;

pub struct RequestData {
    pub method: Method,
    pub path: String,
    pub query: QueryParams,
    pub headers: HeaderMap,
    pub body: Option<serde_json::Value>,
    pub request_id: String,
    pub extensions: Extensions,
}

pub struct HandlerResponse {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}
```

## Language-Specific Implementations

### Node.js Handler (`packages/node/native/src/handler.rs`)

**Pattern:** ThreadsafeFunction bridge to JavaScript

- **Location:** `packages/node/native/src/handler.rs` lines 19-130
- **Implementation:**

  ```rust
  pub struct NodeHandler {
      handler_name: String,
      handler_fn: Arc<ThreadsafeFunction<
          String,  // JSON-serialized RequestData
          Promise<HandlerReturnValue>,
          Vec<String>,
          napi::Status,
          false
      >>,
  }
  ```

- **Serialization:**
  1. Rust `RequestData` → JSON string via serde_json
  2. Call JavaScript handler with JSON string
  3. JavaScript returns Promise<response_object>
  4. Parse response back to `HandlerResponse`
- **Response Interpretation** (lines 38-80):
    - Accepts JavaScript object with `status`, `statusCode`, `headers`, `body`
    - Flexible status field (accepts both `status` and `statusCode`)
    - Headers converted to HashMap<String, String>
    - Null body vs empty object handling
- **Error Handling:**
    - ThreadsafeFunction call errors → 500 Internal Server Error
    - Response parsing errors → 500 Internal Server Error
    - JavaScript exceptions → 500 with error message

### Python Handler (`packages/python/spikard/app.py`)

**Pattern:** PyO3 async bridge with `pyo3_async_runtimes`

- **Location:** `crates/spikard-py/src/handler.rs`
- **Implementation:**
  1. Wraps Python async function in `PythonHandler` struct
  2. Uses `PyO3` to call Python from Rust async context
  3. Supports both sync and async Python handlers
  4. Automatically resolves coroutines
- **Serialization:**
    - Python dict ↔ Rust JSON via serde
    - Automatic type coercion (str, int, list, dict)
    - UUID/datetime → ISO 8601 strings
    - Decimal → float conversion
- **Request Binding:**

  ```python
  @app.get('/users/{id}')
  async def get_user(request, id: int):
      return {
          'status': 200,
          'body': {'user_id': id}
      }
  ```

- **Special Fields:**
    - `request.method` - HTTP method (GET, POST, etc.)
    - `request.path` - matched route path
    - `request.query` - QueryParams object
    - `request.headers` - case-insensitive HeaderMap
    - `request.body` - parsed JSON or None
- **Error Handling:**
    - Python exceptions → 500 Internal Server Error
    - Type mismatches → validation error response
    - Missing required fields → 400 Bad Request

### Ruby Handler (`packages/ruby/lib/spikard.rb`)

**Pattern:** Fiddle FFI bridge to native module

- **Implementation:**
  1. Ruby Proc/Lambda wrapped in handler object
  2. FFI calls to `spikard-rb` Rust crate
  3. JSON serialization for request/response
- **Request Binding:**

  ```ruby
  app.get '/items' do |request|
    {
      status: 200,
      headers: { 'Content-Type' => 'application/json' },
      body: { items: [] }
    }
  end
  ```

## Request/Response Data Structures

### RequestData

**Complete structure in Rust:**

```rust
pub struct RequestData {
    /// HTTP method (GET, POST, PUT, PATCH, DELETE, etc.)
    pub method: Method,

    /// Matched route path (e.g., "/users/{id}")
    pub path: String,

    /// Query parameters parsed from URL
    pub query: QueryParams,

    /// HTTP headers (case-insensitive)
    pub headers: HeaderMap,

    /// Parsed request body (JSON)
    /// None for GET/DELETE or empty bodies
    pub body: Option<serde_json::Value>,

    /// Unique request ID for tracing
    pub request_id: String,

    /// Request extensions (auth claims, middleware state)
    pub extensions: Extensions,
}
```

**Serialized to JSON in language bindings:**

```json
{
  "method": "POST",
  "path": "/users/123",
  "query": { "filter": "active" },
  "headers": {
    "content-type": "application/json",
    "authorization": "Bearer token123"
  },
  "body": { "name": "Alice", "email": "alice@example.com" },
  "requestId": "550e8400-e29b-41d4-a716-446655440000",
  "extensions": {
    "auth": { "user_id": 123 }
  }
}
```

### HandlerResponse

**Complete structure:**

```rust
pub struct HandlerResponse {
    /// HTTP status code (200, 404, 500, etc.)
    pub status: StatusCode,

    /// Response headers (will be merged with middleware-added headers)
    pub headers: HashMap<String, String>,

    /// Response body (JSON)
    /// None for 204 No Content or empty responses
    pub body: Option<serde_json::Value>,
}
```

**Expected from language bindings:**

```json
{
  "status": 200,
  "headers": {
    "content-type": "application/json",
    "x-custom": "value"
  },
  "body": { "id": 123, "name": "Alice" }
}
```

**Alternative formats accepted:**

- `statusCode` instead of `status`
- Missing fields → defaults (status=200, empty headers, null body)
- `body: null` or absent → 204 No Content

## Handler Registration & Routing

### HTTP Method Decorators

All bindings use method-specific decorators/functions:

- **Node.js:** `get()`, `post()`, `put()`, `patch()`, `delete()`, `head()`, `options()`

  ```typescript
  import { Spikard, get, post } from 'spikard';

  get('/users/:id')(async function(request) {
    return { status: 200, body: {...} };
  });
  ```

- **Python:** `@app.get()`, `@app.post()`, etc.

  ```python
  @app.get('/users/{id}')
  async def get_user(request):
      return {'status': 200, 'body': {...}}
  ```

- **Ruby:** `app.get()`, `app.post()`, etc.

  ```ruby
  app.get('/users/{id}') do |request|
    { status: 200, body: {...} }
  end
  ```

### Route Matching

Routes are registered as `Vec<(Route, Arc<dyn Handler>)>` in the HTTP server:

```rust
pub struct Route {
    pub method: Method,
    pub path: String,  // e.g., "/users/{id}/posts/{post_id}"
}
```

Path parameters extracted via regex matching:

- `:param` syntax in Node.js → `{param}` in Rust
- `{param}` syntax in Python/Ruby → preserved
- Colon-style parameters converted to Rust regex patterns
- Type coercion via `ParameterValidator`

## Special Handler Types

### Streaming Handlers (SSE)

**Location:** `crates/spikard-http/src/sse.rs`

- Return `HandlerResponse` with `Content-Type: text/event-stream`
- Body contains SSE event stream
- Client receives streaming updates without closing connection
- Fixtures: `testing_data/sse/` (5+ event types, reconnection)

### WebSocket Handlers

**Location:** `crates/spikard-http/src/websocket.rs`

- Not yet callable from language bindings (Rust-only)
- Future: WebSocket trait for bidirectional communication
- Separate from Handler trait (different protocol)

### GraphQL Handlers

**Location:** `crates/spikard-graphql/`

- Uses standard Handler trait
- Accepts JSON body with `query`, `variables`, `operationName`
- Returns JSON with `data` and `errors` fields
- Language bindings treat GraphQL handlers like any other handler

### JSON-RPC Handlers

**Location:** `crates/spikard-http/src/jsonrpc/`

- JSON-RPC 2.0 specification compliance
- Uses Handler trait with JSON-RPC request/response format
- Single endpoint for multiple methods via `method` field in request

## Testing Handler Implementations

### Unit Tests

**Node.js:** `/Users/naamanhirschfeld/workspace/spikard/packages/node/src/handler-wrapper.spec.ts`

- Test ThreadsafeFunction call behavior
- Test response parsing (status, headers, body)
- Test error handling and Promise rejection

**Python:** `/Users/naamanhirschfeld/workspace/spikard/packages/python/tests/`

- Test async handler execution
- Test request binding and parameter extraction
- Test response serialization

### Integration Tests

Use `TestClient` (in-memory, no server startup):

```typescript
const client = new Spikard().testClient();
const response = await client.get('/users/123');
expect(response.status).toBe(200);
```

```python
client = app.test_client()
response = client.get('/users/123')
assert response.status == 200
```

### Fixture-Driven Tests

Load from `testing_data/*/` directories:

- Test all HTTP methods
- Test all status codes
- Test various request/response payloads
- Validate error responses

## Error Handling Contract

All handlers must return:

1. **Success (2xx):**

   ```json
   { "status": 200, "body": {...} }
   ```

2. **Client Error (4xx):**

   ```json
   {
     "status": 400,
     "body": {
       "type": "https://example.com/problems/validation-error",
       "title": "Validation Failed",
       "status": 400,
       "detail": "Invalid request format"
     }
   }
   ```

3. **Server Error (5xx):**

   ```json
   {
     "status": 500,
     "body": {
       "type": "https://example.com/problems/internal-error",
       "title": "Internal Server Error",
       "status": 500,
       "detail": "An unexpected error occurred"
     }
   }
   ```

## Performance Characteristics

- **Zero serialization** in Rust server (accepts JSON directly)
- **Single serialization** in language bindings (request in, response out)
- **No copying** of request bodies (streaming when possible)
- **Parallel handler execution** across multiple Axum worker threads
- **Composable** with middleware and lifecycle hooks

## Related Skills

- `tower-middleware-patterns` - Middleware provides RequestData
- `request-response-lifecycle` - Lifecycle hooks wrap handlers
- `di-pattern-implementation` - Dependency injection via RequestData extensions
- `code-generator-design` - Auto-generate handler stubs from specs
