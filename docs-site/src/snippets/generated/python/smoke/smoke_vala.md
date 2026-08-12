---
id: fixture_python_smoke_vala
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Foo {\n}\n"
    config = {"language": "vala"}
    _ = process(source, config)

main()

```
