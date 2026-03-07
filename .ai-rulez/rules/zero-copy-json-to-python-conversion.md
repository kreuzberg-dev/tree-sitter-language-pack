---
priority: high
---

# Zero-Copy JSON to Python Conversion

Convert `serde_json::Value` to Python objects using direct PyO3 type construction
(PyDict::new, PyList::empty, PyString::new, etc.) instead of serialize-to-JSON-string
then json.loads. This zero-copy approach in `crates/spikard-py/src/handler.rs::json_to_python()`
eliminates 30-40% conversion overhead. Match on Value variants and recursively build
native Python objects.
