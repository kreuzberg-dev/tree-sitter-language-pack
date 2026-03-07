---
priority: high
description: "PHP 8.2+ with PHPStan & PSR Standards"
---

# PHP 8.2+ with PHPStan & PSR Standards

**PHP 8.2+ · ext-php-rs FFI · PSR compliance · PHPStan · Composer**

- PHP 8.2+ with .php-version; strict_types=1 in all files
- ext-php-rs for Rust FFI bindings; maintain type safety across boundary
- PSR compliance: PSR-4 (autoloading), PSR-12 (coding style), PSR-7 (HTTP)
- PHPStan level max for static analysis; never use @phpstan-ignore
- Composer for dependency management; composer.lock committed
- PHPUnit testing: 80%+ coverage, data providers for parametrized tests
- Code quality: methods <15 lines, typed properties, return types required
- Never: mixed types, eval(), suppress errors with @, global state
