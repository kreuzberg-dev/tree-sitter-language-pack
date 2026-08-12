---
id: fixture_python_injections_query_rust
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_injections_query

def main() -> None:
    language = "rust"
    _ = get_injections_query(language)

main()

```
