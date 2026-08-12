---
id: fixture_python_smoke_ada
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "procedure Main is begin null; end Main;"
    config = {"language": "ada"}
    _ = process(source, config)

main()

```
