---
id: fixture_python_smoke_squirrel
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "function main() {}"
    config = {"language": "squirrel"}
    _ = process(source, config)

main()

```
