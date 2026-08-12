---
id: fixture_python_highlights_query_rust
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_highlights_query

def main() -> None:
    language = "rust"
    _ = get_highlights_query(language)

main()

```
