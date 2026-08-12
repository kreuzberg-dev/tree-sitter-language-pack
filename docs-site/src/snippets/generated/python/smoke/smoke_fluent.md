---
id: fixture_python_smoke_fluent
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "hello = Hello\n"
    config = {"language": "fluent"}
    _ = process(source, config)

main()

```
