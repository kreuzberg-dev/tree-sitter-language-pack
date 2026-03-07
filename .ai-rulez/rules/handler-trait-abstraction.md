---
priority: critical
---

# Handler Trait Abstraction

In `crates/spikard-http`, define language-agnostic Handler trait with
`Pin<Box<dyn Future<Output = HandlerResult> + Send>>` return type. Language bindings
(`spikard-py`, `spikard-node`, `spikard-rb`, `spikard-php`) implement this trait with
Arc<dyn Handler> wrappers. The HTTP server accepts `Vec<(Route, Arc<dyn Handler>)>` enabling
clean separation: spikard-http has zero FFI dependencies, all Python/Node/Ruby/PHP/WASM code
lives in binding crates.
