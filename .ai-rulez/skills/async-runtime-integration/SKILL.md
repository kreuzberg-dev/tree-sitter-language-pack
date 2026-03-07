---
priority: high
description: "Async Runtime Integration - pyo3-async-runtimes & Tokio Coordination"
---

# Async Runtime Integration - pyo3-async-runtimes & Tokio Coordination

**Python async bridge · Tokio runtime · Language-agnostic futures · Thread pools**

## Architecture Overview

Spikard coordinates async execution across language bindings (Python, Node.js, Ruby) using a shared Tokio runtime in a background Rust thread. This enables truly async, non-blocking HTTP handling while respecting each language's event loop.

Reference: `/Users/naamanhirschfeld/workspace/spikard/crates/spikard-http/src/server/`

## Core Runtime Architecture

### Tokio Background Thread

**Location:** `packages/node/native/src/background.rs` and `packages/python/spikard/_internal.rs`

**Setup pattern:**

```rust
// Spawn dedicated Tokio runtime in background thread
let runtime = tokio::runtime::Runtime::new()?;
let (server_handle, shutdown_tx) = runtime.block_on(async {
    // Start Axum HTTP server
    let app = create_router(...);
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let server = axum::serve(listener, app);

    // Return handle for graceful shutdown
    (server, shutdown_channel)
});

// Spawn on background thread
std::thread::spawn(move || {
    runtime.block_on(server_handle);
});
```

**Thread isolation:**

- Tokio runtime runs in dedicated thread
- Does NOT block Node.js or Python event loops
- Safe inter-thread communication via channels

### Graceful Shutdown

**Pattern:** Coordinated shutdown across runtimes

```rust
// Node.js receives SIGTERM
process.on('SIGTERM', async () => {
  console.log('Shutting down gracefully...');

  // Signal Tokio runtime to shutdown
  shutdown_tx.send(()).ok();

  // Wait for server to finish connections
  tokio_handle.join()?;

  // Exit process
  process.exit(0);
});
```

**Shutdown sequence:**

1. Receive shutdown signal
2. Stop accepting new connections
3. Wait for in-flight requests (configurable timeout)
4. Close all connections
5. Cleanup resources
6. Exit

**Configuration:** `ServerConfig::shutdown_timeout` (default 30 seconds)

## Language-Specific Integration

### Python: PyO3 + pyo3-async-runtimes

**Location:** `crates/spikard-py/src/handler.rs` and `crates/spikard-py/src/lifecycle.rs`

**Problem:** Python has single-threaded event loop (GIL). Calling async Python from Rust async requires special handling.

**Solution:** `pyo3_async_runtimes` crate bridges PyO3 and Tokio

```rust
use pyo3_async_runtimes::TaskLocals;

// In Rust async context (Tokio)
pub async fn call_python_handler(
    handler: PyObject,
    request: RequestData,
) -> Result<HandlerResponse> {
    // Get task locals from current Tokio task
    let task_locals = TaskLocals::current();

    // Call Python async function from Tokio context
    let response: HandlerResponse = task_locals
        .scope(async {
            // Convert to PyObject, call handler, convert back
            let py_request = serde_json::to_string(&request)?;
            let py_response = Python::with_gil(|py| {
                handler.call1(py, (py_request,))
            })?;

            // Await Python coroutine
            task_locals.run(async {
                // Handler returns coroutine, await it
                py_response.await
            }).await
        })
        .await?;

    Ok(response)
}
```

**Key requirements:**

1. **GIL management:** Release GIL during async operations
2. **Event loop coordination:** PyO3 async integrates with Python's asyncio
3. **Await support:** Python coroutines properly awaited
4. **Exception handling:** Python exceptions become Rust Errors

**Handler example:**

```python
# Python async handler (runs in asyncio event loop)
@app.get('/users/{id}')
async def get_user(request):
    # This is async Python code
    user = await database.fetch_user(request.params['id'])

    return {
        'status': 200,
        'body': user.__dict__
    }
```

**Execution flow:**

1. Request arrives at Tokio runtime (Rust)
2. Tokio calls Python handler via PyO3
3. PyO3 adds task locals to Tokio task
4. Python async function executes in asyncio context
5. When Python awaits (database.fetch_user), control returns to asyncio
6. Tokio runtime manages concurrency for all waiting requests
7. Result returns to Tokio → HTTP response

### Node.js: NAPI + ThreadsafeFunction

**Location:** `packages/node/native/src/handler.rs` and `packages/node/native/src/lifecycle.rs`

**Pattern:** ThreadsafeFunction callback to JavaScript

```rust
// In Tokio async context
pub async fn call_javascript_handler(
    handler_fn: Arc<ThreadsafeFunction<String, Promise<String>, ...>>,
    request: RequestData,
) -> Result<HandlerResponse> {
    // Serialize request to JSON
    let request_json = serde_json::to_string(&request)?;

    // Call JavaScript (returns Promise)
    let promise = handler_fn.call(
        Ok(request_json),
        napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
    );

    // Wait for Promise to resolve
    let response_json = promise.await?;

    // Deserialize response
    let response = serde_json::from_str(&response_json)?;

    Ok(response)
}
```

