---
id: fixture_python_smoke_cpon
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '{"key": 1}'
    config = {"language": "cpon"}
    _ = process(source, config)

main()

```
