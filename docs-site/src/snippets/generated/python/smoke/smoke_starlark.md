---
id: fixture_python_smoke_starlark
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def hello(): pass"
    config = {"language": "starlark"}
    _ = process(source, config)

main()

```
