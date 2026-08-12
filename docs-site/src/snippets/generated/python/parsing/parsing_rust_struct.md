---
id: fixture_python_parsing_rust_struct
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "struct Point { x: f64, y: f64 }"
    config = {"language": "rust"}
    _ = process(source, config)

main()

```
