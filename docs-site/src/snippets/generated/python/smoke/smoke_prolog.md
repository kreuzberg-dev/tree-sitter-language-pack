---
id: fixture_python_smoke_prolog
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "hello :- write('hello'), nl."
    config = {"language": "prolog"}
    _ = process(source, config)

main()

```
