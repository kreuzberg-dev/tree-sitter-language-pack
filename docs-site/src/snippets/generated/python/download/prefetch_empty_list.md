---
id: fixture_python_prefetch_empty_list
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import prefetch

def main() -> None:
    languages = []
    _ = prefetch(languages)

main()

```
