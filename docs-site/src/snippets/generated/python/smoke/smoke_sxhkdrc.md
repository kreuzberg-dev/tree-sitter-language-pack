---
id: fixture_python_smoke_sxhkdrc
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "super + a\n\techo hi\n"
    config = {"language": "sxhkdrc"}
    _ = process(source, config)

main()

```
