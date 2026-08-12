---
id: fixture_python_smoke_styled
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "color: red;\n"
    config = {"language": "styled"}
    _ = process(source, config)

main()

```
