---
id: fixture_python_registry_list_languages
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import available_languages

def main() -> None:
    _ = available_languages()

main()

```
