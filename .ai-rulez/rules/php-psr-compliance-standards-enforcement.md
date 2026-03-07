---
priority: critical
---

# PHP PSR Compliance & Standards Enforcement

All PHP code in `packages/php/src/` must enforce PSR-4 (autoloading), PSR-12 (coding style),
and PSR-7 (HTTP interfaces). Use PHPStan level max (9) for static analysis; never suppress
errors with @phpstan-ignore directives. Declare strict_types=1 at the top of every file.
Use typed properties and return types on all methods; avoid mixed types. PHPUnit testing
must cover 80%+ of code; use data providers for parametrized tests. Composer dependencies
are committed (composer.lock) and managed strictly. Never use eval(), suppress errors with
@, or maintain global state. Methods must be <15 lines; use composition over inheritance.
