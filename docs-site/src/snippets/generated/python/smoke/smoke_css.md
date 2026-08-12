---
id: fixture_python_smoke_css
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "body { color: red; }"
    config = {"language": "css"}
    _ = process(source, config)

main()

```
