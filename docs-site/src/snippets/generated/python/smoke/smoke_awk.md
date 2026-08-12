---
id: fixture_python_smoke_awk
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'BEGIN { print "hello" }'
    config = {"language": "awk"}
    _ = process(source, config)

main()

```