**Key characteristics:**

1. **Non-blocking to Node.js:** Uses NonBlocking mode
2. **Promise-based:** JavaScript handler returns Promise
3. **Event loop safe:** ThreadsafeFunction doesn't block Node's event loop
4. **Automatic await:** NAPI handles Promise resolution

**Handler example:**

```typescript
// JavaScript async handler (runs in Node.js event loop)
get('/users/{id}')(async (request) => {
  // This is async JavaScript code
  const user = await database.fetchUser(request.params.id);

  return {
    status: 200,
    body: user
  };
});
```

**Execution flow:**

1. Request arrives at Tokio runtime (Rust thread)
2. Tokio calls ThreadsafeFunction to Node.js
3. Node.js event loop receives callback
4. JavaScript async handler called, returns Promise
5. Promise queued in Node.js event loop
6. When JavaScript awaits (database.fetchUser), control returns to Node loop
7. Node.js event loop processes other JavaScript
8. Promise resolves, callback fires
9. ThreadsafeFunction receives result → Tokio task resumes
10. Response sent via HTTP

### Ruby: FFI + Thread Coordination

**Location:** `crates/spikard-rb/src/handler.rs`

**Pattern:** FFI call to Ruby, wait for async completion

```rust
pub async fn call_ruby_handler(
    handler: RbValue,
    request: RequestData,
) -> Result<HandlerResponse> {
    // Call Ruby handler via FFI
    // Ruby can return a promise/future-like object
    let result = ruby::call_handler(&handler, &request)?;

    // If result is a Fiber/Future, await it
    if result.is_async() {
        result.wait().await?
    } else {
        result
    }
}
```

## Concurrent Request Handling

### Worker Threads in Axum

**Configuration:** `ServerConfig::workers` (default 1 per CPU core)

```rust
// Spawn Tokio runtime with multiple worker threads
let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(config.workers)
    .enable_all()
    .build()?;

// Each worker thread independently processes requests
// Handlers execute in parallel across all workers
```

**Default:** One Tokio worker per CPU core

**Example:** 8-core machine = 8 concurrent requests possible

### Request Queuing

**Pattern:** Multiple requests queue for workers

```
Request 1 → Worker 1 (async handler waiting for DB)
Request 2 → Worker 2 (async handler waiting for API)
Request 3 → Worker 3 (sync handler, quick return)
Request 4 → Queued (all workers busy)
Request 5 → Queued
```

**Benefits:**

- True parallelism (not just concurrency)
- Efficient CPU utilization
- Natural request batching

## Timeout & Cancellation

### Request Timeout

**Configuration:** `ServerConfig::request_timeout` (milliseconds)

```rust
// Wrap handler call in timeout
let result = tokio::time::timeout(
    Duration::from_millis(request_timeout),
    handler.handle(request_data)
).await;

match result {
    Ok(Ok(response)) => Ok(response),
    Ok(Err(e)) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    Err(_) => Err((StatusCode::REQUEST_TIMEOUT, "Request timeout".to_string())),
}
```

**Behavior:**

- Handler execution canceled after timeout
- Returns 408 Request Timeout
- In-flight async operations (database queries) cancelled
- Resources cleaned up via Drop trait

### Graceful Cancellation

**Pattern:** Async cleanup on cancellation

```python
@app.get('/long-operation')
async def long_operation(request):
    try:
        result = await long_running_task()
        return {'status': 200, 'body': result}
    except asyncio.CancelledError:
        # Cleanup on timeout
        await database.cancel_pending_queries()
        raise
```

## Background Tasks

**Location:** `crates/spikard-http/src/background.rs`

**Pattern:** Spawn background work without blocking response

```rust
pub enum BackgroundRuntime {
    Tokio,      // Use Tokio runtime (recommended)
    ThreadPool, // Fallback thread pool
}

// In handler
pub async fn create_user(request: RequestData) -> HandlerResult {
    // Insert user synchronously
    let user = database.create_user(&request.body)?;

    // Spawn background task (doesn't block response)
    if let Some(bg_runtime) = &request.extensions.background_runtime {
        bg_runtime.spawn(async move {
            // Send confirmation email asynchronously
            email_service.send_welcome_email(&user).await.ok();

            // Log event asynchronously
            analytics.log_user_created(&user).await.ok();
        });
    }

    // Return immediately
    Ok(HandlerResponse {
        status: StatusCode::CREATED,
        body: Some(user),
        headers: Default::default(),
    })
}
```

**Benefits:**

- Non-blocking background work
- Tasks run after response sent
- Automatic cancellation on shutdown

## Performance Optimization

### Connection Pooling

**Pattern:** Reuse connections across requests

