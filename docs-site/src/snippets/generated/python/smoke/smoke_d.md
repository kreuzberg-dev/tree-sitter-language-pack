---
id: fixture_python_smoke_d
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "void main() {}"
    config = {"language": "d"}
    _ = process(source, config)

main()

```
