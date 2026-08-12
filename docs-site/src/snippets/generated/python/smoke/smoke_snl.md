---
id: fixture_python_smoke_snl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "program test\n"
    config = {"language": "snl"}
    _ = process(source, config)

main()

```
