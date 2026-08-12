---
id: fixture_python_smoke_kotlin
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fun main() {}"
    config = {"language": "kotlin"}
    _ = process(source, config)

main()

```
