---
priority: high
---

# Handler Abstraction Domain

**Unified handler trait across languages (Rust, Python, Node.js, Ruby, PHP)**

## Overview

Spikard provides a language-agnostic `Handler` trait that enables request handling implementations in any language to be called from the Rust HTTP server. This abstracts away all FFI complexity and provides a consistent, type-safe interface.

## Core Trait Definition

Located in `/crates/spikard-http/src/handler_trait.rs`:

```rust
pub trait Handler: Send + Sync {
    fn handle(
        &self,
        request: RequestData,
    ) -> Pin<Box<dyn Future<Output = Result<HandlerResponse, HandlerError>> + Send>>;
}
```

## RequestData Structure

**Complete language-agnostic request representation:**

```rust
pub struct RequestData {
    pub path_params: Arc<HashMap<String, String>>,      // /users/{id} → {"id": "123"}
    pub query_params: Value,                             // ?page=1&limit=10
    pub validated_params: Option<Value>,                 // ParameterValidator output
    pub raw_query_params: Arc<HashMap<String, Vec<String>>>,
    pub body: Value,                                     // Parsed JSON
    pub raw_body: Option<Bytes>,                         // Original bytes (preferred)
    pub headers: Arc<HashMap<String, String>>,          // HTTP headers
    pub cookies: Arc<HashMap<String, String>>,          // Parsed cookies
    pub method: String,                                  // GET, POST, etc.
    pub path: String,                                    // Full request path
    #[cfg(feature = "di")]
    pub dependencies: Option<Arc<ResolvedDependencies>>, // DI container
}
```

**Key characteristics:**

- **Arc-based** for cheap cloning across FFI
- **raw_body preferred** for language bindings (avoid double-parsing)
- **validated_params** from ParameterValidator, not raw query/path
- **Serializable** for passing to FFI boundaries
- **Optional DI** for dependency injection support

## HandlerResponse Structure

```rust
pub struct HandlerResponse {
    pub status: u16,                              // 200, 404, 500, etc.
    pub headers: Option<HashMap<String, String>>, // Response headers
    pub body: HandlerResponseBody,                // Enum: Json, Text, Binary, Empty
}

pub enum HandlerResponseBody {
    Json(Value),
    Text(String),
    Binary(Bytes),
    Empty,
}
```

## Language Binding Architecture

### Binding Layers (Wrapper Pattern)

Each language binding implements Handler by:

1. **Binding Interface Layer** (FFI boundary)
   - Receives `RequestData` (serialized or passed by reference)
   - Calls language-specific code
   - Returns `HandlerResponse` (serialized)

2. **Handler Implementation** (Language-specific)
   - Parses RequestData from Rust representation
   - Executes application logic
   - Returns HandlerResponse

3. **Async Runtime Integration** (Language-specific)
   - Python: `pyo3_async_runtimes` for asyncio integration
   - Node.js: `napi-rs` with libuv event loop
   - Ruby: `magnus` with background threads
   - PHP: `ext-php-rs` with async-php support

### Implementation Patterns by Language

#### Python (PyO3)

Located in `/crates/spikard-py/`:

```python
class Handler:
    async def handle(self, request_data: RequestData) -> HandlerResponse:
        # Process request
        return HandlerResponse(status=200, body={"message": "OK"})
```

- Uses `pyo3_async_runtimes` for proper asyncio integration
- `RequestData` passes through FFI boundary
- Raw body preferred to avoid double JSON parsing

#### Node.js (napi-rs)

Located in `/crates/spikard-node/`:

```typescript
export class Handler {
    async handle(requestData: RequestData): Promise<HandlerResponse> {
        // Process request
        return { status: 200, body: { message: "OK" } };
    }
}
```

- Uses `ThreadsafeFunction` for async callbacks
- Proper Promise handling via napi-rs
- Automatic serialization of responses

#### Ruby (Magnus)

