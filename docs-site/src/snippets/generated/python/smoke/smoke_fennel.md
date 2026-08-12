---
id: fixture_python_smoke_fennel
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "(fn hello [] (print :hello))"
    config = {"language": "fennel"}
    _ = process(source, config)

main()

```
