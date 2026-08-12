---
id: fixture_python_smoke_edoc
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@doc foo\n"
    config = {"language": "edoc"}
    _ = process(source, config)

main()

```
