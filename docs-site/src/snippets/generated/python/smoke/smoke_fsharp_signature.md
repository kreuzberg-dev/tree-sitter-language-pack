---
id: fixture_python_smoke_fsharp_signature
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "val x: int"
    config = {"language": "fsharp_signature"}
    _ = process(source, config)

main()

```
