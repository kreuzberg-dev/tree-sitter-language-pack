---
id: fixture_python_smoke_ninja
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "rule cc\n  command = cc $in -o $out"
    config = {"language": "ninja"}
    _ = process(source, config)

main()

```
