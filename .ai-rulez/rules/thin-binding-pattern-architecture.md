---
priority: high
---

# Thin Binding Pattern Architecture

All language bindings (Python/PyO3, Node/napi-rs, Ruby/Magnus, PHP/ext-php-rs)
must follow the "thin binding" pattern: expose only language-idiomatic APIs over the Rust core.
NEVER duplicate business logic, validation, middleware, or routing across bindings. All heavy
lifting lives in `crates/spikard` and `crates/spikard-http`; bindings translate to/from language
types and forward to Rust via the Handler trait. Example: Python's old approach of re-implementing
validation was removed; now all validation happens once in Rust, bindings only convert errors.
This ensures consistency across platforms, reduces maintenance burden, and prevents security gaps
from per-language divergence. Document binding APIs (not implementations) in examples/ and maintain
parity tests that verify identical behavior across all languages using `testing_data/` fixtures.
