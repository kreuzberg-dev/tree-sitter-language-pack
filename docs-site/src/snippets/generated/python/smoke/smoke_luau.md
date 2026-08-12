---
id: fixture_python_smoke_luau
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "local x: number = 1"
    config = {"language": "luau"}
    _ = process(source, config)

main()

```
