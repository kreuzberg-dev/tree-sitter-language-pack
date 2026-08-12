---
id: fixture_python_python_class_with_methods_process
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def subtract(self, a, b):\n        return a - b\n"
    config = {"language": "python"}
    _ = process(source, config)

main()

```
