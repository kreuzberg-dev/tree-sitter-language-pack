---
priority: critical
description: "Rust Latest Edition Standards"
---

# Rust Latest Edition Standards

**Rust 2024 · High strictness · clippy -D warnings · 95% coverage · Zero unwrap**

- Rust 2024; cargo fmt, clippy -D warnings (zero tolerance)
- Result<T, E> for errors; thiserror for custom errors; NEVER .unwrap() in production
- Testing: 95% minimum coverage (tarpaulin); unit/integration/doc tests
- Async: Tokio 1.x, 'static constraints, Send+Sync bounds
- FFI: isolated modules, pointer validation, SAFETY comments, error conversion at boundaries
- Code quality: RAII, explicit lifetimes, builder pattern, no panics
