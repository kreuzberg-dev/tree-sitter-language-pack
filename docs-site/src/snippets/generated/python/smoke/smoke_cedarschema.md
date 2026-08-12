---
id: fixture_python_smoke_cedarschema
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "entity User;"
    config = {"language": "cedarschema"}
    _ = process(source, config)

main()

```
