---
priority: high
---

# HTTP Framework Domain

**Axum + Tower-HTTP middleware stack, request/response lifecycle**

## Overview

The spikard HTTP framework is built on Axum and Tower-HTTP, providing a production-grade HTTP server with sophisticated middleware composition, lifecycle hooks, and request/response processing pipelines.

## Core Components

### Middleware Stack

Located in `/crates/spikard-http/src/middleware/`:

1. **Compression** - Gzip, Brotli compression (tower-http)
2. **Rate Limiting** - Governor-based request throttling (tower_governor)
3. **Timeout** - Request timeout enforcement
4. **Request ID** - Correlation ID tracking
5. **Authentication** - JWT and API Key middleware
6. **User Agent** - Request user agent parsing
7. **Content Parsing** - Multipart form, URL-encoded data

### Request/Response Lifecycle

- **RequestData** (`handler_trait.rs`): Language-agnostic request representation with:
    - `path_params`: Path segment parameters (Arc<HashMap>)
    - `query_params`: Query string parameters (serde_json::Value)
    - `validated_params`: Combined validated parameters (ParameterValidator output)
    - `body`: Parsed request body (Value)
    - `raw_body`: Original request bytes (Option<Bytes>)
    - `headers`: HTTP headers (Arc<HashMap>)
    - `cookies`: Parsed cookies (Arc<HashMap>)
    - `method`: HTTP method string
    - `path`: Request path

- **HandlerResponse** (`handler_response.rs`): Structured response with status, headers, body

### Server Configuration

**ServerConfig** exposes:

- Middleware configuration (CompressionConfig, RateLimitConfig, StaticFilesConfig)
- TLS/SSL settings
- Graceful shutdown timeouts
- CORS policies
- Static file serving

### Key Files

- `/crates/spikard-http/src/server/mod.rs` - Server implementation
- `/crates/spikard-http/src/server/handler.rs` - Handler execution
- `/crates/spikard-http/src/server/request_extraction.rs` - Request parsing
- `/crates/spikard-http/src/server/routing_factory.rs` - Route compilation
- `/crates/spikard-http/src/server/lifecycle_execution.rs` - Hook execution
- `/crates/spikard-http/src/auth.rs` - JWT/API Key validation
- `/crates/spikard-http/src/cors.rs` - CORS policy enforcement
- `/crates/spikard-http/src/response.rs` - Response serialization

## Performance Characteristics

- **Zero-copy** for raw_body (bytes passed directly to bindings)
- **Arc-based** parameter storage for cheap cloning
- **Lazy parsing** - body parsed only when needed
- **Tower middleware** composes at compile-time with zero overhead
- **Connection pooling** via tokio runtime

## Integration Points

- **Language Bindings**: Python (PyO3), Node.js (napi-rs), Ruby (magnus), PHP (ext-php-rs), WebAssembly
- **Handler Trait**: All bindings implement `trait Handler`
- **Spikard-Core**: Depends on core types (Route, Router, Method, SchemaValidator)

## Testing

Comprehensive tests in `/crates/spikard-http/tests/`:

- `server_middleware_behavior.rs` - Middleware ordering and composition
- `server_auth_middleware_behavior.rs` - Authentication flows
- `middleware_stack_integration.rs` - Full stack integration
- `server_cors_preflight.rs` - CORS preflight handling
- `websocket_integration.rs` - WebSocket upgrade and handling
- `sse_behavior.rs` - Server-Sent Events
- Fixture-driven test data in `/testing_data/http_methods/`

## Dependencies

```toml
axum = { features = ["multipart", "ws"] }
tower = "0.5"
tower-http = "*"
tower_governor = "*"
tokio = "*"
tokio-util = "0.7"
jsonwebtoken = "*"
serde_json = "*"
```

## Configuration Files

See `/crates/spikard-http/tests/common/test_builders.rs` for ServerConfig builder patterns and example configurations.
