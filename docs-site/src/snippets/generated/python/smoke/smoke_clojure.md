---
id: fixture_python_smoke_clojure
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "(def x 1)"
    config = {"language": "clojure"}
    _ = process(source, config)

main()

```
