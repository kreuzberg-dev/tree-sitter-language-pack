---
id: fixture_python_registry_get_language_alias
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_language

def main() -> None:
    name = "shell"
    _ = get_language(name)

main()

```
