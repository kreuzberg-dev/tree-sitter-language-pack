---
priority: high
---

# Lifecycle Hooks Implementation

Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError) must follow
the zero-cost design in `docs/adr/0005-lifecycle-hooks.md`: use Option<Arc<dyn Fn>> for
conditional execution (<1ns when not registered), provide async support via
pyo3_async_runtimes for Python and ThreadsafeFunction for TypeScript, and allow hooks to
short-circuit with early responses. Implement HookResult enum with Continue/ShortCircuit
variants.
