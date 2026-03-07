---
priority: critical
description: "TypeScript Strictest Standards"
---

# TypeScript Strictest Standards

**TypeScript 5.x · Strictest typing · No any/object · Generics required**

- Enable ALL strict flags: strict, noUncheckedIndexedAccess, exactOptionalPropertyTypes
- Ban any and object types; use unknown with guards
- Generics with constraints, satisfies operator, const assertions
- Tests: .spec.ts next to source files; vitest, 80%+ coverage
- Functional: pure functions, map/filter/reduce, immutability, readonly
- Biome for linting/formatting; pnpm ≥10.17, pnpm-lock.yaml committed
- React: function components, custom hooks, proper prop typing
- Never: any/object types, non-null assertions !, || for defaults
