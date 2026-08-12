---
id: fixture_python_registry_has_language_false
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import has_language

def main() -> None:
    name = "nonexistent"
    _ = has_language(name)

main()

```
