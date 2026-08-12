---
id: fixture_python_smoke_cedar
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "permit(principal, action, resource);"
    config = {"language": "cedar"}
    _ = process(source, config)

main()

```
