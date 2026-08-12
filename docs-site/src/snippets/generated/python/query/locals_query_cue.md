---
id: fixture_python_locals_query_cue
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_locals_query

def main() -> None:
    language = "cue"
    _ = get_locals_query(language)

main()

```
