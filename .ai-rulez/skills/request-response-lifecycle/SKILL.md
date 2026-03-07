---
priority: high
description: "Request-Response Lifecycle - Hooks & Middleware Chain"
---

# Request-Response Lifecycle - Hooks & Middleware Chain

**onRequest · preValidation · preHandler · onResponse · onError hooks**

## Architecture Overview

Spikard's lifecycle system provides extension points before/after request handling, enabling cross-cutting concerns (logging, transformation, error handling) without touching handler code.

Reference: `/Users/naamanhirschfeld/workspace/spikard/crates/spikard-http/src/lifecycle/`

## Core Lifecycle Hooks

### Hook Execution Order

```
HTTP Request
  ↓
[1] onRequest Hook
    └─ Inspect/modify incoming request
  ↓
[2] preValidation Hook
    └─ Pre-schema validation transformations
  ↓
Middleware Stack (compression, rate limit, auth, CORS, etc.)
  ↓
[3] preHandler Hook
    └─ Final setup before handler execution
  ↓
Handler Execution
  ↓
[4] onResponse Hook
    └─ Transform response before sending
  ↓
[5] onError Hook (if handler threw)
    └─ Transform error response
  ↓
HTTP Response
```

## Hook Definitions

### 1. onRequest Hook

**Purpose:** First hook after middleware, before validation

**Type Signature:**

```rust
pub trait LifecycleHook<Req, Resp> {
    fn execute_request(
        &self,
        req: Req,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Req, Resp>, String>> + Send>>;
}

pub enum HookResult<Req, Resp> {
    Continue(Req),          // Pass request to next stage
    Abort(Resp),           // Short-circuit with response
}
```

**Language Bindings:**

**Node.js:**

```typescript
app.onRequest(async (request) => {
  console.log(`${request.method} ${request.path}`);
  // Modify request if needed
  request.headers['X-Timestamp'] = new Date().toISOString();
  return request;  // or throw for abort
});
```

**Python:**

```python
@app.onRequest()
async def log_request(request):
    print(f"{request.method} {request.path}")
    request.headers['X-Timestamp'] = datetime.now().isoformat()
    return request  # or raise for abort
```

**Ruby:**

```ruby
app.on_request do |request|
  puts "#{request.method} #{request.path}"
  request.headers['X-Timestamp'] = Time.now.iso8601
  request
end
```

**Use Cases:**

- Request logging and monitoring
- Request ID propagation to logs
- Custom header injection
- Request metadata enrichment
- Rate limiting per user/IP

**Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/lifecycle_hooks/01-on-request-*.json` (3+ fixtures)

### 2. preValidation Hook

**Purpose:** Modify request before schema validation

**Type Signature:**

```rust
pub trait LifecycleHook<Req, Resp> {
    fn execute_request(
        &self,
        req: Req,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Req, Resp>, String>> + Send>>;
}
```

**Language Bindings:**

**Node.js:**

```typescript
app.preValidation(async (request) => {
  // Normalize request data before validation
  if (request.body?.email) {
    request.body.email = request.body.email.toLowerCase();
  }
  return request;
});
```

**Python:**

```python
@app.preValidation()
async def normalize_request(request):
    if request.body and 'email' in request.body:
        request.body['email'] = request.body['email'].lower()
    return request
```

**Use Cases:**

- Data normalization (lowercase email, trim whitespace)
- Default value injection
- Request enrichment with derived fields
- Type coercion before validation
- Deny-listing/allow-listing fields

**Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/lifecycle_hooks/02-pre-validation-*.json` (3+ fixtures)

### 3. preHandler Hook

**Purpose:** Final setup immediately before handler execution

**Type Signature:** Same as onRequest

**Language Bindings:**

**Node.js:**

```typescript
app.preHandler(async (request) => {
  // Last chance to modify request
  request.extensions.startTime = Date.now();
  return request;
});
```

**Python:**

```python
@app.preHandler()
async def setup_handler_context(request):
    request.extensions['start_time'] = time.time()
    return request
```

**Use Cases:**

- Performance timing setup
- Request context binding (user, session, etc.)
- Final authorization checks
- Resource acquisition (database connection, cache)
- Request tracer span creation

**Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/lifecycle_hooks/03-pre-handler-*.json` (3+ fixtures)

### 4. onResponse Hook

**Purpose:** Transform handler response before sending

**Type Signature:**

```rust
pub trait LifecycleHook<Req, Resp> {
    fn execute_response(
        &self,
        req: Req,
        resp: Resp,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Req, Resp>, String>> + Send>>;
}
```

**Language Bindings:**

**Node.js:**

```typescript
app.onResponse(async (request, response) => {
  // Add headers, modify body, etc.
  response.headers['X-Request-Duration'] = Date.now() - request.extensions.startTime;

  // Wrap response body
  if (response.status === 200 && response.body) {
    response.body = {
      data: response.body,
      timestamp: new Date().toISOString()
    };
  }

  return response;
});
```

**Python:**

```python
@app.onResponse()
async def enrich_response(request, response):
    response.headers['X-Request-Duration'] = str(time.time() - request.extensions['start_time'])

