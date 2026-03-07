---
priority: high
description: "HTTP Routing & Middleware Design"
---

# HTTP Routing & Middleware Design

**Tower-HTTP middleware stack · OpenAPI codegen · Lifecycle hooks**

Middleware Stack: Compression → RateLimit → Timeout → RequestId → Auth → UserAgent → Handler. All configurable via CompressionConfig, RateLimitConfig. Auth validates against testing_data/headers/*.json.

Handler Trait: Language-agnostic `trait Handler { fn handle(&self, req: Request) -> Pin<Box<dyn Future<...>>> }`. Binding wrappers implement Arc<dyn Handler>. HTTP server accepts `Vec<(Route, Arc<dyn Handler>)>`.

Lifecycle Hooks (docs/adr/0005-lifecycle-hooks.md): onRequest, preValidation, preHandler, onResponse, onError. Zero-cost: Option<Arc<dyn Fn>>. Async via pyo3_async_runtimes (Python) and ThreadsafeFunction (TypeScript).
