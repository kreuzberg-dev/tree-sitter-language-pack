---
id: fixture_python_indents_query_unknown_language
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_indents_query

def main() -> None:
    language = "nonexistent_xyz"
    _ = get_indents_query(language)

main()

```
