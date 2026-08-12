---
id: fixture_python_smoke_gren
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "module Main exposing (..)"
    config = {"language": "gren"}
    _ = process(source, config)

main()

```
