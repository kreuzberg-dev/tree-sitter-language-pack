---
id: fixture_python_highlights_query_unknown_language
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_highlights_query

def main() -> None:
    language = "nonexistent_language_xyz"
    _ = get_highlights_query(language)

main()

```
