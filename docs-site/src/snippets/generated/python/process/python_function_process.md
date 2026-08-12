---
id: fixture_python_python_function_process
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def greet(name):\n    return f'Hello, {name}!'\n"
    config = {"language": "python"}
    _ = process(source, config)

main()

```
