---
id: fixture_python_rust_chunking_process
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n"
    config = {"chunk_max_size": 30, "language": "rust"}
    _ = process(source, config)

main()

```
