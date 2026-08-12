---
id: fixture_python_smoke_slim
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "p hello\n"
    config = {"language": "slim"}
    _ = process(source, config)

main()

```
