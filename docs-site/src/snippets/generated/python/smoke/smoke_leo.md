---
id: fixture_python_smoke_leo
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "program test.aleo {\n}\n"
    config = {"language": "leo"}
    _ = process(source, config)

main()

```
