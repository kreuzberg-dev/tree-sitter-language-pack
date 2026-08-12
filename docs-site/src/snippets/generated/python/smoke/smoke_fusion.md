---
id: fixture_python_smoke_fusion
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "foo = 1\n"
    config = {"language": "fusion"}
    _ = process(source, config)

main()

```
