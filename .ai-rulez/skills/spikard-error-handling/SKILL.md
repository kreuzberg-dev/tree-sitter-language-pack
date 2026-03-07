---
priority: critical
description: "Cross-Language Error Handling"
---

# Cross-Language Error Handling

**Structured error payloads · FFI boundaries · Validation schema alignment**

Error Structure: All errors return JSON { "error": string, "code": string, "details": {} }. Rust uses Result<T, E> with thiserror. Python: PyResult<T> → PyErr. Node: napi::Result<T> → napi::Error. Ruby: raise_error. PHP: ext-php-rs Result → exceptions. All preserve same JSON payload.

FFI Boundaries: Rust core returns Result<T, E> to handlers. Adapters (PyO3/napi/magnus/ext-php-rs) convert to language errors while preserving JSON. Python: PyErr::new_err(json_string). Node: Error::from_reason(json_string). PHP: throw exceptions. Never let unwrap cross FFI boundary.

Validation: HTTP handlers validate against testing_data/{headers,cookies,json_bodies}. Reject with errors matching testing_data/validation_errors/schema.json. Assert in packages/python/tests/test_all_fixtures.py. Keep schema.json in sync with handler code.
