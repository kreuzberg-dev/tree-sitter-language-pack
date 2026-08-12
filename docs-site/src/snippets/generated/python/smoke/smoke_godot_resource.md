---
id: fixture_python_smoke_godot_resource
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
    config = {"language": "godot_resource"}
    _ = process(source, config)

main()

```
