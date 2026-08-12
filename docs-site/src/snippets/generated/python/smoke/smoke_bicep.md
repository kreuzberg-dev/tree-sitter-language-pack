---
id: fixture_python_smoke_bicep
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "param name string"
    config = {"language": "bicep"}
    _ = process(source, config)

main()

```
