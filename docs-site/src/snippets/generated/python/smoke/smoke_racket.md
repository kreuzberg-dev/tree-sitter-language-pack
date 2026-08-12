---
id: fixture_python_smoke_racket
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "#lang racket\n(define x 1)"
    config = {"language": "racket"}
    _ = process(source, config)

main()

```
