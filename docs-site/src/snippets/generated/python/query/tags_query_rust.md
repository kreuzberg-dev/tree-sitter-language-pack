---
id: fixture_python_tags_query_rust
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_tags_query

def main() -> None:
    language = "rust"
    _ = get_tags_query(language)

main()

```
