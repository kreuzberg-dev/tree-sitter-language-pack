---
id: fixture_python_smoke_dockerfile
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "FROM alpine"
    config = {"language": "dockerfile"}
    _ = process(source, config)

main()

```
