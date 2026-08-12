---
id: fixture_python_smoke_comment
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Review: handle edge case"
    config = {"language": "comment"}
    _ = process(source, config)

main()

```
