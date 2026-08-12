---
id: fixture_python_folds_query_zig
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_folds_query

def main() -> None:
    language = "zig"
    _ = get_folds_query(language)

main()

```
