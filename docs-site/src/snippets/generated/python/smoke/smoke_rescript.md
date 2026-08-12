---
id: fixture_python_smoke_rescript
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "let x = 1"
    config = {"language": "rescript"}
    _ = process(source, config)

main()

```
