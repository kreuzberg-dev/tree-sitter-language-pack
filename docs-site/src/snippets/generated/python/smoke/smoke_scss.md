---
id: fixture_python_smoke_scss
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "$color: red;\nbody { color: $color; }"
    config = {"language": "scss"}
    _ = process(source, config)

main()

```