    # Transform successful responses
    if response.status == 200 and response.body:
        response.body = {
            'data': response.body,
            'timestamp': datetime.now().isoformat()
        }

    return response
```

**Use Cases:**

- Response logging and monitoring
- Add timing headers (X-Response-Time)
- Response body wrapping (envelope pattern)
- Sanitize sensitive data before sending
- Add caching headers (Cache-Control, ETag)
- Response compression hints

**Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/lifecycle_hooks/04-on-response-*.json` (3+ fixtures)

### 5. onError Hook

**Purpose:** Transform error responses

**Type Signature:**

```rust
pub trait LifecycleHook<Req, Resp> {
    fn execute_error(
        &self,
        req: Req,
        error: Error,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Req, Resp>, String>> + Send>>;
}
```

**Language Bindings:**

**Node.js:**

```typescript
app.onError(async (request, error) => {
  console.error(`Error handling ${request.method} ${request.path}:`, error);

  // Transform error response
  const response = {
    status: error.status || 500,
    body: {
      type: `https://example.com/problems/${error.type || 'internal-error'}`,
      title: error.message,
      status: error.status || 500,
      detail: process.env.NODE_ENV === 'development' ? error.stack : 'An error occurred'
    }
  };

  return response;
});
```

**Python:**

```python
@app.onError()
async def handle_error(request, error):
    import logging
    logging.error(f"Error in {request.method} {request.path}: {error}")

    # Custom error response
    response = {
        'status': getattr(error, 'status', 500),
        'body': {
            'type': f"https://example.com/problems/{getattr(error, 'type', 'internal-error')}",
            'title': str(error),
            'status': getattr(error, 'status', 500),
            'detail': str(error.__dict__) if os.getenv('DEBUG') else 'An error occurred'
        }
    }

    return response
```

**Use Cases:**

- Custom error formatting and messaging
- Error logging with context
- Sensitive error hiding (production vs development)
- Error code mapping (domain errors → HTTP codes)
- Error recovery and fallback responses
- Error metrics collection

**Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/lifecycle_hooks/05-on-error-*.json` (3+ fixtures)

## Hook Registration

### Global Hooks

**Node.js:**

```typescript
const app = new Spikard();

app.onRequest(async (request) => {
  // Applied to ALL routes
});

app.get('/users')(async (request) => {
  // Handler
});
```

### Route-Specific Hooks

**Python:**

```python
app = Spikard()

@app.onRequest()
async def global_request_hook(request):
    pass

@app.get('/users/{id}')
async def get_user(request):
    pass

# Route-specific hook (planned feature)
@app.preHandler()
@app.get('/admin/*')
async def admin_only(request):
    if request.extensions.get('role') != 'admin':
        raise PermissionError()
    return request
```

## Hook Execution Details

### Async Execution

All hooks are async, allowing:

- Database queries (fetch user from request ID)
- External API calls (audit logging service)
- Resource initialization (connection pools)
- Slow transformations

**Concurrent vs Sequential:**

- **Sequential:** Hooks execute in order
- **Parallel:** Independent hooks could execute in parallel (not yet optimized)

### Error Propagation

Hook errors short-circuit the request:

```typescript
// If onRequest throws, skip to onError
app.onRequest(async (request) => {
  if (!request.headers.authorization) {
    throw new Error('Missing authorization header');  // → onError hook
  }
  return request;
});
```

### Multiple Hook Instances

All hooks of the same type execute sequentially:

```typescript
app.onRequest(hook1);
app.onRequest(hook2);
app.onRequest(hook3);

// Execution: hook1 → hook2 → hook3
```

## Request Extensions

Hooks share state via `request.extensions` (request-scoped storage):

```typescript
app.preHandler(async (request) => {
  const user = await fetchUser(request.headers.authorization);
  request.extensions.user = user;
  return request;
});

// Handler accesses user
async function getProfile(request) {
  const user = request.extensions.user;  // Available in handler
  return { status: 200, body: user };
}
```

**Extension Lifecycle:**

1. Created when request arrives
2. Populated by hooks and middleware
3. Accessible in handler
4. Passed to onResponse/onError hooks
5. Cleaned up after response sent

## Common Patterns