```rust
// Shared database connection pool
let pool = sqlx::PgPool::connect(&db_url).await?;

// Each handler uses connection from pool
pub async fn get_user(request: RequestData) -> HandlerResult {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&request.params["id"])
        .fetch_one(&pool)
        .await?;

    Ok(HandlerResponse {
        status: StatusCode::OK,
        body: Some(serde_json::to_value(user)?),
        headers: Default::default(),
    })
}
```

### Response Streaming

**Pattern:** Stream large responses without buffering

```typescript
app.get('/large-file')(async (request) => {
  // Instead of loading entire file, stream it
  const fileStream = fs.createReadStream('large-file.bin');

  return {
    status: 200,
    headers: { 'Content-Type': 'application/octet-stream' },
    body: fileStream  // Tokio handles streaming
  };
});
```

### Zero-Copy Optimizations

**Pattern:** Avoid unnecessary serialization

```rust
// In middleware: pre-parse JSON once
pub struct PreParsedJson(pub serde_json::Value);

// Handler reuses parsed value
pub async fn handler(request: RequestData) -> HandlerResult {
    let body = request.extensions.get::<PreParsedJson>()?;
    // Use pre-parsed body, no re-parsing
}
```

## Benchmarking & Monitoring

### Request Latency

**Measurement:** Hook-based timing

```typescript
app.preHandler(async (request) => {
  request.extensions.startTime = Date.now();
  return request;
});

app.onResponse(async (request, response) => {
  const duration = Date.now() - request.extensions.startTime;
  metrics.record('request_duration_ms', duration);
  return response;
});
```

### Async Operation Tracing

**Pattern:** Instrument async operations

```python
import time

@app.preHandler()
async def start_timer(request):
    request.extensions['timer'] = time.time()
    return request

@app.get('/users/{id}')
async def get_user(request):
    start = time.time()

    # Database operation
    user = await database.fetch_user(request.params['id'])

    db_time = time.time() - start
    print(f"Database took {db_time:.3f}s")

    return {'status': 200, 'body': user}
```

## Thread-Safety & Deadlock Prevention

### Send + Sync Constraints

All handler code must be `Send + Sync`:

```rust
// OK: Send + Sync
struct MyHandler {
    db: Arc<DatabasePool>,  // Arc is Send + Sync
}

// ERROR: Not Send + Sync
struct BadHandler {
    db: Rc<DatabasePool>,   // Rc is NOT Send
}
```

### No Blocking in Async Context

```rust
// BAD: Blocks Tokio worker
pub async fn bad_handler(request: RequestData) -> HandlerResult {
    let user = std::thread::sleep(Duration::from_secs(1));  // BLOCKS!
    Ok(...)
}

// GOOD: Truly async
pub async fn good_handler(request: RequestData) -> HandlerResult {
    tokio::time::sleep(Duration::from_secs(1)).await;  // Non-blocking
    Ok(...)
}
```

## Testing Async Behavior

### Unit Tests

**Node.js:**

```typescript
test('async handler waits for database', async () => {
  const app = new Spikard();

  app.get('/users/{id}')(async (request) => {
    const user = await mockDatabase.get(request.params.id);
    return { status: 200, body: user };
  });

  const client = app.testClient();
  const response = await client.get('/users/123');

  expect(response.status).toBe(200);
  expect(response.body.id).toBe(123);
});
```

### Concurrent Requests

**Pattern:** Test parallel request handling

```python
import asyncio
from spikard.testing import TestClient

async def test_concurrent_requests():
    app = Spikard()

    @app.get('/delay/{seconds}')
    async def delay_handler(request):
        await asyncio.sleep(int(request.params['seconds']))
        return {'status': 200, 'body': 'done'}

    client = TestClient(app)

    # 3 concurrent requests
    tasks = [
        asyncio.create_task(client.get('/delay/1')),
        asyncio.create_task(client.get('/delay/1')),
        asyncio.create_task(client.get('/delay/1')),
    ]

    # Should complete in ~1 second (parallel), not 3 seconds (serial)
    results = await asyncio.gather(*tasks)
    assert all(r.status == 200 for r in results)
```

## Debugging Async Issues

### Common Problems

1. **Blocking in async context:** Use `tokio::time::sleep` not `std::thread::sleep`
2. **Deadlock:** Ensure no nested locks or circular dependencies
3. **Memory leaks:** Check for unclosed resources in exception paths
4. **Panic in async:** Tokio isolates panic to single task, doesn't crash all workers

### Tracing & Logging

```rust
use tracing::{debug, info, span, Level};

pub async fn handler(request: RequestData) -> HandlerResult {
    let span = span!(Level::DEBUG, "handle_request", id = %request.request_id);
    let _guard = span.enter();

    debug!("handler started");
    let result = do_work().await;
    debug!("handler finished");

    Ok(result)
}
```

## Related Skills

- `handler-trait-design` - Handler trait requires Send + Sync
- `request-response-lifecycle` - Lifecycle hooks execute async
- `tower-middleware-patterns` - Middleware integrated with Tokio
- `di-pattern-implementation` - DI resolution is async
