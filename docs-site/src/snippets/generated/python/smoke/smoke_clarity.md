---
id: fixture_python_smoke_clarity
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "(define-public (hello) (ok true))"
    config = {"language": "clarity"}
    _ = process(source, config)

main()

```
