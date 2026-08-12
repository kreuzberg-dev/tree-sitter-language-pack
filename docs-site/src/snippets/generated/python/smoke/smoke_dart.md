---
id: fixture_python_smoke_dart
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
    config = {"language": "dart"}
    _ = process(source, config)

main()

```
