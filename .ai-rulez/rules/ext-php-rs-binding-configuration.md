---
priority: critical
---

# ext-php-rs Binding Configuration

PHP bindings via ext-php-rs in `crates/spikard-php/` must maintain type safety across the
Rust-PHP FFI boundary. Configure ext-php-rs properly in Cargo.toml and ensure all Rust
errors convert to PHP exceptions with structured JSON payloads (error, code, details).
Never expose raw Rust panics; all fallible paths must return ext-php-rs Result types that
translate to thrown PHP exceptions. Maintain PSR-4 autoloading in `packages/php/src/`
and ensure PHPStan level max static analysis passes without @phpstan-ignore directives.
