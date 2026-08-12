---
id: fixture_python_smoke_textproto
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'key: "value"'
    config = {"language": "textproto"}
    _ = process(source, config)

main()

```
