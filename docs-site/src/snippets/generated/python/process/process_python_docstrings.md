---
id: fixture_python_process_python_docstrings
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'def greet(name):\n    """Say hello to someone."""\n    return f"Hello {name}"\n'
    config = {"docstrings": True, "language": "python"}
    _ = process(source, config)

main()

```
