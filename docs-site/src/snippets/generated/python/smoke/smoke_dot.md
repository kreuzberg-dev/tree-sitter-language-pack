---
id: fixture_python_smoke_dot
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "digraph G { A -> B; }"
    config = {"language": "dot"}
    _ = process(source, config)

main()

```
