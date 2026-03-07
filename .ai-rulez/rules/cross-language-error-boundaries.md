---
priority: critical
---

# Cross-Language Error Boundaries

Rust code in `crates/spikard`, `crates/spikard-http`, and the binding crates must
avoid panics; expose fallible APIs as `Result<T, E>` and propagate with `?`. When
exporting to Python (`crates/spikard-py/src`), always return `PyResult<T>` and convert
domain failures with `PyErr::new_err(...)`; for Node (`crates/spikard-node/src`),
return `napi::Result<T>` and build errors via `napi::Error::from_reason`; for PHP
(`crates/spikard-php/src`), return ext-php-rs Result and throw exceptions. Never let
an unwrap cross the FFI boundary.
