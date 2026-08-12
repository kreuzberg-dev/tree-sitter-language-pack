---
id: fixture_python_tags_query_kotlin
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_tags_query

def main() -> None:
    language = "kotlin"
    _ = get_tags_query(language)

main()

```
