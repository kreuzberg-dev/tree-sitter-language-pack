---
id: fixture_python_error_handling_empty_source
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = ""
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