### Request Logging

```typescript
app.onRequest(async (request) => {
  const startTime = Date.now();
  request.extensions.startTime = startTime;
  console.log(`[${request.requestId}] ${request.method} ${request.path}`);
  return request;
});

app.onResponse(async (request, response) => {
  const duration = Date.now() - request.extensions.startTime;
  console.log(
    `[${request.requestId}] ${response.status} ${duration}ms`
  );
  return response;
});
```

### Authentication & Authorization

```typescript
app.preHandler(async (request) => {
  const token = request.headers.authorization?.replace('Bearer ', '');
  if (!token) {
    const response = {
      status: 401,
      body: { error: 'Unauthorized' }
    };
    throw new Error('Missing token');
  }

  const user = await verifyToken(token);
  request.extensions.user = user;
  return request;
});
```

### Response Wrapping

```typescript
app.onResponse(async (request, response) => {
  if (response.status === 200) {
    response.body = {
      success: true,
      data: response.body,
      timestamp: new Date().toISOString()
    };
  }
  return response;
});
```

### Error Transformation

```typescript
app.onError(async (request, error) => {
  return {
    status: error.status || 500,
    headers: { 'Content-Type': 'application/json' },
    body: {
      error: {
        code: error.code,
        message: error.message,
        timestamp: new Date().toISOString()
      }
    }
  };
});
```

### Resource Cleanup

```typescript
app.preHandler(async (request) => {
  const connection = await database.acquire();
  request.extensions.db = connection;
  return request;
});

app.onResponse(async (request, response) => {
  if (request.extensions.db) {
    await request.extensions.db.release();
  }
  return response;
});

app.onError(async (request, error) => {
  if (request.extensions.db) {
    await request.extensions.db.rollback();
    await request.extensions.db.release();
  }
  // Return error response
});
```

## Language Binding Implementation

### Node.js Implementation

**Location:** `packages/node/native/src/lifecycle.rs`

- Uses `ThreadsafeFunction` to call JavaScript hooks
- Async/await via napi-rs Promise support
- Request/response serialization via JSON
- Error handling with proper propagation

```rust
pub struct NodeLifecycleHook {
    func: Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>,
}

impl LifecycleHook for NodeLifecycleHook {
    fn execute_request(&self, req: Request<Body>) -> Pin<Box<...>> {
        let func = Arc::clone(&self.func);
        Box::pin(async move {
            // Serialize request to JSON
            // Call JavaScript function
            // Deserialize response
        })
    }
}
```

### Python Implementation

**Location:** `crates/spikard-py/src/lifecycle.rs`

- Uses PyO3 and `pyo3_async_runtimes`
- Async Python functions via `asyncio` integration
- Type checking and validation
- Proper exception handling

## Testing Lifecycle Hooks

### Unit Tests

**Node.js:**

```typescript
test('onRequest hook modifies request', async () => {
  const app = new Spikard();

  app.onRequest(async (request) => {
    request.headers['X-Custom'] = 'value';
    return request;
  });

  const client = app.testClient();
  const response = await client.get('/test');

  expect(response.headers['X-Custom']).toBe('value');
});
```

**Python:**

```python
def test_onRequest_hook_modifies_request():
    app = Spikard()

    @app.onRequest()
    async def add_header(request):
        request.headers['X-Custom'] = 'value'
        return request

    @app.get('/test')
    async def handler(request):
        return {'status': 200, 'body': request.headers}

    client = TestClient(app)
    response = client.get('/test')
    assert response.headers['X-Custom'] == 'value'
```

### Fixture-Based Tests

**Pattern:** Use fixtures with expected hook behavior

```json
{
  "category": "lifecycle_hooks",
  "name": "onRequest_adds_header",
  "method": "GET",
  "path": "/test",
  "request": {
    "headers": {}
  },
  "response": {
    "status": 200,
    "headers": {
      "X-Custom": "value"
    }
  }
}
```

## Hook Limitations

- **No access to response body** in preHandler (response not yet created)
- **No access to request body** if streaming (already consumed)
- **Limited async operations** (avoid slow external calls in hot path)
- **No hook composition** (cannot chain/modify hook behavior)

## Performance Considerations

- **Zero-cost if no hooks:** Compiled away
- **Minimal overhead:** Single function call per hook type
- **Async-friendly:** No blocking operations
- **Parallelizable:** Independent hooks could run concurrently (future optimization)

## Related Skills

- `handler-trait-design` - Handler receives RequestData from hooks
- `tower-middleware-patterns` - Hooks execute after middleware
- `async-runtime-integration` - Hooks run on async runtime
- `di-pattern-implementation` - Extensions store DI-resolved dependencies
