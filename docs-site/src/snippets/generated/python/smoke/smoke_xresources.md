---
id: fixture_python_smoke_xresources
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "*.foreground: #ffffff\n"
    config = {"language": "xresources"}
    _ = process(source, config)

main()

```
