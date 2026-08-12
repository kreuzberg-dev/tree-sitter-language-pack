---
id: fixture_python_parsing_python_function
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def hello(): pass"
    config = {"language": "python"}
    _ = process(source, config)

main()

```
