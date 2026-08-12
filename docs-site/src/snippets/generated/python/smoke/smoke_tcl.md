---
id: fixture_python_smoke_tcl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "puts hello"
    config = {"language": "tcl"}
    _ = process(source, config)

main()

```
