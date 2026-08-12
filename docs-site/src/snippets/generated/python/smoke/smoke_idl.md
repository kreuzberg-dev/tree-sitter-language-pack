---
id: fixture_python_smoke_idl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "module M {\n};\n"
    config = {"language": "idl"}
    _ = process(source, config)

main()

```
