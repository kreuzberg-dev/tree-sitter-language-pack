---
priority: critical
---

# PyO3 Async Performance

For async Python handlers in `crates/spikard-py/src/handler.rs`, use
`pyo3_async_runtimes::tokio::into_future()` to convert Python coroutines directly to
Rust futures, eliminating spawn_blocking overhead. Initialize the event loop once with
`TaskLocals` stored in a `OnceCell` to avoid per-request event loop creation. Ensure
GIL is released before awaiting Rust futures: `Python::attach(|py| {...}).await`
not `Python::with_gil(|py| {...}).await`.
