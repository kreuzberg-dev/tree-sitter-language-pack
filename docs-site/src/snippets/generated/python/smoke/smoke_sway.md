---
id: fixture_python_smoke_sway
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "x"
    config = {"language": "sway"}
    _ = process(source, config)

main()

```
