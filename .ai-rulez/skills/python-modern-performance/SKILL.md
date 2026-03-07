---
priority: critical
description: "Python Modern & Performance Standards"
---

# Python Modern & Performance Standards

**Python 3.10+ · Functional-first · msgspec · Fully async · Strict typing**

- Python 3.10+; match/case, union types (X | Y), structural pattern matching
- msgspec ONLY (NEVER pydantic); msgspec.Struct with slots=True, frozen=True
- Full type hints: ParamSpec, TypeVar/Generic[T], mypy --strict; never use Any
- Functional patterns: pure functions, composition, immutability
- Fully async: anyio.Path, httpx AsyncClient, asyncpg, asyncio.gather
- Function-based tests ONLY (*_test.py); pytest fixtures, 95% coverage
- Never: class tests, pydantic, sync I/O in async, Any type, Optional[T]
