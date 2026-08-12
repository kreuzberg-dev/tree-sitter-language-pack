---
id: fixture_python_smoke_gosum
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "example.com/pkg v1.0.0 h1:abc="
    config = {"language": "gosum"}
    _ = process(source, config)

main()

```
