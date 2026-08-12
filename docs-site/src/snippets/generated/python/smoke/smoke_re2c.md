---
id: fixture_python_smoke_re2c
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "/*!re2c\n  [a-z]+ { return; }\n*/"
    config = {"language": "re2c"}
    _ = process(source, config)

main()

```
