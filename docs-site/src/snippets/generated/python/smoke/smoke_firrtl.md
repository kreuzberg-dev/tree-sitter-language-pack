---
id: fixture_python_smoke_firrtl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "circuit Main :"
    config = {"language": "firrtl"}
    _ = process(source, config)

main()

```
