---
priority: high
---

# Tower-HTTP Middleware Stack

All standard middleware (compression, rate limiting, timeouts, graceful shutdown, static
files, request IDs) is implemented in Rust using tower-http and exposed via typed
ServerConfig. Configuration structs (CompressionConfig, RateLimitConfig, StaticFilesConfig,
etc.) must be forwarded to Python/TypeScript/Ruby bindings with proper type safety.
See `docs/adr/0002-runtime-and-middleware.md` for the complete middleware stack order
and configuration options.
