---
id: fixture_python_smoke_wdl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "version 1.0\n"
    config = {"language": "wdl"}
    _ = process(source, config)

main()

```