Located in `/crates/spikard-rb/`:

```ruby
class Handler
    def handle(request_data)
        # Process request
        HandlerResponse.new(status: 200, body: { message: "OK" })
    end
end
```

- Uses background threads for async operations
- `magnus` gem for FFI integration
- Type checking via RBS files

#### PHP (ext-php-rs)

Located in `/crates/spikard-php/`:

```php
class Handler {
    public function handle(RequestData $requestData): HandlerResponse {
        // Process request
        return new HandlerResponse(200, ["message" => "OK"]);
    }
}
```

- Synchronous execution model with thread pool
- `ext-php-rs` for Rust extension
- Type declarations via phpstan

## DI (Dependency Injection) Handler

Located in `/crates/spikard-http/src/di_handler.rs`:

**Optional feature** `di` enables automatic dependency resolution:

```rust
pub struct DependencyInjectingHandler {
    inner: Arc<dyn Handler>,
    di_container: Arc<DiContainer>,
}

impl Handler for DependencyInjectingHandler {
    fn handle(&self, mut request: RequestData) -> /* ... */ {
        // Resolve dependencies for this request
        request.dependencies = Some(Arc::new(
            self.di_container.resolve_for_request(&request)
        ));
        self.inner.handle(request)
    }
}
```

**Features:**

- Request-scoped dependency resolution
- Type-safe dependency graph
- Zero-cost when disabled
- Language bindings access via `request.dependencies`

## Handler Registration & Routing

Located in `/crates/spikard-http/src/server/mod.rs`:

```rust
pub struct Server {
    handlers: Vec<(Route, Arc<dyn Handler>)>,
    // ... other config
}

impl Server {
    pub async fn register_handler(
        &mut self,
        route: Route,
        handler: Arc<dyn Handler>,
    ) { /* ... */ }
}
```

**Registration flow:**

1. Language binding creates Handler implementation
2. Wraps in Arc for thread-safety
3. Registers with route definition
4. Server compiles routes into Axum router
5. Incoming requests dispatched to handler

## Handler Lifecycle

1. **Request arrives** at HTTP server
2. **Route matching** determines handler
3. **Middleware execution** (compression, auth, rate limiting)
4. **RequestData construction** from HTTP request
5. **Optional DI resolution** if di_handler wraps
6. **Handler invocation** - `handler.handle(request_data)`
7. **Response extraction** from HandlerResponse
8. **Response middleware** (serialization, encoding)
9. **HTTP response transmission**

## Testing Handler Implementations

Located in `/crates/spikard-http/tests/`:

- Fixture-based tests in `/testing_data/http_methods/`
- Tests verify same behavior across all language bindings
- Round-trip tests ensure request/response fidelity
- Performance benchmarks for FFI overhead

## Error Handling

**Handler errors** use `HandlerError` type:

```rust
pub enum HandlerError {
    ValidationError(String),
    NotFound(String),
    Unauthorized,
    InternalError(String),
}
```

Converts to HTTP status codes:

- ValidationError → 400 Bad Request
- NotFound → 404 Not Found
- Unauthorized → 401 Unauthorized
- InternalError → 500 Internal Server Error

## Performance Characteristics

- **Arc-based** cloning is O(1), atomic operation
- **No data copy** for raw_body (zero-copy to bindings)
- **Serialization** only at FFI boundaries
- **Async throughout** - no blocking in Rust
- **Binding-specific** optimization (inline code generation)
- **Connection pooling** via tokio for database/service calls

## Integration Points

- **HTTP Framework**: Handler receives RequestData, returns HandlerResponse
- **Code Generation**: OpenAPI/GraphQL/AsyncAPI generators produce Handler implementations
- **Language Bindings**: Python/Node/Ruby/PHP all implement Handler trait
- **Middleware**: Lifecycle hooks integrate Handler execution
- **Testing**: Fixture tests validate Handler behavior across all languages
