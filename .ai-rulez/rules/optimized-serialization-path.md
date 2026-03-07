---
priority: medium
---

# Optimized Serialization Path

Follow the conversion patterns captured in `docs/adr/0003-validation-and-fixtures.md`
so data exchanged between Rust and Python leverages `msgspec` without extra JSON hops.
Share zero-copy buffers where possible, and use `task build:rust` (release mode) when
benchmarking or publishing bindings to avoid debug-performance regressions.
