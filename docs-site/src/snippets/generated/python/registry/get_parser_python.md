---
id: fixture_python_get_parser_python
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_parser

def main() -> None:
    name = "python"
    _ = get_parser(name)

main()

```
