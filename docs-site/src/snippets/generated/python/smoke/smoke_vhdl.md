---
id: fixture_python_smoke_vhdl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "entity main is end main;"
    config = {"language": "vhdl"}
    _ = process(source, config)

main()

```
