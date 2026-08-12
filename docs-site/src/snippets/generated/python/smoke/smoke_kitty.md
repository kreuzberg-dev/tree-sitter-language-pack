---
id: fixture_python_smoke_kitty
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "font_size 12\n"
    config = {"language": "kitty"}
    _ = process(source, config)

main()

```
