---
id: fixture_python_smoke_bsl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Procedure Main() EndProcedure"
    config = {"language": "bsl"}
    _ = process(source, config)

main()

```
